use base64::Engine;
use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::city::City;
use kernel::error::{Error, Result};
use kernel::files::clean_file_name;
use kernel::ids::new_id;
use kernel::storage::Storage;
use serde_json::json;
use sqlx::PgPool;

use super::kind::MediaKind;
use super::queries::{self, NewMedia};
use super::storage::{delete_image, image_url, upload_image};
use super::types::{
    AddMediaInput, MediaChanges, PropertyMediaContent, PropertyMediaRemoved, PropertyMediaView, RemoveMediaInput, UpdateMediaInput,
};
use super::validation::{check_image, clean_alt_text};
use crate::properties::messages::{
    MEDIA_ALT_TEXT_TOO_LONG, MEDIA_CLIENT_KEY_INVALID, MEDIA_CONTENT_INVALID, MEDIA_COVER_CANNOT_BE_CLEARED,
    MEDIA_COVER_NOT_READY, MEDIA_MIME_TYPE_INVALID, MEDIA_NOT_FOUND, MEDIA_VERSION_CONFLICT,
};

const MAX_MIME_TYPE_LENGTH: usize = 32;
const MAX_CLIENT_KEY_LENGTH: usize = 100;

#[derive(Debug, Clone, Copy)]
pub struct Owner<'a> {
    pub kind: MediaKind,
    pub id: &'a str,
    pub city: City,
}

fn alt_text(value: &str) -> Result<String> {
    clean_alt_text(value).map_err(|_| Error::bad_request(MEDIA_ALT_TEXT_TOO_LONG))
}

fn client_key(value: &str) -> Result<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.len() > MAX_CLIENT_KEY_LENGTH {
        return Err(Error::bad_request(MEDIA_CLIENT_KEY_INVALID).with_params(json!({ "max": MAX_CLIENT_KEY_LENGTH })));
    }
    Ok(trimmed.to_string())
}

pub async fn list(pool: &PgPool, owner: Owner<'_>) -> Result<Vec<PropertyMediaView>> {
    queries::list(pool, owner.kind, owner.id).await
}

pub async fn find(pool: &PgPool, kind: MediaKind, media_id: &str) -> Result<PropertyMediaView> {
    queries::find(pool, kind, media_id).await?.ok_or_else(|| Error::not_found(MEDIA_NOT_FOUND))
}

pub async fn add(
    pool: &PgPool,
    storage: &Storage,
    actor: &Actor,
    owner: Owner<'_>,
    input: AddMediaInput,
) -> Result<PropertyMediaView> {
    actor.ensure_writable()?;
    let key = client_key(&input.client_key)?;
    if let Some(existing) = queries::find_by_client_key(pool, owner.kind, owner.id, &key).await? {
        return Ok(existing);
    }
    let file_name = clean_file_name(&input.file_name)?;
    let alt_en = alt_text(&input.alt_en)?;
    let alt_fr = alt_text(&input.alt_fr)?;
    if input.mime_type.len() > MAX_MIME_TYPE_LENGTH {
        return Err(Error::bad_request(MEDIA_MIME_TYPE_INVALID));
    }
    let content = base64::engine::general_purpose::STANDARD
        .decode(input.content_base64.trim())
        .map_err(|_| Error::bad_request(MEDIA_CONTENT_INVALID))?;
    let checked = check_image(&input.mime_type, &content);
    let media_id = new_id("media");
    let storage_key = match checked {
        Ok(image_type) => {
            Some(upload_image(storage, owner.city, owner.kind.folder(), &media_id, image_type, &content).await?)
        }
        Err(_) => None,
    };
    let rejection = checked.err().map(|reason| reason.code());
    let size = if rejection.is_some() { 0 } else { content.len() };
    let stored = insert_and_audit(pool, actor, owner, &media_id, &key, &file_name, &input, (&alt_en, &alt_fr), storage_key.as_deref(), rejection, size).await;
    if let Err(error) = stored {
        if let Some(object) = &storage_key {
            let _ = delete_image(storage, object).await;
        }
        return Err(error);
    }
    find(pool, owner.kind, &media_id).await
}

#[allow(clippy::too_many_arguments)]
async fn insert_and_audit(
    pool: &PgPool,
    actor: &Actor,
    owner: Owner<'_>,
    media_id: &str,
    client_key: &str,
    file_name: &str,
    input: &AddMediaInput,
    alt: (&str, &str),
    storage_key: Option<&str>,
    rejection: Option<&str>,
    size: usize,
) -> Result<()> {
    let mut tx = pool.begin().await?;
    queries::lock_owner(&mut tx, owner.kind, owner.id).await?;
    let sort_order = queries::next_sort_order(&mut tx, owner.kind, owner.id).await?;
    let is_cover = rejection.is_none() && !queries::has_cover(&mut tx, owner.kind, owner.id).await?;
    let status = if rejection.is_some() { "REJECTED" } else { "READY" };
    queries::insert(&mut tx, owner.kind, NewMedia {
        id: media_id,
        owner_id: owner.id,
        status,
        sort_order,
        is_cover,
        alt_en: alt.0,
        alt_fr: alt.1,
        file_name,
        mime_type: &input.mime_type,
        storage_key,
        size_bytes: size,
        rejection_reason: rejection,
        client_key,
        actor: actor.user_id().as_str(),
    })
    .await?;
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: audit_action(owner.kind, "media_added"),
        resource_type: owner.kind.resource_type(),
        resource_id: owner.id,
        reason: None,
        change: json!({ "mediaId": media_id, "status": status, "sizeBytes": size }),
    })
    .await?;
    tx.commit().await?;
    Ok(())
}

