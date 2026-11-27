use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::error::{codes, Error, Result};
use kernel::outbox;
use kernel::permissions::names::{PROPERTIES_READ, PROPERTIES_WRITE};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use utoipa::ToSchema;

use super::contact_validation;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateHostContact {
    pub guest_contact_phone: String,
    pub guest_contact_email: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HostContact {
    pub host_id: String,
    pub guest_contact_phone: String,
    pub guest_contact_email: String,
}

pub async fn get_contact(pool: &PgPool, actor: &Actor, host_id: &str) -> Result<HostContact> {
    if !actor.access.is_member_of(host_id) {
        return Err(Error::not_found(codes::NOT_FOUND));
    }
    actor.require_scoped_any(&[PROPERTIES_READ, PROPERTIES_WRITE], Some(host_id), None)?;
    let row: Option<(String, String, String)> = sqlx::query_as(
        "SELECT h.id, h.guest_contact_phone, h.guest_contact_email FROM hosts h WHERE h.id = $1 AND h.city_id = $2 AND h.status = 'ACTIVE' AND EXISTS (SELECT 1 FROM host_members m JOIN identity_users u ON u.id = m.user_id WHERE m.host_id = h.id AND m.user_id = $3 AND m.status = 'ACTIVE' AND u.status = 'ACTIVE')",
    )
    .bind(host_id)
    .bind(actor.city.code())
    .bind(actor.user_id().as_str())
    .fetch_optional(pool)
    .await?;
    let (host_id, guest_contact_phone, guest_contact_email) = row.ok_or_else(|| Error::not_found(codes::NOT_FOUND))?;
    Ok(HostContact { host_id, guest_contact_phone, guest_contact_email })
}

pub async fn update_contact(
    pool: &PgPool,
    actor: &Actor,
    host_id: &str,
    input: UpdateHostContact,
) -> Result<HostContact> {
    actor.ensure_writable()?;
    actor.require(PROPERTIES_WRITE)?;
    if !actor.access.is_member_of(host_id) {
        return Err(Error::not_found(codes::NOT_FOUND));
    }
    actor.require_scoped(PROPERTIES_WRITE, Some(host_id), None)?;
    let phone = contact_validation::phone(&input.guest_contact_phone)?;
    let email = contact_validation::email(&input.guest_contact_email)?;
    let mut tx = pool.begin().await?;
    let updated: Option<(String,)> = sqlx::query_as(
        "UPDATE hosts h SET guest_contact_phone = $3, guest_contact_email = $4 WHERE h.id = $1 AND h.city_id = $2 AND h.status = 'ACTIVE' AND EXISTS (SELECT 1 FROM host_members m JOIN identity_users u ON u.id = m.user_id WHERE m.host_id = h.id AND m.user_id = $5 AND m.status = 'ACTIVE' AND u.status = 'ACTIVE') RETURNING h.id",
    )
    .bind(host_id)
    .bind(actor.city.code())
    .bind(&phone)
    .bind(&email)
    .bind(actor.user_id().as_str())
    .fetch_optional(&mut *tx)
    .await?;
    if updated.is_none() {
        return Err(Error::not_found(codes::NOT_FOUND));
    }
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "host.contact.updated",
        resource_type: "host",
        resource_id: host_id,
        reason: None,
        change: json!({ "fields": ["guestContactPhone", "guestContactEmail"] }),
    }).await?;
    outbox::enqueue(&mut tx, actor.city, "host", host_id, "host.contact.updated", json!({ "hostId": host_id })).await?;
    tx.commit().await?;
    Ok(HostContact { host_id: host_id.to_owned(), guest_contact_phone: phone, guest_contact_email: email })
}
