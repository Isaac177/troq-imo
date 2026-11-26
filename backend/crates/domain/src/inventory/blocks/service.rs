use chrono::NaiveDate;
use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::error::{Error, Result};
use kernel::idempotency;
use kernel::ids::new_id;
use kernel::outbox;
use kernel::paging::{Page, PageRequest};
use kernel::permissions::names::{INVENTORY_WRITE, PROPERTIES_READ};
use serde_json::json;
use sqlx::PgPool;

use super::queries::{self, NewBlock};
use super::types::{BlockView, CreateBlockInput, RemoveBlockInput};
use super::validation::{validate_range, validate_reason, PLATFORM_REASON};
use crate::inventory::messages::{
    BLOCK_ALREADY_REMOVED, BLOCK_NOT_FOUND, BLOCK_PLATFORM_ONLY, BLOCK_PLATFORM_REASON_RESERVED,
    BLOCK_VERSION_CONFLICT, IDEMPOTENCY_IN_PROGRESS, NIGHTS_UNAVAILABLE, UNIT_TYPE_NOT_FOUND,
};
use crate::inventory::nights::{release, take_range, ReleaseState};
use crate::inventory::scope::{actor_property, host_property, lock_property, unit_type_belongs};

const CREATE_SCOPE: &str = "block.create";

fn join_dates(dates: &[NaiveDate]) -> String {
    dates.iter().map(|date| date.to_string()).collect::<Vec<_>>().join(", ")
}

pub async fn create(
    pool: &PgPool,
    actor: &Actor,
    input: CreateBlockInput,
    as_admin: bool,
) -> Result<BlockView> {
    let property = actor_property(pool, actor, &input.property_id, INVENTORY_WRITE, as_admin).await?;
    validate_range(input.start_date, input.end_date)?;
    validate_reason(&input.reason)?;
    if (input.reason == PLATFORM_REASON) != as_admin {
        return Err(Error::bad_request(BLOCK_PLATFORM_REASON_RESERVED));
    }
    if !unit_type_belongs(pool, &property.id, &input.unit_type_id).await? {
        return Err(Error::not_found(UNIT_TYPE_NOT_FOUND));
    }
    let key = input.idempotency_key.as_deref().map(idempotency::clean_key).transpose()?;
    if let Some(key) = key {
        if let Some(existing) = idempotency::resource_for(pool, property.city, CREATE_SCOPE, key).await? {
            return queries::find(pool, &existing)
                .await?
                .ok_or_else(|| Error::internal("idempotent block not readable"));
        }
    }
    let block_id = new_id("block");
    let mut tx = pool.begin().await?;
    if let Some(key) = key {
        if !idempotency::claim(&mut tx, property.city, CREATE_SCOPE, key, &block_id).await? {
            return Err(Error::conflict(IDEMPOTENCY_IN_PROGRESS));
        }
    }
    lock_property(&mut tx, &property.id).await?;
    let unavailable = take_range(
        &mut tx,
        property.city,
        &input.unit_type_id,
        "BLOCK",
        &block_id,
        input.start_date,
        input.end_date,
        None,
    )
    .await?;
    if !unavailable.is_empty() {
        return Err(Error::conflict(NIGHTS_UNAVAILABLE)
            .with_params(json!({ "nights": join_dates(&unavailable), "dates": unavailable })));
    }
    queries::insert(
        &mut tx,
        NewBlock {
            id: &block_id,
            city: property.city,
            property_id: &property.id,
            unit_type_id: &input.unit_type_id,
            start_date: input.start_date,
            end_date: input.end_date,
            reason: &input.reason,
            origin: "OPERATOR",
            actor: actor.user_id().as_str(),
        },
    )
    .await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "property.block_created",
            resource_type: "property",
            resource_id: &property.id,
            reason: Some(&input.reason),
            change: json!({ "blockId": block_id, "unitTypeId": input.unit_type_id, "startDate": input.start_date, "endDate": input.end_date }),
        },
    )
    .await?;
    outbox::enqueue(
        &mut tx,
        property.city,
        "property",
        &property.id,
        "property.block_created",
        json!({ "blockId": block_id, "propertyId": property.id }),
    )
    .await?;
    tx.commit().await?;
    queries::find(pool, &block_id)
        .await?
        .ok_or_else(|| Error::internal("created block not readable"))
}

pub async fn remove(
    pool: &PgPool,
    actor: &Actor,
    block_id: &str,
    input: RemoveBlockInput,
    as_admin: bool,
) -> Result<BlockView> {
    let block = queries::find(pool, block_id)
        .await?
        .ok_or_else(|| Error::not_found(BLOCK_NOT_FOUND))?;
    if block.removed_at.is_some() {
        return Err(Error::conflict(BLOCK_ALREADY_REMOVED));
    }
    if block.reason == PLATFORM_REASON && !as_admin {
        return Err(Error::forbidden(BLOCK_PLATFORM_ONLY));
    }
    let property = actor_property(pool, actor, &block.property_id, INVENTORY_WRITE, as_admin).await?;
    let mut tx = pool.begin().await?;
    lock_property(&mut tx, &property.id).await?;
    if !queries::remove_if_version(&mut tx, &block.id, input.expected_version).await? {
        return Err(Error::conflict(BLOCK_VERSION_CONFLICT));
    }
    let released = release(&mut tx, "BLOCK", &block.id, ReleaseState::Released).await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "property.block_removed",
            resource_type: "property",
            resource_id: &property.id,
            reason: None,
            change: json!({ "blockId": block.id, "releasedNights": released.nights }),
        },
    )
    .await?;
    outbox::enqueue(
        &mut tx,
        property.city,
        "property",
        &property.id,
        "property.block_removed",
        json!({ "blockId": block.id, "propertyId": property.id }),
    )
    .await?;
    tx.commit().await?;
    queries::find(pool, &block.id)
        .await?
        .ok_or_else(|| Error::internal("removed block not readable"))
}

pub async fn list(
    pool: &PgPool,
    actor: &Actor,
    property_id: &str,
    page: PageRequest,
    as_admin: bool,
) -> Result<Page<BlockView>> {
    let property = actor_property(pool, actor, property_id, PROPERTIES_READ, as_admin).await?;
    let (items, total) = queries::list(pool, &property.id, page.limit(), page.offset()).await?;
    Ok(Page::new(items, total, page))
}

pub async fn get(pool: &PgPool, actor: &Actor, block_id: &str) -> Result<BlockView> {
    let block = queries::find(pool, block_id)
        .await?
        .ok_or_else(|| Error::not_found(BLOCK_NOT_FOUND))?;
    host_property(pool, actor, &block.property_id, PROPERTIES_READ).await?;
    Ok(block)
}
