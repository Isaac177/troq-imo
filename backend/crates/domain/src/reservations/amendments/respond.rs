use chrono::Utc;
use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::{Error, Kernel, Result};
use serde_json::json;

use super::queries::{self, AmendmentRow, NewDates, ReservationRow};
use super::types::StoredAmendment;
use crate::reservations::codes::{
    AMENDMENT_CLOSED_STAY_STARTED, AMENDMENT_EXPIRED, AMENDMENT_INVENTORY_GONE, AMENDMENT_NOT_FOUND,
    AMENDMENT_NOT_PENDING, AMENDMENT_PROPOSED_CHECK_IN_PASSED, AMENDMENT_RESERVATION_CHANGED, AMENDMENT_STATE_INVALID,
    NIGHT_TAKEN,
};
use crate::reservations::lifecycle::state::CONFIRMED;
use crate::reservations::lifecycle::types::ReservationView;
use crate::reservations::lifecycle::view::require_view;
use crate::reservations::events::{self, types::{AMENDMENT_ACCEPTED, AMENDMENT_DECLINED}};
use crate::reservations::holds::release::release_reservation_nights;
use crate::reservations::links::inventory::{insert_allocation, take_night};
use crate::reservations::links::{billing, inventory, pricing, properties};
use crate::reservations::support::{retain_until, today_in};

async fn owned(kernel: &Kernel, actor: &Actor, amendment_id: &str) -> Result<(AmendmentRow, ReservationRow)> {
    let pool = kernel.db.pool();
    let amendment = queries::find_amendment(pool, actor.city, amendment_id)
        .await?
        .ok_or_else(|| Error::not_found(AMENDMENT_NOT_FOUND))?;
    let reservation = queries::find_reservation(pool, actor.city, &amendment.reservation_id)
        .await?
        .filter(|row| row.guest_user_id == actor.user_id().as_str())
        .ok_or_else(|| Error::not_found(AMENDMENT_NOT_FOUND))?;
    Ok((amendment, reservation))
}

fn ensure_pending(amendment: &AmendmentRow) -> Result<()> {
    if amendment.state != "PROPOSED" {
        return Err(Error::conflict(AMENDMENT_STATE_INVALID).with_params(json!({ "state": amendment.state })));
    }
    Ok(())
}

