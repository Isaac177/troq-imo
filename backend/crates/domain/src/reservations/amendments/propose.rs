use chrono::{Duration, Utc};
use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::ids::new_id;
use kernel::permissions::names::RESERVATIONS_WRITE;
use kernel::{Error, Kernel, Result};
use serde_json::json;

use super::availability::dates_available;
use super::queries::{self, NewAmendment, ReservationRow};
use super::types::{AmendmentProposal, ProposeInput};
use crate::reservations::codes::{
    AMENDMENT_ALREADY_PENDING, AMENDMENT_CHECK_IN_PAST, AMENDMENT_CLOSED_STAY_STARTED, AMENDMENT_NIGHTS_RANGE,
    AMENDMENT_RESERVATION_CHANGED, DATES_UNAVAILABLE, RATE_PLAN_GONE, RESERVATION_NOT_FOUND, RESERVATION_STATE_INVALID,
};
use crate::reservations::contract::{DiscountView, NightLine};
use crate::reservations::lifecycle::cancellation::{cancellation_terms, read_contract};
use crate::reservations::lifecycle::state::CONFIRMED;
use crate::reservations::events::{self, types::AMENDMENT_PROPOSED};
use crate::reservations::links::{inventory, pricing, settings};
use crate::reservations::support::{nights_between, parse_minor, signed_wire_minor, today_in, unique_violation, wire_minor};

const MAX_STAY_NIGHTS: i64 = 365;

pub async fn propose_amendment(kernel: &Kernel, actor: &Actor, reservation_id: &str, input: ProposeInput) -> Result<AmendmentProposal> {
    let pool = kernel.db.pool();
    let reservation = queries::find_reservation(pool, actor.city, reservation_id)
        .await?
        .filter(|row| row.guest_user_id == actor.user_id().as_str())
        .ok_or_else(|| Error::not_found(RESERVATION_NOT_FOUND))?;
    create_proposal(kernel, actor, reservation, input).await
}

pub async fn propose_operator_amendment(kernel: &Kernel, actor: &Actor, reservation_id: &str, input: ProposeInput) -> Result<AmendmentProposal> {
    actor.require(RESERVATIONS_WRITE)?;
    let pool = kernel.db.pool();
    let reservation = queries::find_reservation(pool, actor.city, reservation_id)
        .await?
        .ok_or_else(|| Error::not_found(RESERVATION_NOT_FOUND))?;
    actor
        .ensure_host_scope(RESERVATIONS_WRITE, &reservation.operator_id, &reservation.property_id)
        .map_err(crate::reservations::lifecycle::access::hide_membership)?;
    create_proposal(kernel, actor, reservation, input).await
}

