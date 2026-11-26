use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::error::{Error, Result};
use kernel::ids::new_id;
use kernel::outbox;
use kernel::permissions::names::{RESERVATIONS_READ, RESERVATIONS_WRITE};
use serde_json::json;
use sqlx::PgPool;

use super::buffer::turnover_blocked;
use super::queries;
use super::types::ItemContext;
use super::types::{AssignUnitInput, AssignmentView, MoveUnitInput, UnitOptionView};
use crate::inventory::messages::{
    ASSIGNMENT_ALREADY_HAS_UNIT, ASSIGNMENT_CHANGED, ASSIGNMENT_INACTIVE, ASSIGNMENT_MOVE_DATE_INVALID,
    ASSIGNMENT_NOT_FOUND, ASSIGNMENT_RESERVATION_CHANGED, ASSIGNMENT_SAME_UNIT, ASSIGNMENT_STATE_INVALID,
    ITEM_NOT_FOUND, UNIT_INCOMPATIBLE, UNIT_TURNOVER_BUFFER, UNIT_UNAVAILABLE,
};
use crate::inventory::scope::{host_property, lock_property, PropertyRef};

fn assignable(state: &str) -> bool {
    state == "CONFIRMED" || state == "CHECKED_IN"
}

async fn item_with_scope(
    pool: &PgPool,
    actor: &Actor,
    item_id: &str,
    permission: &str,
) -> Result<(ItemContext, PropertyRef)> {
    let item = queries::item_context(pool, item_id)
        .await?
        .ok_or_else(|| Error::not_found(ITEM_NOT_FOUND))?;
    let property = host_property(pool, actor, &item.property_id, permission).await?;
    Ok((item, property))
}

async fn ensure_free_of_buffer(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    unit_id: &str,
    reservation_id: &str,
    from: chrono::NaiveDate,
    to: chrono::NaiveDate,
) -> Result<()> {
    match turnover_blocked(tx, unit_id, reservation_id, from, to).await? {
        Some(buffer) => Err(Error::conflict(UNIT_TURNOVER_BUFFER).with_params(json!({ "nights": buffer }))),
        None => Ok(()),
    }
}

pub async fn assign(pool: &PgPool, actor: &Actor, input: AssignUnitInput) -> Result<AssignmentView> {
    let (item, property) = item_with_scope(pool, actor, &input.reservation_item_id, RESERVATIONS_WRITE).await?;
    if !assignable(&item.state) {
        return Err(Error::conflict(ASSIGNMENT_STATE_INVALID));
    }
    if !queries::unit_is_compatible(pool, &input.physical_unit_id, &item.unit_type_id).await? {
        return Err(Error::bad_request(UNIT_INCOMPATIBLE));
    }
    let assignment_id = new_id("assign");
    let mut tx = pool.begin().await?;
    lock_property(&mut tx, &item.property_id).await?;
    let item = queries::lock_item_context(&mut tx, &item.item_id)
        .await?
        .ok_or_else(|| Error::not_found(ITEM_NOT_FOUND))?;
    if !assignable(&item.state) {
        return Err(Error::conflict(ASSIGNMENT_STATE_INVALID));
    }
    queries::lock_unit(&mut tx, &input.physical_unit_id).await?;
    if let Some(active) = queries::history(pool, &item.item_id)
        .await?
        .into_iter()
        .find(|row| row.released_at.is_none())
    {
        if active.physical_unit_id == input.physical_unit_id {
            return Ok(active);
        }
        return Err(Error::conflict(ASSIGNMENT_ALREADY_HAS_UNIT));
    }
    if queries::unit_blocked(&mut tx, &input.physical_unit_id, item.check_in, item.check_out, &item.reservation_id).await? {
        return Err(Error::conflict(UNIT_UNAVAILABLE));
    }
    ensure_free_of_buffer(&mut tx, &input.physical_unit_id, &item.reservation_id, item.check_in, item.check_out).await?;
    queries::insert_assignment(
        &mut tx,
        property.city,
        &assignment_id,
        &item.item_id,
        &input.physical_unit_id,
        item.check_in,
        item.check_out,
        "ASSIGNED",
        actor.user_id().as_str(),
    )
    .await?;
    queries::set_item_unit(&mut tx, &item.item_id, &input.physical_unit_id).await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "stay.unit_assigned",
            resource_type: "reservation_item",
            resource_id: &item.item_id,
            reason: None,
            change: json!({ "assignmentId": assignment_id, "physicalUnitId": input.physical_unit_id }),
        },
    )
    .await?;
    outbox::enqueue(
        &mut tx,
        property.city,
        "reservation",
        &item.reservation_id,
        "reservation.unit_assigned",
        json!({ "reservationItemId": item.item_id, "assignmentId": assignment_id }),
    )
    .await?;
    tx.commit().await?;
    queries::assignment_by_id(pool, &assignment_id)
        .await?
        .ok_or_else(|| Error::internal("assignment not readable"))
}

