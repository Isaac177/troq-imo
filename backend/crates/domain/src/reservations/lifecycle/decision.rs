use chrono::Utc;
use kernel::audit::{self, AuditActor, AuditEntry};
use kernel::auth::Actor;
use kernel::{Error, Kernel, Result};
use serde_json::json;
use sqlx::PgPool;

use super::access::ensure_manage;
use super::decision_queries::{find_for_decision, move_request_if_version, DecisionRow};
use super::state::{CONFIRMED, DECLINED, REQUESTED};
use super::types::{DecisionInput, ReservationView};
use super::view::require_view;
use crate::reservations::codes::{
    APPROVAL_WINDOW_CLOSED, DECISION_UNKNOWN, NOT_AWAITING_APPROVAL, RESERVATION_NOT_FOUND, RESERVATION_VERSION_CONFLICT,
};
use crate::reservations::events::{self, types::{REQUEST_ACCEPTED, REQUEST_DECLINED}};
use crate::reservations::holds::release::{convert_holds_to_reservations, release_holds};
use crate::reservations::links::billing;

pub async fn decide_request(kernel: &Kernel, actor: &Actor, reservation_id: &str, input: DecisionInput) -> Result<ReservationView> {
    let pool = kernel.db.pool();
    let row = find_for_decision(pool, actor.city, reservation_id)
        .await?
        .ok_or_else(|| Error::not_found(RESERVATION_NOT_FOUND))?;
    ensure_manage(actor, &row)?;
    apply_decision(pool, &actor.audit(), reservation_id, &input.action, input.expected_version, row).await?;
    require_view(kernel, actor.city, reservation_id).await
}

pub async fn apply_decision(
    pool: &PgPool,
    actor: &AuditActor,
    reservation_id: &str,
    action: &str,
    expected_version: i32,
    row: DecisionRow,
) -> Result<()> {
    let (new_state, event) = match action {
        "ACCEPT" => (CONFIRMED, REQUEST_ACCEPTED),
        "DECLINE" => (DECLINED, REQUEST_DECLINED),
        _ => return Err(Error::bad_request(DECISION_UNKNOWN).with_params(json!({ "action": action }))),
    };
    if row.state != REQUESTED {
        return Err(Error::conflict(NOT_AWAITING_APPROVAL).with_params(json!({ "state": row.state })));
    }
    if row.approval_deadline_at.is_some_and(|deadline| deadline <= Utc::now()) {
        return Err(Error::conflict(APPROVAL_WINDOW_CLOSED));
    }
    let mut tx = pool.begin().await?;
    if !move_request_if_version(&mut tx, reservation_id, expected_version, new_state).await? {
        return Err(Error::conflict(RESERVATION_VERSION_CONFLICT));
    }
    if new_state == CONFIRMED {
        convert_holds_to_reservations(&mut tx, reservation_id).await?;
        billing::take_commission(&mut tx, reservation_id).await?;
    } else {
        release_holds(&mut tx, reservation_id).await?;
    }
    audit::record(
        &mut tx,
        actor,
        AuditEntry {
            action: "reservation.decide",
            resource_type: "reservation",
            resource_id: reservation_id,
            reason: None,
            change: json!({ "action": action, "from": REQUESTED, "to": new_state }),
        },
    )
    .await?;
    events::reservation(&mut tx, actor.city, reservation_id, event).await?;
    tx.commit().await?;
    Ok(())
}
