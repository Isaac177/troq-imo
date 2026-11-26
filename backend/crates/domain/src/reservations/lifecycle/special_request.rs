use kernel::audit::{self, AuditActor, AuditEntry};
use kernel::auth::Actor;
use kernel::{Error, Kernel, Result};
use serde_json::json;
use sqlx::PgPool;

use super::access::ensure_manage;
use super::decision_queries::find_for_decision;
use super::types::{ReservationView, SpecialRequestDecisionInput};
use super::view::require_view;
use crate::reservations::codes::{
    RESERVATION_NOT_FOUND, SPECIAL_REQUEST_DECISION_UNKNOWN, SPECIAL_REQUEST_NOT_PENDING,
};
use crate::reservations::events::{self, types::SPECIAL_REQUEST_DECIDED};

pub async fn decide_special_request(
    kernel: &Kernel,
    actor: &Actor,
    reservation_id: &str,
    input: SpecialRequestDecisionInput,
) -> Result<ReservationView> {
    let pool = kernel.db.pool();
    let row = find_for_decision(pool, actor.city, reservation_id)
        .await?
        .ok_or_else(|| Error::not_found(RESERVATION_NOT_FOUND))?;
    ensure_manage(actor, &row)?;
    apply_special_request(pool, &actor.audit(), reservation_id, &input).await?;
    require_view(kernel, actor.city, reservation_id).await
}

pub async fn apply_special_request(
    pool: &PgPool,
    actor: &AuditActor,
    reservation_id: &str,
    input: &SpecialRequestDecisionInput,
) -> Result<()> {
    let next = match input.decision.as_str() {
        "ACCEPT" => "ACCEPTED",
        "DECLINE" => "DECLINED",
        other => return Err(Error::bad_request(SPECIAL_REQUEST_DECISION_UNKNOWN).with_params(json!({ "decision": other }))),
    };
    let mut tx = pool.begin().await?;
    let moved = sqlx::query(
        r#"UPDATE stay_reservations
           SET "specialRequestState" = $3, version = version + 1, "updatedAt" = now()
           WHERE id = $1 AND version = $2 AND "specialRequestState" = 'REQUESTED'
             AND state IN ('REQUESTED', 'CONFIRMED')"#,
    )
    .bind(reservation_id)
    .bind(input.expected_version)
    .bind(next)
    .execute(&mut *tx)
    .await?;
    if moved.rows_affected() != 1 {
        return Err(Error::conflict(SPECIAL_REQUEST_NOT_PENDING));
    }
    audit::record(
        &mut tx,
        actor,
        AuditEntry {
            action: "reservation.special_request_decided",
            resource_type: "reservation",
            resource_id: reservation_id,
            reason: None,
            change: json!({ "from": "REQUESTED", "to": next }),
        },
    )
    .await?;
    events::emit(
        &mut tx,
        actor.city,
        "reservation",
        reservation_id,
        SPECIAL_REQUEST_DECIDED,
        json!({ "reservationId": reservation_id, "specialRequestState": next }),
    )
    .await?;
    tx.commit().await?;
    Ok(())
}