pub async fn move_unit(
    pool: &PgPool,
    actor: &Actor,
    assignment_id: &str,
    input: MoveUnitInput,
) -> Result<AssignmentView> {
    let current = queries::assignment_by_id(pool, assignment_id)
        .await?
        .ok_or_else(|| Error::not_found(ASSIGNMENT_NOT_FOUND))?;
    if current.released_at.is_some() {
        return Err(Error::conflict(ASSIGNMENT_INACTIVE));
    }
    let (item, property) = item_with_scope(pool, actor, &current.reservation_item_id, RESERVATIONS_WRITE).await?;
    if input.from_date < current.start_date || input.from_date >= current.end_date {
        return Err(Error::bad_request(ASSIGNMENT_MOVE_DATE_INVALID));
    }
    if !queries::unit_is_compatible(pool, &input.physical_unit_id, &item.unit_type_id).await? {
        return Err(Error::bad_request(UNIT_INCOMPATIBLE));
    }
    if input.physical_unit_id == current.physical_unit_id {
        return Err(Error::bad_request(ASSIGNMENT_SAME_UNIT));
    }
    let new_id = new_id("assign");
    let mut tx = pool.begin().await?;
    lock_property(&mut tx, &item.property_id).await?;
    let item = queries::lock_item_context(&mut tx, &item.item_id)
        .await?
        .ok_or_else(|| Error::not_found(ITEM_NOT_FOUND))?;
    if !assignable(&item.state) || current.start_date < item.check_in || current.end_date > item.check_out {
        return Err(Error::conflict(ASSIGNMENT_RESERVATION_CHANGED));
    }
    queries::lock_unit(&mut tx, &input.physical_unit_id).await?;
    if queries::unit_blocked(&mut tx, &input.physical_unit_id, input.from_date, current.end_date, &item.reservation_id).await? {
        return Err(Error::conflict(UNIT_UNAVAILABLE));
    }
    ensure_free_of_buffer(&mut tx, &input.physical_unit_id, &item.reservation_id, input.from_date, current.end_date).await?;
    if !queries::release_for_move(&mut tx, &current.id, input.from_date, current.start_date, current.end_date).await? {
        return Err(Error::conflict(ASSIGNMENT_CHANGED));
    }
    queries::insert_assignment(
        &mut tx,
        property.city,
        &new_id,
        &item.item_id,
        &input.physical_unit_id,
        input.from_date,
        current.end_date,
        "MOVED",
        actor.user_id().as_str(),
    )
    .await?;
    if input.from_date == current.start_date && current.start_date == item.check_in {
        queries::set_item_unit(&mut tx, &item.item_id, &input.physical_unit_id).await?;
    }
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "stay.unit_moved",
            resource_type: "reservation_item",
            resource_id: &item.item_id,
            reason: None,
            change: json!({ "fromAssignment": current.id, "toAssignment": new_id, "fromDate": input.from_date, "physicalUnitId": input.physical_unit_id }),
        },
    )
    .await?;
    tx.commit().await?;
    queries::assignment_by_id(pool, &new_id)
        .await?
        .ok_or_else(|| Error::internal("moved assignment not readable"))
}

pub async fn history(pool: &PgPool, actor: &Actor, item_id: &str) -> Result<Vec<AssignmentView>> {
    let (item, _) = item_with_scope(pool, actor, item_id, RESERVATIONS_READ).await?;
    queries::history(pool, &item.item_id).await
}

pub async fn unit_options(pool: &PgPool, actor: &Actor, item_id: &str) -> Result<Vec<UnitOptionView>> {
    let (item, _) = item_with_scope(pool, actor, item_id, RESERVATIONS_WRITE).await?;
    queries::free_units(pool, &item.unit_type_id, item.check_in, item.check_out).await
}