async fn create_proposal(kernel: &Kernel, actor: &Actor, reservation: ReservationRow, input: ProposeInput) -> Result<AmendmentProposal> {
    let pool = kernel.db.pool();
    let city = actor.city;
    if reservation.state != CONFIRMED {
        return Err(Error::conflict(RESERVATION_STATE_INVALID).with_params(json!({ "state": reservation.state })));
    }
    let offset = kernel::timezone::offset_for_timezone(&reservation.timezone)
        .ok_or_else(|| Error::internal("property timezone has no offset"))?;
    let today = today_in(&reservation.timezone, Utc::now())?;
    if reservation.check_in <= today {
        return Err(Error::conflict(AMENDMENT_CLOSED_STAY_STARTED));
    }
    let nights_count = (input.check_out - input.check_in).num_days();
    if !(1..=MAX_STAY_NIGHTS).contains(&nights_count) {
        return Err(Error::bad_request(AMENDMENT_NIGHTS_RANGE).with_params(json!({ "max": MAX_STAY_NIGHTS })));
    }
    if input.check_in < today {
        return Err(Error::bad_request(AMENDMENT_CHECK_IN_PAST));
    }
    let context = pricing::load_context(pool, city, &reservation.property_id, &reservation.rate_plan_id)
        .await?
        .ok_or_else(|| Error::not_found(RATE_PLAN_GONE))?;
    pricing::ensure_bookable(&context, &reservation.unit_type_id, reservation.adult_count, reservation.child_count)?;
    inventory::enforce_amendment(
        pool,
        &reservation.property_id,
        &inventory::StayRequest {
            unit_type_id: reservation.unit_type_id.clone(),
            check_in: input.check_in,
            check_out: input.check_out,
            booking_day: today,
        },
        &reservation.id,
    )
    .await?;
    let old_nights = nights_between(reservation.check_in, reservation.check_out);
    let new_nights = nights_between(input.check_in, input.check_out);
    if !dates_available(pool, city, &reservation.unit_type_id, &reservation.id, &old_nights, &new_nights, input.check_in, input.check_out).await? {
        return Err(Error::conflict(DATES_UNAVAILABLE));
    }

    let rules = pricing::load_rules(pool, city, &reservation.rate_plan_id).await?;
    let mut promotions = pricing::load_active(pool, city, &reservation.property_id, &reservation.rate_plan_id, input.check_in, input.check_out).await?;
    let existing = reservation
        .contract
        .get("discount")
        .and_then(|discount| discount.get("promotionId"))
        .and_then(|id| id.as_str());
    if let Some(existing) = existing.and_then(|id| promotions.iter_mut().find(|promotion| promotion.id == id)) {
        existing.redeemed = existing.redeemed.saturating_sub(1);
    }
    let snapshot = read_contract(reservation.contract.clone())?;
    let deadline = pricing::free_cancellation_deadline(input.check_in, offset, cancellation_terms(&snapshot.policies)?)?;
    let priced = pricing::price_stay(
        &new_nights,
        &pricing::StayTerms {
            rules: &rules,
            guest_factor: pricing::guest_factor(&context.occupancy_basis, reservation.adult_count),
            child_nightly_minor: parse_minor(&context.child_nightly_amount)?,
            child_count: i64::from(reservation.child_count),
            promotions: &promotions,
        },
    )?;
    let lines: Vec<NightLine> = priced
        .nights
        .iter()
        .map(|night| NightLine { date: night.date, amount: wire_minor(night.amount_minor) })
        .collect();
    let delta_text = signed_wire_minor(priced.total_minor - parse_minor(&reservation.total)?);
    let total = wire_minor(priced.total_minor);
    let amendment_id = new_id("amend");
    let contract = json!({
        "checkIn": input.check_in,
        "checkOut": input.check_out,
        "nights": lines,
        "total": total,
        "subtotal": wire_minor(priced.subtotal_minor),
        "discount": priced.discount.as_ref().map(|discount| DiscountView {
            promotion_id: discount.promotion_id.clone(),
            amount: wire_minor(discount.amount_minor),
        }),
        "cancellationDeadline": deadline,
    });
    let expires_at = Utc::now() + Duration::hours(settings::amendment_window_hours(pool, city).await?);
    let mut tx = pool.begin().await?;
    let current: Option<i32> = sqlx::query_scalar(
        r#"SELECT version FROM stay_reservations WHERE id = $1 AND state = 'CONFIRMED' FOR UPDATE"#,
    )
    .bind(&reservation.id)
    .fetch_optional(&mut *tx)
    .await?;
    if current != Some(reservation.version) {
        return Err(Error::conflict(AMENDMENT_RESERVATION_CHANGED));
    }
    queries::close_stale_amendments(&mut tx, &reservation.id, reservation.version).await?;
    let inserted = queries::insert_amendment(
        &mut tx,
        &NewAmendment {
            id: &amendment_id,
            city,
            reservation_id: &reservation.id,
            original_version: reservation.version,
            contract: &contract,
            delta: &delta_text,
            expires_at,
        },
    )
    .await;
    if let Err(error) = inserted {
        if unique_violation(&error, "stay_amendments_open_unique_idx") {
            return Err(Error::conflict(AMENDMENT_ALREADY_PENDING));
        }
        return Err(error.into());
    }
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "reservation.amendment_proposed",
            resource_type: "reservation",
            resource_id: &reservation.id,
            reason: None,
            change: json!({ "amendmentId": amendment_id, "delta": delta_text }),
        },
    )
    .await?;
    events::emit(
        &mut tx,
        city,
        "reservation",
        &reservation.id,
        AMENDMENT_PROPOSED,
        json!({ "reservationId": reservation.id, "amendmentId": amendment_id }),
    )
    .await?;
    tx.commit().await?;
    Ok(AmendmentProposal {
        id: amendment_id,
        reservation_id: reservation.id,
        state: "PROPOSED".into(),
        check_in: input.check_in,
        check_out: input.check_out,
        nights: lines,
        total,
        delta: delta_text,
        expires_at,
    })
}
