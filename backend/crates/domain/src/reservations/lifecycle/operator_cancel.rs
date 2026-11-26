use chrono::Utc;
use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::permissions::names::RESERVATIONS_WRITE;
use kernel::{Error, Kernel, Result};
use serde_json::json;

use super::access::ensure_manage;
use super::decision_queries::find_for_decision;
use super::queries::{cancel_confirmed, release_item_assignments};
use super::state::{CANCELLED, CONFIRMED};
use super::types::{OperatorCancelInput, ReservationView};
use super::view::require_view;
use crate::reservations::codes::{
    CANCEL_REASON_REQUIRED, CANCEL_REASON_TOO_LONG, CANNOT_CANCEL, RESERVATION_NOT_FOUND, RESERVATION_VERSION_CONFLICT,
};
use crate::reservations::events::{self, types::CANCELLED as CANCELLED_EVENT};
use crate::reservations::holds::release::release_reservation_nights;
use crate::reservations::links::billing;

const MAX_REASON_CHARS: usize = 300;
const OPERATOR_CANCEL_CODE: &str = "OPERATOR_CANCELLED";

pub fn clean_reason(reason: &str) -> Result<String> {
    let reason = reason.trim().to_string();
    if reason.is_empty() {
        return Err(Error::bad_request(CANCEL_REASON_REQUIRED));
    }
    if reason.chars().count() > MAX_REASON_CHARS {
        return Err(Error::bad_request(CANCEL_REASON_TOO_LONG).with_params(json!({ "max": MAX_REASON_CHARS })));
    }
    Ok(reason)
}

pub async fn operator_cancel(kernel: &Kernel, actor: &Actor, reservation_id: &str, input: OperatorCancelInput) -> Result<ReservationView> {
    actor.require(RESERVATIONS_WRITE)?;
    let reason = clean_reason(&input.reason)?;
    let pool = kernel.db.pool();
    let city = actor.city;
    let row = find_for_decision(pool, city, reservation_id)
        .await?
        .ok_or_else(|| Error::not_found(RESERVATION_NOT_FOUND))?;
    ensure_manage(actor, &row)?;
    if row.state != CONFIRMED {
        return Err(Error::conflict(CANNOT_CANCEL).with_params(json!({ "state": row.state })));
    }
    let mut tx = pool.begin().await?;
    if !cancel_confirmed(&mut tx, reservation_id, input.expected_version, Utc::now(), OPERATOR_CANCEL_CODE, "0").await? {
        return Err(Error::conflict(RESERVATION_VERSION_CONFLICT));
    }
    release_reservation_nights(&mut tx, reservation_id).await?;
    release_item_assignments(&mut tx, reservation_id).await?;
    billing::release_commission(&mut tx, reservation_id, 0).await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "reservation.cancelled_by_operator",
            resource_type: "reservation",
            resource_id: reservation_id,
            reason: Some(reason.as_str()),
            change: json!({ "from": CONFIRMED, "to": CANCELLED, "chargeDueAtProperty": "0.00" }),
        },
    )
    .await?;
    events::reservation_with(
        &mut tx,
        city,
        reservation_id,
        CANCELLED_EVENT,
        json!({ "chargeDueAtProperty": "0.00", "cancelledBy": "OPERATOR" }),
    )
    .await?;
    tx.commit().await?;
    require_view(kernel, city, reservation_id).await
}