fn audit_action(kind: MediaKind, event: &str) -> &'static str {
    match (kind, event) {
        (MediaKind::Property, "media_added") => "property.media_added",
        (MediaKind::Property, "media_updated") => "property.media_updated",
        (MediaKind::Property, _) => "property.media_removed",
        (MediaKind::Template, "media_added") => "accommodation_template.media_added",
        (MediaKind::Template, "media_updated") => "accommodation_template.media_updated",
        (MediaKind::Template, _) => "accommodation_template.media_removed",
    }
}

pub async fn update(
    pool: &PgPool,
    actor: &Actor,
    owner: Owner<'_>,
    media: &PropertyMediaView,
    input: UpdateMediaInput,
) -> Result<PropertyMediaView> {
    actor.ensure_writable()?;
    if input.is_cover == Some(false) {
        return Err(Error::bad_request(MEDIA_COVER_CANNOT_BE_CLEARED));
    }
    if input.is_cover == Some(true) && media.status != "READY" {
        return Err(Error::conflict(MEDIA_COVER_NOT_READY));
    }
    let changes = MediaChanges {
        alt_en: input.alt_en.as_deref().map(alt_text).transpose()?,
        alt_fr: input.alt_fr.as_deref().map(alt_text).transpose()?,
        is_cover: input.is_cover.filter(|value| *value),
        sort_order: input.sort_order,
        remove: false,
    };
    let mut tx = pool.begin().await?;
    queries::lock_owner(&mut tx, owner.kind, owner.id).await?;
    if changes.is_cover == Some(true) {
        queries::clear_covers_except(&mut tx, owner.kind, owner.id, &media.id).await?;
    }
    if !queries::update_if_version(&mut tx, owner.kind, &media.id, input.expected_version, &changes).await? {
        return Err(Error::conflict(MEDIA_VERSION_CONFLICT));
    }
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: audit_action(owner.kind, "media_updated"),
        resource_type: owner.kind.resource_type(),
        resource_id: owner.id,
        reason: None,
        change: json!({ "mediaId": media.id, "isCover": changes.is_cover, "sortOrder": changes.sort_order }),
    })
    .await?;
    tx.commit().await?;
    find(pool, owner.kind, &media.id).await
}

pub async fn remove(
    pool: &PgPool,
    storage: &Storage,
    actor: &Actor,
    owner: Owner<'_>,
    media: &PropertyMediaView,
    input: RemoveMediaInput,
) -> Result<PropertyMediaRemoved> {
    actor.ensure_writable()?;
    let storage_key = queries::storage_key(pool, owner.kind, &media.id, false).await?;
    let changes = MediaChanges { remove: true, ..MediaChanges::default() };
    let mut tx = pool.begin().await?;
    queries::lock_owner(&mut tx, owner.kind, owner.id).await?;
    if !queries::update_if_version(&mut tx, owner.kind, &media.id, input.expected_version, &changes).await? {
        return Err(Error::conflict(MEDIA_VERSION_CONFLICT));
    }
    if media.is_cover {
        queries::promote_first_ready(&mut tx, owner.kind, owner.id).await?;
    }
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: audit_action(owner.kind, "media_removed"),
        resource_type: owner.kind.resource_type(),
        resource_id: owner.id,
        reason: None,
        change: json!({ "mediaId": media.id }),
    })
    .await?;
    tx.commit().await?;
    if let Some(key) = storage_key {
        if let Err(error) = delete_image(storage, &key).await {
            tracing::warn!(media_id = %media.id, error = %error, "media object deletion failed");
        }
    }
    Ok(PropertyMediaRemoved { media_id: media.id.clone(), status: "REMOVED".into() })
}

pub async fn content(pool: &PgPool, storage: &Storage, kind: MediaKind, media: PropertyMediaView) -> Result<PropertyMediaContent> {
    let key = queries::storage_key(pool, kind, &media.id, true)
        .await?
        .ok_or_else(|| Error::not_found(MEDIA_NOT_FOUND))?;
    let url = image_url(storage, &key).await?;
    Ok(PropertyMediaContent { media, url })
}
