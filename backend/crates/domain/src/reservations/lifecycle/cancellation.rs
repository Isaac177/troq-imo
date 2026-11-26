use chrono::Utc;
use kernel::auth::Actor;
use kernel::{City, Error, Kernel, Result};
use serde_json::{json, Value};
use sqlx::{FromRow, PgPool};

use super::state::CONFIRMED;
use super::types::{CancelPreview, CancellationView};
use crate::reservations::codes::{
    CANNOT_CANCEL, NO_CANCELLATION_DEADLINE, NO_CANCELLATION_POLICY, RESERVATION_NOT_FOUND,
};
use crate::reservations::contract::{ContractSnapshot, PolicyEntry};
use crate::reservations::links::pricing::{evaluate, CancellationTerms};
use crate::reservations::support::{parse_minor, wire_minor};

#[derive(Debug, FromRow)]
pub struct CancellationSource {
    pub guest_user_id: String,
    pub state: String,
    pub version: i32,
    pub total_amount: String,
    pub contract: Value,
}

pub fn cancellation_terms(policies: &[PolicyEntry]) -> Result<CancellationTerms> {
    let policy = policies
        .iter()
        .find(|policy| policy.kind == "CANCELLATION")
        .ok_or_else(|| Error::conflict(NO_CANCELLATION_POLICY))?;
    CancellationTerms::from_content(&policy.content)
}

pub async fn load_source(pool: &PgPool, city: City, reservation_id: &str) -> Result<Option<CancellationSource>> {
    let row = sqlx::query_as::<_, CancellationSource>(
        r#"SELECT "guestUserId" AS guest_user_id, state, version, "totalAmount"::text AS total_amount,
                  "contractSnapshot" AS contract
           FROM stay_reservations WHERE id = $1 AND city_id = $2"#,
    )
    .bind(reservation_id)
    .bind(city.code())
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub fn read_contract(contract: Value) -> Result<ContractSnapshot> {
    serde_json::from_value(contract).map_err(|error| Error::internal(format!("reservation contract unreadable: {error}")))
}

pub async fn evaluate_cancellation(kernel: &Kernel, actor: &Actor, reservation_id: &str) -> Result<CancellationView> {
    let pool = kernel.db.pool();
    let session = actor.user_id().as_str();
    let source = load_source(pool, actor.city, reservation_id)
        .await?
        .filter(|source| source.guest_user_id == session)
        .ok_or_else(|| Error::not_found(RESERVATION_NOT_FOUND))?;
    if source.state != CONFIRMED {
        return Err(Error::conflict(CANNOT_CANCEL).with_params(json!({ "state": source.state })));
    }
    let snapshot = read_contract(source.contract)?;
    let terms = cancellation_terms(&snapshot.policies)?;
    let deadline = snapshot.cancellation_deadline.ok_or_else(|| Error::conflict(NO_CANCELLATION_DEADLINE))?;
    let paid_minor = parse_minor(&source.total_amount)?;
    let now = Utc::now();
    let evaluation = evaluate(terms, deadline, now, paid_minor);
    Ok(CancellationView {
        reservation_id: reservation_id.to_string(),
        currency: "CDF".into(),
        total: wire_minor(paid_minor),
        deadline,
        evaluated_at: now,
        reason: evaluation.reason.to_string(),
        refund: wire_minor(evaluation.refund_minor),
        penalty: wire_minor(evaluation.penalty_minor),
    })
}

pub async fn platform_preview(pool: &PgPool, city: City, reservation_id: &str) -> Result<CancelPreview> {
    let source = load_source(pool, city, reservation_id)
        .await?
        .ok_or_else(|| Error::not_found(RESERVATION_NOT_FOUND))?;
    let paid_minor = parse_minor(&source.total_amount)?;
    let mut view = CancelPreview {
        reservation_id: reservation_id.to_string(),
        can_cancel: source.state == CONFIRMED,
        state: source.state,
        version: source.version,
        total: wire_minor(paid_minor),
        refund: wire_minor(paid_minor),
        policy_reason: None,
        policy_refund: None,
        policy_penalty: None,
        deadline: None,
    };
    let Ok(snapshot) = read_contract(source.contract) else {
        return Ok(view);
    };
    let (Ok(terms), Some(deadline)) = (cancellation_terms(&snapshot.policies), snapshot.cancellation_deadline) else {
        return Ok(view);
    };
    let evaluation = evaluate(terms, deadline, Utc::now(), paid_minor);
    view.deadline = Some(deadline);
    view.policy_reason = Some(evaluation.reason.to_string());
    view.policy_refund = Some(wire_minor(evaluation.refund_minor));
    view.policy_penalty = Some(wire_minor(evaluation.penalty_minor));
    Ok(view)
}
