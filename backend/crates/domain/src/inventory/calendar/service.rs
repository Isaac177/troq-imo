use kernel::auth::Actor;
use kernel::error::Result;
use kernel::storage::Storage;
use kernel::permissions::names::{RESERVATIONS_GUEST_DETAILS, RESERVATIONS_READ};
use sqlx::PgPool;

use super::types::{validate_range, CalendarQuery, CalendarView};
use super::view::{build_view, feed_text};
use crate::inventory::query::paging;
use crate::inventory::scope::{admin_property, host_property};

pub async fn host_calendar(
    pool: &PgPool,
    storage: &Storage,
    actor: &Actor,
    query: CalendarQuery,
) -> Result<CalendarView> {
    actor.require(RESERVATIONS_READ)?;
    validate_range(query.from, query.to)?;
    let property = host_property(pool, actor, &query.property_id, RESERVATIONS_READ).await?;
    let guest_names = actor.require(RESERVATIONS_GUEST_DETAILS).is_ok()
        && host_property(pool, actor, &query.property_id, RESERVATIONS_GUEST_DETAILS)
            .await
            .is_ok();
    let page = paging(query.page, query.page_size)?;
    build_view(pool, storage, &property, query.from, query.to, page, guest_names).await
}

pub async fn admin_calendar(
    pool: &PgPool,
    storage: &Storage,
    actor: &Actor,
    query: CalendarQuery,
) -> Result<CalendarView> {
    actor.require_platform(kernel::permissions::names::PROPERTIES_MODERATE)?;
    validate_range(query.from, query.to)?;
    let property = admin_property(pool, actor, &query.property_id).await?;
    let page = paging(query.page, query.page_size)?;
    build_view(pool, storage, &property, query.from, query.to, page, true).await
}

pub async fn host_feed(pool: &PgPool, actor: &Actor, property_id: &str) -> Result<String> {
    actor.require(RESERVATIONS_READ)?;
    let property = host_property(pool, actor, property_id, RESERVATIONS_READ).await?;
    feed_text(pool, &property.id).await
}
