use kernel::auth::Actor;
use kernel::error::Result;
use kernel::permissions::names::{BILLING_READ, PROPERTIES_MODERATE};
use kernel::{City, Error};
use sqlx::PgPool;

use super::messages::{HOST_NOT_FOUND, HOST_REQUIRED};

pub fn require_staff(actor: &Actor) -> Result<()> {
    actor.require_platform(PROPERTIES_MODERATE)
}

pub async fn ensure_host(pool: &PgPool, host_id: &str, city: City) -> Result<()> {
    let found: Option<String> = sqlx::query_scalar("SELECT id FROM hosts WHERE id = $1 AND city_id = $2")
        .bind(host_id)
        .bind(city.code())
        .fetch_optional(pool)
        .await?;
    found.map(|_| ()).ok_or_else(|| Error::not_found(HOST_NOT_FOUND))
}

pub async fn authorize_host_read(pool: &PgPool, actor: &Actor, host_id: &str) -> Result<()> {
    if host_id.trim().is_empty() {
        return Err(Error::bad_request(HOST_REQUIRED));
    }
    if !actor.access.is_member_of(host_id) {
        return Err(Error::not_found(HOST_NOT_FOUND));
    }
    actor.require_scoped(BILLING_READ, Some(host_id), None)?;
    ensure_host(pool, host_id, actor.city).await
}
