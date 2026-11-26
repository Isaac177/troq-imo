use chrono::Utc;
use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::{Error, Kernel, Result};
use serde_json::json;
use sqlx::FromRow;

use super::state::{CANCELLED, PENDING_PAYMENT, REQUESTED};
use super::types::{ReservationView, VersionedAction};
use super::view::require_view;
use crate::reservations::codes::{
    APPROVAL_WINDOW_CLOSED, CANNOT_WITHDRAW, RESERVATION_NOT_FOUND, RESERVATION_VERSION_CONFLICT,
};
use crate::reservations::events::{self, types::CANCELLED as CANCELLED_EVENT};
use crate::reservations::holds::release::release_holds;

const WITHDRAWN_CODE: &str = "GUEST_WITHDRAWN";

#[derive(Debug, FromRow)]
struct WithdrawSource {
    guest_user_id: String,
    state: String,
    approval_deadline_at: Option<chrono::DateTime<Utc>>,
}

pub async fn withdraw_request(kernel: &Kernel, actor: &Actor, reservation_id: &str, input: VersionedAction) -> Result<ReservationView> {
    let pool = kernel.db.pool();
    let session = actor.user_id().as_str();
    let source = sqlx::query_as::<_, WithdrawSource>(
        r#"SELECT "guestUserId" AS guest_user_id, state, "approvalDeadlineAt" AS approval_deadline_at
           FROM stay_reservations WHERE id = $1 AND city_id = $2"#,
    )
    .bind(reservation_id)
    .bind(actor.city.code())
    .fetch_optional(pool)
    .await?
    .filter(|source| source.guest_user_id == session)
    .ok_or_else(|| Error::not_found(RESERVATION_NOT_FOUND))?;
    if source.state != REQUESTED && source.state != PENDING_PAYMENT {
        return Err(Error::conflict(CANNOT_WITHDRAW).with_params(json!({ "state": source.state })));
    }
    if source.state == REQUESTED && source.approval_deadline_at.is_some_and(|deadline| deadline <= Utc::now()) {
        return Err(Error::conflict(APPROVAL_WINDOW_CLOSED));
    }
    let mut tx = pool.begin().await?;
    let moved = sqlx::query(
        r#"UPDATE stay_reservations
           SET state = 'CANCELLED', "approvalDeadlineAt" = NULL, version = version + 1, "updatedAt" = now(),
               "cancelledAt" = now(), "cancellationReason" = $3
           WHERE id = $1 AND version = $2 AND state IN ('REQUESTED', 'PENDING_PAYMENT')"#,
    )
    .bind(reservation_id)
    .bind(input.expected_version)
    .bind(WITHDRAWN_CODE)
    .execute(&mut *tx)
    .await?;
    if moved.rows_affected() != 1 {
        return Err(Error::conflict(RESERVATION_VERSION_CONFLICT));
    }
    release_holds(&mut tx, reservation_id).await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "reservation.withdrawn",
            resource_type: "reservation",
            resource_id: reservation_id,
            reason: Some(WITHDRAWN_CODE),
            change: json!({ "from": source.state, "to": CANCELLED }),
        },
    )
    .await?;
    events::reservation(&mut tx, actor.city, reservation_id, CANCELLED_EVENT).await?;
    tx.commit().await?;
    require_view(kernel, actor.city, reservation_id).await
}
