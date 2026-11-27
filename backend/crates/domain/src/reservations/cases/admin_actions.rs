use base64::Engine;
use chrono::{Duration, Utc};
use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::ids::new_id;
use kernel::permissions::names::PROPERTIES_MODERATE;
use kernel::{Error, Kernel, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::Row;
use utoipa::ToSchema;

use super::admin::{find_item, AdminCaseItem};
use super::queries::{self, NewCase};
use super::rules::{ensure_closable, validate_reason, PLATFORM_CASE_TYPES};
use crate::reservations::codes::{
    CASE_ALREADY_ESCALATED, CASE_ASSIGNEE_INVALID, CASE_TYPE_NOT_ALLOWED, CASE_VERSION_CONFLICT, EVIDENCE_NOT_FOUND, RESERVATION_NOT_FOUND,
};
use crate::reservations::events::{self, types::{CASE_ESCALATED, CASE_OPENED}};
use crate::reservations::links::settings;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AssignCaseInput {
    #[serde(default)]
    pub assignee_user_id: Option<String>,
    pub expected_version: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EscalateCaseInput {
    pub reason: String,
    pub expected_version: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminOpenCaseInput {
    pub reservation_id: String,
    pub case_type: String,
    pub reason: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminEvidenceContent {
    pub file_name: String,
    pub mime_type: String,
    pub content_base64: String,
}

pub async fn assign_case(kernel: &Kernel, actor: &Actor, case_id: &str, input: AssignCaseInput) -> Result<AdminCaseItem> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    let pool = kernel.db.pool();
    let assignee = input.assignee_user_id.as_deref().map(str::trim).filter(|value| !value.is_empty());
    if let Some(user_id) = assignee {
        if !super::assignees::eligible(pool, user_id).await? {
            return Err(Error::bad_request(CASE_ASSIGNEE_INVALID));
        }
    }
    let case = find_item(pool, actor.city, case_id).await?;
    ensure_closable(&case.state)?;
    let mut tx = pool.begin().await?;
    let updated = sqlx::query(
        r#"UPDATE stay_cases
           SET "assignedToUserId" = $3,
               "assignedAt" = CASE WHEN $3::text IS NULL THEN NULL ELSE now() END,
               version = version + 1, "updatedAt" = now()
           WHERE id = $1 AND version = $2 AND state IN ('OPEN', 'AWAITING_RESPONSE')"#,
    )
    .bind(&case.id)
    .bind(input.expected_version)
    .bind(assignee)
    .execute(&mut *tx)
    .await?;
    if updated.rows_affected() == 0 {
        return Err(Error::conflict(CASE_VERSION_CONFLICT));
    }
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "case.assign",
            resource_type: "case",
            resource_id: &case.id,
            reason: None,
            change: json!({ "from": case.assigned_to_user_id, "to": assignee }),
        },
    )
    .await?;
    tx.commit().await?;
    find_item(pool, actor.city, &case.id).await
}

pub async fn escalate_case(kernel: &Kernel, actor: &Actor, case_id: &str, input: EscalateCaseInput) -> Result<AdminCaseItem> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    let pool = kernel.db.pool();
    let reason = validate_reason(&input.reason)?;
    let case = find_item(pool, actor.city, case_id).await?;
    ensure_closable(&case.state)?;
    if case.escalated_at.is_some() {
        return Err(Error::conflict(CASE_ALREADY_ESCALATED));
    }
    let mut tx = pool.begin().await?;
    let updated = sqlx::query(
        r#"UPDATE stay_cases
           SET "escalatedAt" = now(), "escalatedByUserId" = $3, "escalationReason" = $4,
               version = version + 1, "updatedAt" = now()
           WHERE id = $1 AND version = $2 AND "escalatedAt" IS NULL
             AND state IN ('OPEN', 'AWAITING_RESPONSE')"#,
    )
    .bind(&case.id)
    .bind(input.expected_version)
    .bind(actor.user_id().as_str())
    .bind(&reason)
    .execute(&mut *tx)
    .await?;
    if updated.rows_affected() == 0 {
        return Err(Error::conflict(CASE_VERSION_CONFLICT));
    }
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "case.escalate",
            resource_type: "case",
            resource_id: &case.id,
            reason: Some(&reason),
            change: json!({ "escalated": true }),
        },
    )
    .await?;
    events::emit(&mut tx, actor.city, "case", &case.id, CASE_ESCALATED, json!({ "caseId": case.id, "operatorId": case.operator_id })).await?;
    tx.commit().await?;
    find_item(pool, actor.city, &case.id).await
}

pub async fn open_case(kernel: &Kernel, actor: &Actor, input: AdminOpenCaseInput) -> Result<AdminCaseItem> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    if !PLATFORM_CASE_TYPES.contains(&input.case_type.as_str()) {
        return Err(Error::bad_request(CASE_TYPE_NOT_ALLOWED).with_params(json!({ "caseType": input.case_type })));
    }
    let pool = kernel.db.pool();
    let city = actor.city;
    let reason = validate_reason(&input.reason)?;
    let reservation = queries::reservation_for_case(pool, city, &input.reservation_id)
        .await?
        .ok_or_else(|| Error::not_found(RESERVATION_NOT_FOUND))?;
    let case_id = new_id("case");
    let parties = json!({
        "guestUserId": reservation.guest_user_id,
        "openedBy": "PLATFORM",
        "openedByUserId": actor.user_id().as_str(),
    });
    let deadline = Utc::now() + Duration::days(settings::case_response_days(pool, city).await?);
    let mut tx = pool.begin().await?;
    queries::insert_case(
        &mut tx,
        &NewCase {
            id: &case_id,
            city,
            operator_id: &reservation.operator_id,
            reservation_id: Some(&input.reservation_id),
            case_type: &input.case_type,
            reason: &reason,
            parties: &parties,
            deadline_at: deadline,
        },
    )
    .await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "case.open",
            resource_type: "case",
            resource_id: &case_id,
            reason: Some(&reason),
            change: json!({ "caseType": input.case_type, "operatorId": reservation.operator_id, "byPlatform": true }),
        },
    )
    .await?;
    events::emit(
        &mut tx,
        city,
        "case",
        &case_id,
        CASE_OPENED,
        json!({ "caseId": case_id, "operatorId": reservation.operator_id, "caseType": input.case_type }),
    )
    .await?;
    tx.commit().await?;
    find_item(pool, city, &case_id).await
}

pub async fn get_evidence(kernel: &Kernel, actor: &Actor, evidence_id: &str) -> Result<AdminEvidenceContent> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    let row = sqlx::query(
        r#"SELECT e."fileName" AS file_name, e."mimeType" AS mime_type, e.content
           FROM stay_case_evidence e JOIN stay_cases c ON c.id = e."caseId"
           WHERE e.id = $1 AND c.city_id = $2"#,
    )
    .bind(evidence_id)
    .bind(actor.city.code())
    .fetch_optional(kernel.db.pool())
    .await?
    .ok_or_else(|| Error::not_found(EVIDENCE_NOT_FOUND))?;
    let content: Vec<u8> = row.try_get("content")?;
    Ok(AdminEvidenceContent {
        file_name: row.try_get("file_name")?,
        mime_type: row.try_get("mime_type")?,
        content_base64: base64::engine::general_purpose::STANDARD.encode(content),
    })
}
