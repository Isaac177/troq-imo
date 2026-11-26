use base64::Engine;
use chrono::{DateTime, Utc};
use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::error::{codes, Error, Result};
use kernel::files::clean_file_name;
use kernel::ids::new_id;
use kernel::storage::{key_for, validate_key, Scope, Storage, Visibility};
use serde_json::json;
use sqlx::PgPool;

use crate::hosts::messages;
use super::types::{document_kind, AddDocumentInput, HostDocumentView};

const MAX_BYTES: usize = 10 * 1024 * 1024;
const MIME_TYPES: &[(&str, &str)] = &[
    ("application/pdf", "pdf"),
    ("image/jpeg", "jpg"),
    ("image/png", "png"),
    ("image/webp", "webp"),
];

type DocumentRow = (String, String, String, String, i32, String, Option<String>, DateTime<Utc>);

fn view(row: DocumentRow) -> HostDocumentView {
    let (id, kind, file_name, mime_type, size_bytes, status, reason_code, created_at) = row;
    HostDocumentView { id, kind, file_name, mime_type, size_bytes, status, reason_code, created_at }
}

pub async fn list_for(pool: &PgPool, application_id: &str) -> Result<Vec<HostDocumentView>> {
    let rows = sqlx::query_as::<_, DocumentRow>(
        "SELECT id, kind, file_name, mime_type, size_bytes, status, reason_code, created_at FROM host_application_documents WHERE application_id = $1 ORDER BY created_at, id",
    )
    .bind(application_id)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(view).collect())
}

fn extension(mime_type: &str) -> Option<&'static str> {
    MIME_TYPES
        .iter()
        .find(|(mime, _)| mime.eq_ignore_ascii_case(mime_type.trim()))
        .map(|(_, extension)| *extension)
}

async fn draft_application(pool: &PgPool, actor: &Actor, id: &str) -> Result<()> {
    let status: Option<String> = sqlx::query_scalar(
        "SELECT status FROM host_applications WHERE id = $1 AND applicant_user_id = $2",
    )
    .bind(id)
    .bind(actor.user_id().as_str())
    .fetch_optional(pool)
    .await?;
    match status.as_deref() {
        Some("DRAFT") => Ok(()),
        Some(status) => Err(Error::conflict(messages::APPLICATION_STATE).with_params(json!({ "status": status }))),
        None => Err(Error::not_found(codes::NOT_FOUND)),
    }
}

pub async fn add(
    pool: &PgPool,
    storage: &Storage,
    actor: &Actor,
    application_id: &str,
    input: AddDocumentInput,
) -> Result<HostDocumentView> {
    actor.ensure_writable()?;
    draft_application(pool, actor, application_id).await?;
    let kind = document_kind(&input.kind).ok_or_else(|| Error::bad_request(messages::DOCUMENT_KIND_INVALID))?;
    let mime_type = input.mime_type.trim().to_ascii_lowercase();
    let extension = extension(&mime_type).ok_or_else(|| Error::bad_request(messages::DOCUMENT_KIND_INVALID))?;
    let file_name = clean_file_name(&input.file_name)?;
    if file_name.chars().count() > 255 {
        return Err(Error::bad_request(codes::FILE_NAME_INVALID));
    }
    let content = base64::engine::general_purpose::STANDARD
        .decode(input.content_base64.trim())
        .map_err(|_| Error::bad_request(messages::DOCUMENT_KEY_INVALID))?;
    if content.is_empty() || content.len() > MAX_BYTES {
        return Err(Error::bad_request(messages::DOCUMENT_KEY_INVALID).with_params(json!({ "maxBytes": MAX_BYTES })));
    }
    let id = new_id("hostdoc");
    let key = key_for(
        &Scope::for_city(actor.city),
        &format!("hosts/applications/{application_id}"),
        &format!("{id}.{extension}"),
    )
    .map_err(|_| Error::bad_request(messages::DOCUMENT_KEY_INVALID))?;
    validate_key(&key).map_err(|_| Error::bad_request(messages::DOCUMENT_KEY_INVALID))?;
    storage
        .put(Visibility::Private, &key, content.clone(), &mime_type)
        .await
        .map_err(|error| Error::internal(error.to_string()))?;
    let size_bytes = i32::try_from(content.len()).map_err(|_| Error::bad_request(messages::DOCUMENT_KEY_INVALID))?;
    let mut tx = pool.begin().await?;
    let created: DateTime<Utc> = sqlx::query_scalar(
        "INSERT INTO host_application_documents (id, application_id, kind, storage_key, file_name, mime_type, size_bytes) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING created_at",
    )
    .bind(&id)
    .bind(application_id)
    .bind(kind)
    .bind(&key)
    .bind(&file_name)
    .bind(&mime_type)
    .bind(size_bytes)
    .fetch_one(&mut *tx)
    .await?;
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "host.application.document_added",
        resource_type: "host_application_document",
        resource_id: &id,
        reason: None,
        change: json!({ "applicationId": application_id, "kind": kind }),
    })
    .await?;
    tx.commit().await?;
    Ok(HostDocumentView {
        id,
        kind: kind.to_string(),
        file_name,
        mime_type,
        size_bytes,
        status: "PENDING".into(),
        reason_code: None,
        created_at: created,
    })
}

pub async fn remove(pool: &PgPool, storage: &Storage, actor: &Actor, application_id: &str, document_id: &str) -> Result<()> {
    actor.ensure_writable()?;
    draft_application(pool, actor, application_id).await?;
    let mut tx = pool.begin().await?;
    let key: Option<String> = sqlx::query_scalar(
        "DELETE FROM host_application_documents WHERE id = $1 AND application_id = $2 AND status = 'PENDING' RETURNING storage_key",
    )
    .bind(document_id)
    .bind(application_id)
    .fetch_optional(&mut *tx)
    .await?;
    let key = key.ok_or_else(|| Error::not_found(codes::NOT_FOUND))?;
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "host.application.document_removed",
        resource_type: "host_application_document",
        resource_id: document_id,
        reason: None,
        change: json!({ "applicationId": application_id }),
    })
    .await?;
    tx.commit().await?;
    let _ = storage.delete(Visibility::Private, &key).await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_supported_document_mime_types_are_accepted() {
        assert_eq!(extension("application/pdf"), Some("pdf"));
        assert_eq!(extension("IMAGE/PNG"), Some("png"));
        assert!(extension("application/zip").is_none());
    }
}
