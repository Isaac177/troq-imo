use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::permissions::names::PROPERTIES_MODERATE;
use kernel::{Error, Kernel, Money, Result};
use serde_json::json;

use super::reasons::{approved_minor, ensure_reason};
use super::types::{ClaimDecisionResult, DecideInput};
use crate::reservations::codes::{CLAIM_ALREADY_DECIDED, CLAIM_NOT_FOUND, CLAIM_VERSION_CONFLICT};
use crate::reservations::events::{self, types::CLAIM_DECIDED};

pub async fn decide(kernel: &Kernel, actor: &Actor, case_id: &str, input: DecideInput) -> Result<ClaimDecisionResult> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    let pool = kernel.db.pool();
    ensure_reason(&input.decision, &input.reason_code)?;
    let case: Option<(String, Option<String>, String)> = sqlx::query_as(
        r#"SELECT state, "claimAmount"::text, "operatorId" FROM stay_cases
           WHERE id = $1 AND city_id = $2 AND "caseType" = 'DAMAGE_CLAIM'"#,
    )
    .bind(case_id)
    .bind(actor.city.code())
    .fetch_optional(pool)
    .await?;
    let (state, claim, host_id) = case.ok_or_else(|| Error::not_found(CLAIM_NOT_FOUND))?;
    if state != "OPEN" && state != "AWAITING_RESPONSE" {
        return Err(Error::conflict(CLAIM_ALREADY_DECIDED));
    }
    let claim_minor = Money::parse(claim.as_deref().unwrap_or("0"))?.minor();
    let requested = input.approved_amount.as_deref().map(Money::parse_input).transpose()?.map(Money::minor);
    let approved = approved_minor(&input.decision, claim_minor, requested)?;
    let approved_text = Money::from_minor(approved).to_wire();
    let payout = if input.decision == "REJECTED" { "NOT_DUE" } else { "PENDING" };

    let mut tx = pool.begin().await?;
    let closed = sqlx::query(
        r#"UPDATE stay_cases
           SET state = 'RESOLVED', resolution = $3::jsonb, version = version + 1, "closedAt" = now(), "updatedAt" = now()
           WHERE id = $1 AND version = $2 AND state IN ('OPEN', 'AWAITING_RESPONSE')"#,
    )
    .bind(case_id)
    .bind(input.expected_version)
    .bind(json!({ "decision": input.decision, "approvedAmount": approved_text, "reasonCode": input.reason_code }).to_string())
    .execute(&mut *tx)
    .await?;
    if closed.rows_affected() != 1 {
        return Err(Error::conflict(CLAIM_VERSION_CONFLICT));
    }
    sqlx::query(
        r#"INSERT INTO stay_claim_decisions ("caseId", decision, "approvedAmount", "reasonCode", "decidedByUserId", "payoutState")
           VALUES ($1, $2, $3::numeric, $4, $5, $6)"#,
    )
    .bind(case_id)
    .bind(&input.decision)
    .bind(&approved_text)
    .bind(&input.reason_code)
    .bind(actor.user_id().as_str())
    .bind(payout)
    .execute(&mut *tx)
    .await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "claim.decided",
            resource_type: "case",
            resource_id: case_id,
            reason: Some(input.reason_code.as_str()),
            change: json!({ "decision": input.decision, "approvedAmount": approved_text, "payoutState": payout }),
        },
    )
    .await?;
    events::emit(
        &mut tx,
        actor.city,
        "case",
        case_id,
        CLAIM_DECIDED,
        json!({ "caseId": case_id, "operatorId": host_id, "decision": input.decision, "approvedAmount": approved_text }),
    )
    .await?;
    tx.commit().await?;
    Ok(ClaimDecisionResult {
        case_id: case_id.to_string(),
        decision: input.decision,
        approved_amount: approved_text,
        payout_state: payout.to_string(),
    })
}
