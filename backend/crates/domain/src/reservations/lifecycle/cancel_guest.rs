use chrono::Utc;
use kernel::audit::{self, AuditActor, AuditEntry};
use kernel::auth::Actor;
use kernel::idempotency;
use kernel::{Error, Kernel, Result};
use sqlx::PgPool;
use serde_json::json;

use super::cancellation::{cancellation_terms, load_source, read_contract};
use super::queries::{cancel_confirmed, release_item_assignments};
use super::state::{CANCELLED, CONFIRMED};
use super::types::{CancelStayInput, CancelledStayView};
use crate::reservations::codes::{
    CANNOT_CANCEL, NO_CANCELLATION_DEADLINE, REQUEST_IN_PROGRESS, RESERVATION_NOT_FOUND, RESERVATION_VERSION_CONFLICT,
};
use crate::reservations::events::{self, types::CANCELLED as CANCELLED_EVENT};
use crate::reservations::holds::release::release_reservation_nights;
use crate::reservations::links::{billing, pricing};
use crate::reservations::support::{parse_minor, wire_minor};

const IDEMPOTENCY_SCOPE: &str = "reservation.cancel";

async fn replay(pool: &PgPool, actor: &AuditActor, key: Option<&str>) -> Result<Option<CancelledStayView>> {
    let Some(key) = key else {
        return Ok(None);
    };
    let stored = idempotency::stored_response(pool, actor.city, IDEMPOTENCY_SCOPE, key).await?;
    stored
        .map(|value| serde_json::from_value(value).map_err(|error| Error::internal(format!("stored response unreadable: {error}"))))
        .transpose()
}

pub async fn cancel_stay(kernel: &Kernel, actor: &Actor, reservation_id: &str, input: CancelStayInput) -> Result<CancelledStayView> {
    cancel_stay_in(kernel.db.pool(), &actor.audit(), reservation_id, input).await
}

pub async fn cancel_stay_in(pool: &PgPool, actor: &AuditActor, reservation_id: &str, input: CancelStayInput) -> Result<CancelledStayView> {
    let city = actor.city;
    let session = actor.user_id.as_str();
    let key = input.idempotency_key.as_deref().map(idempotency::clean_key).transpose()?;
    if let Some(view) = replay(pool, actor, key).await? {
        return Ok(view);
    }
    let source = load_source(pool, city, reservation_id)
        .await?
        .filter(|source| source.guest_user_id == session)
        .ok_or_else(|| Error::not_found(RESERVATION_NOT_FOUND))?;
    if source.state != CONFIRMED {
        return Err(Error::conflict(CANNOT_CANCEL).with_params(json!({ "state": source.state })));
    }
    if source.version != input.expected_version {
        return Err(Error::conflict(RESERVATION_VERSION_CONFLICT));
    }
    let snapshot = read_contract(source.contract.clone())?;
    let terms = cancellation_terms(&snapshot.policies)?;
    let deadline = snapshot.cancellation_deadline.ok_or_else(|| Error::conflict(NO_CANCELLATION_DEADLINE))?;
    let now = Utc::now();
    let evaluation = pricing::evaluate(terms, deadline, now, parse_minor(&source.total_amount)?);
    let charge = wire_minor(evaluation.penalty_minor);

    let mut tx = pool.begin().await?;
    if let Some(key) = key {
        if !idempotency::claim(&mut tx, city, IDEMPOTENCY_SCOPE, key, reservation_id).await? {
            tx.rollback().await?;
            return replay(pool, actor, Some(key)).await?.ok_or_else(|| Error::conflict(REQUEST_IN_PROGRESS));
        }
    }
    if !cancel_confirmed(&mut tx, reservation_id, input.expected_version, now, evaluation.reason, &charge).await? {
        return Err(Error::conflict(RESERVATION_VERSION_CONFLICT));
    }
    release_reservation_nights(&mut tx, reservation_id).await?;
    release_item_assignments(&mut tx, reservation_id).await?;
    billing::release_for_penalty(&mut tx, reservation_id).await?;
    audit::record(
        &mut tx,
        actor,
        AuditEntry {
            action: "reservation.cancelled",
            resource_type: "reservation",
            resource_id: reservation_id,
            reason: Some(evaluation.reason),
            change: json!({ "from": CONFIRMED, "to": CANCELLED, "chargeDueAtProperty": charge, "refund": "0.00" }),
        },
    )
    .await?;
    events::reservation_with(&mut tx, city, reservation_id, CANCELLED_EVENT, json!({ "chargeDueAtProperty": charge })).await?;
    let view = CancelledStayView {
        reservation_id: reservation_id.to_string(),
        state: CANCELLED.into(),
        cancelled_at: now,
        reason: evaluation.reason.into(),
        currency: "CDF".into(),
        refund: wire_minor(0),
        charge_due_at_property: charge,
    };
    if let Some(key) = key {
        let stored = serde_json::to_value(&view).map_err(|error| Error::internal(error.to_string()))?;
        idempotency::store_response(&mut tx, city, IDEMPOTENCY_SCOPE, key, &stored).await?;
    }
    tx.commit().await?;
    Ok(view)
}
