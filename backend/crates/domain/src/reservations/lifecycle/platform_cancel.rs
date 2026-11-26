use chrono::Utc;
use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::permissions::names::PROPERTIES_MODERATE;
use kernel::{Error, Kernel, Result};
use serde_json::json;

use super::cancellation::{load_source, platform_preview};
use super::queries::{cancel_confirmed, release_item_assignments};
use super::state::{CANCELLED, CONFIRMED};
use super::types::{CancelPreview, PlatformCancelInput, ReservationView};
use super::view::require_view;
use crate::reservations::codes::{
    CANCEL_REASON_UNKNOWN, CANNOT_CANCEL, RESERVATION_NOT_FOUND, RESERVATION_VERSION_CONFLICT,
};
use crate::reservations::events::{self, types::CANCELLED as CANCELLED_EVENT};
use crate::reservations::holds::release::release_reservation_nights;
use crate::reservations::links::billing;

const PLATFORM_CANCEL_CODE: &str = "PLATFORM_CANCELLED";
pub const PLATFORM_CANCEL_REASONS: [&str; 6] = [
    "FRAUD_SUSPECTED",
    "POLICY_VIOLATION",
    "PROPERTY_UNAVAILABLE",
    "GUEST_REQUEST",
    "SAFETY_CONCERN",
    "PAYMENT_ISSUE",
];

pub async fn preview(kernel: &Kernel, actor: &Actor, reservation_id: &str) -> Result<CancelPreview> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    platform_preview(kernel.db.pool(), actor.city, reservation_id).await
}

pub async fn platform_cancel(kernel: &Kernel, actor: &Actor, reservation_id: &str, input: PlatformCancelInput) -> Result<ReservationView> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    if !PLATFORM_CANCEL_REASONS.contains(&input.reason_code.as_str()) {
        return Err(Error::bad_request(CANCEL_REASON_UNKNOWN).with_params(json!({ "reason": input.reason_code })));
    }
    let pool = kernel.db.pool();
    let city = actor.city;
    let source = load_source(pool, city, reservation_id)
        .await?
        .ok_or_else(|| Error::not_found(RESERVATION_NOT_FOUND))?;
    if source.state != CONFIRMED {
        return Err(Error::conflict(CANNOT_CANCEL).with_params(json!({ "state": source.state })));
    }
    let mut tx = pool.begin().await?;
    if !cancel_confirmed(&mut tx, reservation_id, input.expected_version, Utc::now(), PLATFORM_CANCEL_CODE, "0").await? {
        return Err(Error::conflict(RESERVATION_VERSION_CONFLICT));
    }
    release_reservation_nights(&mut tx, reservation_id).await?;
    release_item_assignments(&mut tx, reservation_id).await?;
    billing::release_commission(&mut tx, reservation_id, 0).await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "reservation.cancelled_by_platform",
            resource_type: "reservation",
            resource_id: reservation_id,
            reason: Some(input.reason_code.as_str()),
            change: json!({ "from": CONFIRMED, "to": CANCELLED, "chargeDueAtProperty": "0.00" }),
        },
    )
    .await?;
    events::reservation_with(
        &mut tx,
        city,
        reservation_id,
        CANCELLED_EVENT,
        json!({ "chargeDueAtProperty": "0.00", "cancelledBy": "PLATFORM" }),
    )
    .await?;
    tx.commit().await?;
    require_view(kernel, city, reservation_id).await
}
