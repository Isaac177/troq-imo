use kernel::auth::Actor;
use kernel::permissions::names::PROPERTIES_MODERATE;
use kernel::{Error, Kernel, Result};

use super::types::{ClaimCore, ClaimDetail, DecisionRow, EvidenceRow, StatementRow};
use crate::reservations::codes::CLAIM_NOT_FOUND;

const CORE_SQL: &str = r#"SELECT c.id AS case_id, c.version, c.state AS case_state, c.reason, c."operatorId" AS operator_id,
        o.display_name AS operator_name, c."reservationId" AS reservation_id, r.reference AS reservation_reference,
        p."nameEn" AS property_name_en, p."nameFr" AS property_name_fr,
        (SELECT g."fullName" FROM stay_reservation_items i
           JOIN stay_reservation_guests g ON g."reservationItemId" = i.id AND g.role = 'PRIMARY'
          WHERE i."reservationId" = r.id ORDER BY i.id LIMIT 1) AS guest_name,
        (SELECT MIN(i."checkIn") FROM stay_reservation_items i WHERE i."reservationId" = r.id) AS check_in,
        (SELECT MAX(i."checkOut") FROM stay_reservation_items i WHERE i."reservationId" = r.id) AS check_out,
        r."totalAmount"::text AS stay_total, COALESCE(c."claimAmount", 0)::text AS claim_amount,
        c."createdAt" AS opened_at, c."deadlineAt" AS deadline_at
   FROM stay_cases c
   JOIN hosts o ON o.id = c."operatorId"
   LEFT JOIN stay_reservations r ON r.id = c."reservationId"
   LEFT JOIN stay_properties p ON p.id = r."propertyId"
   WHERE c.id = $1 AND c.city_id = $2 AND c."caseType" = 'DAMAGE_CLAIM'"#;

const EVIDENCE_SQL: &str = r#"SELECT id, "fileName" AS file_name, "mimeType" AS mime_type, "sizeBytes" AS size_bytes, "createdAt" AS created_at
   FROM stay_case_evidence WHERE "caseId" = $1 ORDER BY "createdAt", id"#;

const STATEMENTS_SQL: &str = r#"SELECT id, party, body, "createdAt" AS created_at
   FROM stay_claim_statements WHERE "caseId" = $1 ORDER BY "createdAt", id"#;

const DECISION_SQL: &str = r#"SELECT decision, "approvedAmount"::text AS approved_amount, "reasonCode" AS reason_code,
        "decidedAt" AS decided_at, "payoutState" AS payout_state, "payoutRef" AS payout_ref, "payoutUpdatedAt" AS payout_updated_at
   FROM stay_claim_decisions WHERE "caseId" = $1"#;

pub async fn get(kernel: &Kernel, actor: &Actor, case_id: &str) -> Result<ClaimDetail> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    let pool = kernel.db.pool();
    let core = sqlx::query_as::<_, ClaimCore>(CORE_SQL)
        .bind(case_id)
        .bind(actor.city.code())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found(CLAIM_NOT_FOUND))?;
    let evidence = sqlx::query_as::<_, EvidenceRow>(EVIDENCE_SQL).bind(case_id).fetch_all(pool).await?;
    let statements = sqlx::query_as::<_, StatementRow>(STATEMENTS_SQL).bind(case_id).fetch_all(pool).await?;
    let decision = sqlx::query_as::<_, DecisionRow>(DECISION_SQL).bind(case_id).fetch_optional(pool).await?;
    Ok(ClaimDetail { core, evidence, statements, decision })
}