pub async fn accept_amendment(kernel: &Kernel, actor: &Actor, amendment_id: &str) -> Result<ReservationView> {
    let pool = kernel.db.pool();
    let city = actor.city;
    let (amendment, reservation) = owned(kernel, actor, amendment_id).await?;
    ensure_pending(&amendment)?;
    if amendment.expires_at <= Utc::now() {
        return Err(Error::conflict(AMENDMENT_EXPIRED));
    }
    if reservation.state != CONFIRMED || reservation.version != amendment.original_version {
        return Err(Error::conflict(AMENDMENT_RESERVATION_CHANGED));
    }
    let today = today_in(&reservation.timezone, Utc::now())?;
    if reservation.check_in <= today {
        return Err(Error::conflict(AMENDMENT_CLOSED_STAY_STARTED));
    }
    let stored: StoredAmendment = serde_json::from_value(amendment.contract.clone())
        .map_err(|error| Error::internal(format!("amendment contract unreadable: {error}")))?;
    if stored.check_in < today {
        return Err(Error::conflict(AMENDMENT_PROPOSED_CHECK_IN_PASSED));
    }
    inventory::enforce_amendment(
        pool,
        &reservation.property_id,
        &inventory::StayRequest {
            unit_type_id: reservation.unit_type_id.clone(),
            check_in: stored.check_in,
            check_out: stored.check_out,
            booking_day: today,
        },
        &reservation.id,
    )
    .await?;
    let nights_json = serde_json::to_value(&stored.nights).map_err(|error| Error::internal(error.to_string()))?;

    let mut tx = pool.begin().await?;
    properties::lock_property(&mut tx, &reservation.property_id).await?;
    let current_version: Option<i32> = sqlx::query_scalar(
        r#"SELECT version FROM stay_reservations WHERE id = $1 AND state = 'CONFIRMED' FOR UPDATE"#,
    )
    .bind(&reservation.id)
    .fetch_optional(&mut *tx)
    .await?;
    if current_version != Some(amendment.original_version) {
        return Err(Error::conflict(AMENDMENT_RESERVATION_CHANGED));
    }
    let current_amendment: Option<(String, chrono::DateTime<Utc>)> =
        sqlx::query_as(r#"SELECT state, "expiresAt" FROM stay_amendments WHERE id = $1 FOR UPDATE"#)
            .bind(&amendment.id)
            .fetch_optional(&mut *tx)
            .await?;
    if !current_amendment.is_some_and(|(state, expires_at)| state == "PROPOSED" && expires_at > Utc::now()) {
        return Err(Error::conflict(AMENDMENT_NOT_PENDING));
    }
    let previous_promotion = reservation
        .contract
        .get("discount")
        .and_then(|discount| discount.get("promotionId"))
        .and_then(|id| id.as_str());
    if let Some(discount) = stored.discount.as_ref() {
        if previous_promotion != Some(discount.promotion_id.as_str()) {
            pricing::lock_budget(&mut tx, &discount.promotion_id).await?;
        }
    }
    let pool_id: Option<String> = sqlx::query_scalar(r#"SELECT id FROM stay_inventory_pools WHERE "unitTypeId" = $1 FOR UPDATE"#)
        .bind(&reservation.unit_type_id)
        .fetch_optional(&mut *tx)
        .await?;
    if pool_id.is_none() {
        return Err(Error::conflict(AMENDMENT_INVENTORY_GONE));
    }
    release_reservation_nights(&mut tx, &reservation.id).await?;
    queries::release_unit_assignment(&mut tx, &reservation.item_id).await?;
    for night in stored.nights.iter().map(|line| line.date) {
        let pool_id = take_night(&mut tx, &reservation.unit_type_id, night)
            .await?
            .ok_or_else(|| Error::conflict(NIGHT_TAKEN))?;
        insert_allocation(&mut tx, city, &pool_id, "RESERVATION", &reservation.id, night, None).await?;
    }
    let deadline = stored.cancellation_deadline.or_else(|| {
        reservation
            .contract
            .get("cancellationDeadline")
            .and_then(|value| serde_json::from_value(value.clone()).ok())
    });
    let patch = json!({ "nights": nights_json.clone(), "discount": stored.discount, "cancellationDeadline": deadline });
    let applied = queries::apply_new_dates(
        &mut tx,
        &NewDates {
            item_id: &reservation.item_id,
            reservation_id: &reservation.id,
            original_version: amendment.original_version,
            check_in: stored.check_in,
            check_out: stored.check_out,
            nights: &nights_json,
            subtotal: stored.subtotal.as_deref().unwrap_or(&stored.total),
            total: &stored.total,
            contract_patch: &patch,
            retain_until: retain_until(stored.check_out, kernel.config.guest_retention_days),
        },
    )
    .await?;
    if !applied {
        return Err(Error::conflict(AMENDMENT_RESERVATION_CHANGED));
    }
    queries::reserve_new_room(&mut tx, city, &reservation, stored.check_in, stored.check_out).await?;
    billing::sync_amended_commission(&mut tx, &reservation.id).await?;
    queries::set_amendment_state(&mut tx, &amendment.id, "ACCEPTED").await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "reservation.amendment_accepted",
            resource_type: "reservation",
            resource_id: &reservation.id,
            reason: None,
            change: json!({ "amendmentId": amendment.id, "checkIn": stored.check_in, "checkOut": stored.check_out }),
        },
    )
    .await?;
    events::emit(
        &mut tx,
        city,
        "reservation",
        &reservation.id,
        AMENDMENT_ACCEPTED,
        json!({ "reservationId": reservation.id, "amendmentId": amendment.id }),
    )
    .await?;
    tx.commit().await?;
    require_view(kernel, city, &reservation.id).await
}

pub async fn decline_amendment(kernel: &Kernel, actor: &Actor, amendment_id: &str) -> Result<ReservationView> {
    let pool = kernel.db.pool();
    let city = actor.city;
    let (amendment, reservation) = owned(kernel, actor, amendment_id).await?;
    ensure_pending(&amendment)?;
    let mut tx = pool.begin().await?;
    queries::set_amendment_state(&mut tx, &amendment.id, "DECLINED").await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "reservation.amendment_declined",
            resource_type: "reservation",
            resource_id: &reservation.id,
            reason: None,
            change: json!({ "amendmentId": amendment.id }),
        },
    )
    .await?;
    events::emit(
        &mut tx,
        city,
        "reservation",
        &reservation.id,
        AMENDMENT_DECLINED,
        json!({ "reservationId": reservation.id, "amendmentId": amendment.id }),
    )
    .await?;
    tx.commit().await?;
    require_view(kernel, city, &reservation.id).await
}
