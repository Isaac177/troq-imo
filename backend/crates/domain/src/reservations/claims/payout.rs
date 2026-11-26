use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::permissions::names::PROPERTIES_MODERATE;
use kernel::{Error, Kernel, Result};
use serde_json::json;

use super::reasons::ensure_payout_state;
use super::types::{PayoutInput, PayoutResult};
use crate::reservations::codes::{
    CLAIM_DECISION_NOT_FOUND, PAYOUT_ALREADY_PAID, PAYOUT_NOT_DUE, PAYOUT_REF_REQUIRED, PAYOUT_REF_TOO_LONG,
};

const MAX_REF_CHARS: usize = 120;

pub async fn update(kernel: &Kernel, actor: &Actor, case_id: &str, input: PayoutInput) -> Result<PayoutResult> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    let pool = kernel.db.pool();
    ensure_payout_state(&input.payout_state)?;
    let reference = input.payout_ref.as_deref().map(str::trim).filter(|value| !value.is_empty());
    if input.payout_state == "PAID" && reference.is_none() {
        return Err(Error::bad_request(PAYOUT_REF_REQUIRED));
    }
    if reference.is_some_and(|value| value.chars().count() > MAX_REF_CHARS) {
        return Err(Error::bad_request(PAYOUT_REF_TOO_LONG).with_params(json!({ "max": MAX_REF_CHARS })));
    }
    let previous: String = sqlx::query_scalar(
        r#"SELECT d."payoutState" FROM stay_claim_decisions d JOIN stay_cases c ON c.id = d."caseId"
           WHERE d."caseId" = $1 AND c.city_id = $2"#,
    )
    .bind(case_id)
    .bind(actor.city.code())
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| Error::not_found(CLAIM_DECISION_NOT_FOUND))?;
    if previous == "NOT_DUE" {
        return Err(Error::conflict(PAYOUT_NOT_DUE));
    }
    if previous == "PAID" {
        return Err(Error::conflict(PAYOUT_ALREADY_PAID));
    }
    let mut tx = pool.begin().await?;
    sqlx::query(
        r#"UPDATE stay_claim_decisions
           SET "payoutState" = $2, "payoutRef" = COALESCE($3, "payoutRef"), "payoutUpdatedAt" = now()
           WHERE "caseId" = $1"#,
    )
    .bind(case_id)
    .bind(&input.payout_state)
    .bind(reference)
    .execute(&mut *tx)
    .await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "claim.payout_updated",
            resource_type: "case",
            resource_id: case_id,
            reason: Some(input.payout_state.as_str()),
            change: json!({ "from": previous, "to": input.payout_state }),
        },
    )
    .await?;
    tx.commit().await?;
    Ok(PayoutResult { case_id: case_id.to_string(), payout_state: input.payout_state })
}
