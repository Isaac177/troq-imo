use base64::Engine;
use chrono::{DateTime, Utc};
use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::files::clean_file_name;
use kernel::ids::new_id;
use kernel::{Error, Kernel, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{FromRow, PgPool, Row};
use utoipa::ToSchema;

use super::access::{can_manage, can_view};
use super::queries::find_case;
use super::rules::CASE_STATES;
use crate::reservations::codes::{
    CASE_NOT_FOUND, CASE_NOT_OPEN_FOR_EVIDENCE, EVIDENCE_CONTENT_MISMATCH, EVIDENCE_ENCODING_INVALID, EVIDENCE_NOT_FOUND,
    EVIDENCE_SIZE_INVALID, EVIDENCE_TYPE_INVALID,
};
use crate::reservations::events::{self, types::CASE_EVIDENCE_ADDED};

pub const MAX_EVIDENCE_BYTES: usize = 5 * 1024 * 1024;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddEvidenceInput {
    pub file_name: String,
    pub mime_type: String,
    pub content_base64: String,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceView {
    pub id: String,
    pub case_id: String,
    pub file_name: String,
    pub mime_type: String,
    pub size_bytes: i32,
    pub added_by_user_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceContent {
    pub evidence: EvidenceView,
    pub content_base64: String,
}

pub const EVIDENCE_COLUMNS: &str = r#"id, "caseId" AS case_id, "fileName" AS file_name, "mimeType" AS mime_type,
    "sizeBytes" AS size_bytes, "addedByUserId" AS added_by_user_id, "createdAt" AS created_at"#;

pub async fn evidence_for_case(pool: &PgPool, case_id: &str) -> Result<Vec<EvidenceView>> {
    let sql = format!(r#"SELECT {EVIDENCE_COLUMNS} FROM stay_case_evidence WHERE "caseId" = $1 ORDER BY "createdAt", id"#);
    Ok(sqlx::query_as::<_, EvidenceView>(&sql).bind(case_id).fetch_all(pool).await?)
}

async fn find_view(pool: &PgPool, evidence_id: &str) -> Result<Option<EvidenceView>> {
    let sql = format!("SELECT {EVIDENCE_COLUMNS} FROM stay_case_evidence WHERE id = $1");
    Ok(sqlx::query_as::<_, EvidenceView>(&sql).bind(evidence_id).fetch_optional(pool).await?)
}

pub async fn add_evidence(kernel: &Kernel, actor: &Actor, case_id: &str, input: AddEvidenceInput) -> Result<EvidenceView> {
    let pool = kernel.db.pool();
    let city = actor.city;
    let file_name = clean_file_name(&input.file_name)?;
    let content = base64::engine::general_purpose::STANDARD
        .decode(input.content_base64.trim())
        .map_err(|_| Error::bad_request(EVIDENCE_ENCODING_INVALID))?;
    check_content(&input.mime_type, &content)?;
    let case = find_case(pool, city, case_id).await?.ok_or_else(|| Error::not_found(CASE_NOT_FOUND))?;
    if !can_manage(actor, &case) {
        return Err(Error::not_found(CASE_NOT_FOUND));
    }
    if !matches!(case.state.as_str(), "OPEN" | "AWAITING_RESPONSE") || !CASE_STATES.contains(&case.state.as_str()) {
        return Err(Error::conflict(CASE_NOT_OPEN_FOR_EVIDENCE));
    }
    let evidence_id = new_id("evidence");
    let mut tx = pool.begin().await?;
    sqlx::query(
        r#"INSERT INTO stay_case_evidence (id, "caseId", "fileName", "mimeType", "sizeBytes", content, "addedByUserId")
           VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
    )
    .bind(&evidence_id)
    .bind(&case.id)
    .bind(&file_name)
    .bind(&input.mime_type)
    .bind(content.len() as i32)
    .bind(&content)
    .bind(actor.user_id().as_str())
    .execute(&mut *tx)
    .await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "case.evidence_added",
            resource_type: "case",
            resource_id: &case.id,
            reason: None,
            change: json!({ "evidenceId": evidence_id, "mimeType": input.mime_type, "sizeBytes": content.len() }),
        },
    )
    .await?;
    events::emit(
        &mut tx,
        city,
        "case",
        &case.id,
        CASE_EVIDENCE_ADDED,
        json!({ "caseId": case.id, "operatorId": case.operator_id, "evidenceId": evidence_id }),
    )
    .await?;
    tx.commit().await?;
    find_view(pool, &evidence_id).await?.ok_or_else(|| Error::internal("evidence not readable"))
}

pub async fn list_evidence(kernel: &Kernel, actor: &Actor, case_id: &str) -> Result<Vec<EvidenceView>> {
    let pool = kernel.db.pool();
    let case = find_case(pool, actor.city, case_id).await?.ok_or_else(|| Error::not_found(CASE_NOT_FOUND))?;
    if !can_view(actor, &case) {
        return Err(Error::not_found(CASE_NOT_FOUND));
    }
    evidence_for_case(pool, &case.id).await
}

pub async fn get_evidence(kernel: &Kernel, actor: &Actor, evidence_id: &str) -> Result<EvidenceContent> {
    let pool = kernel.db.pool();
    let row = sqlx::query(r#"SELECT "caseId" AS case_id, content FROM stay_case_evidence WHERE id = $1"#)
        .bind(evidence_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found(EVIDENCE_NOT_FOUND))?;
    let case_id: String = row.try_get("case_id")?;
    let content: Vec<u8> = row.try_get("content")?;
    let case = find_case(pool, actor.city, &case_id).await?.ok_or_else(|| Error::not_found(EVIDENCE_NOT_FOUND))?;
    if !can_view(actor, &case) {
        return Err(Error::not_found(CASE_NOT_FOUND));
    }
    let evidence = find_view(pool, evidence_id).await?.ok_or_else(|| Error::not_found(EVIDENCE_NOT_FOUND))?;
    Ok(EvidenceContent { evidence, content_base64: base64::engine::general_purpose::STANDARD.encode(content) })
}

pub fn check_content(mime_type: &str, content: &[u8]) -> Result<()> {
    if content.is_empty() || content.len() > MAX_EVIDENCE_BYTES {
        return Err(Error::bad_request(EVIDENCE_SIZE_INVALID).with_params(json!({ "maxBytes": MAX_EVIDENCE_BYTES })));
    }
    let matches = match mime_type {
        "application/pdf" => content.starts_with(b"%PDF"),
        "image/png" => content.starts_with(b"\x89PNG\r\n\x1a\n"),
        "image/jpeg" => content.starts_with(&[0xFF, 0xD8, 0xFF]),
        _ => return Err(Error::bad_request(EVIDENCE_TYPE_INVALID)),
    };
    if matches {
        Ok(())
    } else {
        Err(Error::bad_request(EVIDENCE_CONTENT_MISMATCH))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_content_that_matches_its_type() {
        assert!(check_content("application/pdf", b"%PDF-1.7 body").is_ok());
        assert!(check_content("image/png", b"\x89PNG\r\n\x1a\nrest").is_ok());
        assert!(check_content("image/jpeg", &[0xFF, 0xD8, 0xFF, 0xE0]).is_ok());
    }

    #[test]
    fn rejects_content_that_does_not_match_its_type() {
        assert!(check_content("application/pdf", b"MZ executable").is_err());
        assert!(check_content("image/png", b"%PDF-1.7").is_err());
        assert!(check_content("text/html", b"<html>").is_err());
    }

    #[test]
    fn rejects_empty_and_oversized_files() {
        assert!(check_content("application/pdf", b"").is_err());
        let mut big = b"%PDF".to_vec();
        big.resize(MAX_EVIDENCE_BYTES + 1, 0);
        assert!(check_content("application/pdf", &big).is_err());
    }

    #[test]
    fn file_names_must_be_plain() {
        assert_eq!(clean_file_name("  receipt.pdf ").unwrap(), "receipt.pdf");
        assert!(clean_file_name("../secret.pdf").is_err());
        assert!(clean_file_name("a\\b.png").is_err());
        assert!(clean_file_name("   ").is_err());
    }
}
