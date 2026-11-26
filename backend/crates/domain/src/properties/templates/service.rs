use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::error::{Error, Result};
use kernel::ids::new_id;
use kernel::paging::{Page, PageRequest};
use kernel::permissions::names::{PROPERTIES_CONFIGURE, PROPERTIES_READ};
use kernel::storage::Storage;
use serde_json::json;
use sqlx::PgPool;

use super::access::{ensure_host_city, ensure_member, template_for};
use super::queries;
use super::types::{TemplateInput, TemplateListQuery, TemplateView};
use super::validation::{status, validate};
use crate::properties::categories::ensure_selectable;
use crate::properties::messages::{TEMPLATE_NOT_FOUND, TEMPLATE_VERSION_CONFLICT};
use crate::properties::template_media::read::covers_for_templates;

async fn attach_covers(pool: &PgPool, storage: &Storage, items: &mut [TemplateView]) -> Result<()> {
    let ids: Vec<String> = items.iter().filter(|item| item.image_count > 0).map(|item| item.id.clone()).collect();
    let mut covers = covers_for_templates(pool, storage, &ids).await?;
    for item in items.iter_mut() {
        item.cover_url = covers.remove(&item.id);
    }
    Ok(())
}

pub async fn find_with_cover(pool: &PgPool, storage: &Storage, id: &str) -> Result<TemplateView> {
    let item = queries::find(pool, id).await?.ok_or_else(|| Error::not_found(TEMPLATE_NOT_FOUND))?;
    let mut items = [item];
    attach_covers(pool, storage, &mut items).await?;
    let [item] = items;
    Ok(item)
}

pub async fn list(
    pool: &PgPool,
    storage: &Storage,
    actor: &Actor,
    query: &TemplateListQuery,
    page: PageRequest,
    as_admin: bool,
) -> Result<Page<TemplateView>> {
    ensure_member(actor, query.host_id.as_deref().unwrap_or(""), PROPERTIES_READ, as_admin)?;
    let mut result = queries::list(pool, actor.city, query, page).await?;
    attach_covers(pool, storage, &mut result.items).await?;
    Ok(result)
}

pub async fn create(
    pool: &PgPool,
    storage: &Storage,
    actor: &Actor,
    input: TemplateInput,
    as_admin: bool,
) -> Result<TemplateView> {
    actor.ensure_writable()?;
    ensure_member(actor, &input.host_id, PROPERTIES_CONFIGURE, as_admin)?;
    let amount = validate(&input)?;
    ensure_selectable(pool, input.category_code.trim(), None).await?;
    let id = new_id("template");
    let mut tx = pool.begin().await?;
    ensure_host_city(&mut tx, &input.host_id, actor.city).await?;
    queries::insert(&mut tx, &id, actor.city, &input, amount).await?;
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "accommodation_template.create",
        resource_type: "accommodation_template",
        resource_id: &id,
        reason: None,
        change: json!({ "hostId": input.host_id }),
    })
    .await?;
    tx.commit().await?;
    find_with_cover(pool, storage, &id).await
}

pub async fn update(
    pool: &PgPool,
    storage: &Storage,
    actor: &Actor,
    template_id: &str,
    input: TemplateInput,
    as_admin: bool,
) -> Result<TemplateView> {
    actor.ensure_writable()?;
    let current = template_for(pool, actor, template_id, PROPERTIES_CONFIGURE, as_admin).await?;
    if current.operator_id != input.host_id.trim() {
        return Err(Error::not_found(TEMPLATE_NOT_FOUND));
    }
    let amount = validate(&input)?;
    ensure_selectable(pool, input.category_code.trim(), Some(current.category_code.as_str())).await?;
    let next_status = input.status.as_deref().unwrap_or(&current.status);
    status(next_status)?;
    let expected = input.expected_version.unwrap_or(current.version);
    let mut tx = pool.begin().await?;
    if !queries::update_if_version(&mut tx, template_id, expected, &input, next_status, amount).await? {
        return Err(Error::conflict(TEMPLATE_VERSION_CONFLICT));
    }
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "accommodation_template.update",
        resource_type: "accommodation_template",
        resource_id: template_id,
        reason: None,
        change: json!({ "status": next_status }),
    })
    .await?;
    tx.commit().await?;
    find_with_cover(pool, storage, template_id).await
}
