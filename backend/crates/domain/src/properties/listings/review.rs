use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::error::{Error, Result};
use kernel::outbox;
use serde_json::json;
use sqlx::PgPool;

use super::apply::apply_content;
use super::events::payload;
use super::guards::{ensure_not_archived, ensure_status, ensure_version};
use super::revision_view::load;
use super::types::{ListingDecisionInput, ModerationInput};
use super::{queries, readiness};
use crate::properties::messages::{
    DECISION_UNKNOWN, PROPERTY_NOT_FOUND, REASON_CODES_NOT_ALLOWED, REASON_CODES_REQUIRED, REASON_CODE_REQUIRED,
    REASON_CODE_UNKNOWN, REVISION_NOT_FOUND,
};
use crate::properties::property::queries as property_queries;
use crate::properties::property::types::PropertyView;
use crate::properties::revisions::content::ListingContent;
use crate::properties::revisions::queries as revision_queries;
use crate::properties::revisions::types::RevisionView;
use crate::properties::validation::{bounded, field_error};

pub const LISTING_REASON_CODES: [&str; 6] = [
    "CONTENT_INCOMPLETE",
    "MEDIA_UNSUITABLE",
    "DESCRIPTION_MISLEADING",
    "LOCATION_UNCLEAR",
    "POLICY_MISSING",
    "OTHER",
];
pub const SUSPENSION_REASON_CODES: [&str; 4] = ["SAFETY_CONCERN", "POLICY_VIOLATION", "OPERATOR_REQUEST", "OTHER"];
const MAX_GUIDANCE_LENGTH: usize = 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Decision {
    Approve,
    RequestChanges,
    Reject,
}

impl Decision {
    fn parse(value: &str) -> Result<Self> {
        match value {
            "APPROVE" => Ok(Self::Approve),
            "REQUEST_CHANGES" => Ok(Self::RequestChanges),
            "REJECT" => Ok(Self::Reject),
            _ => Err(Error::bad_request(DECISION_UNKNOWN)),
        }
    }
}

fn reason_codes(decision: Decision, codes: &[String]) -> Result<Vec<String>> {
    if decision == Decision::Approve {
        return if codes.is_empty() {
            Ok(Vec::new())
        } else {
            Err(Error::bad_request(REASON_CODES_NOT_ALLOWED))
        };
    }
    if codes.is_empty() {
        return Err(Error::bad_request(REASON_CODES_REQUIRED));
    }
    codes
        .iter()
        .map(|code| {
            if LISTING_REASON_CODES.contains(&code.as_str()) {
                Ok(code.clone())
            } else {
                Err(Error::bad_request(REASON_CODE_UNKNOWN).with_params(json!({ "code": code })))
            }
        })
        .collect()
}

fn guidance(value: Option<String>) -> Result<Option<String>> {
    match value.map(|text| text.trim().to_string()).filter(|text| !text.is_empty()) {
        Some(text) => bounded("guidance", &text, MAX_GUIDANCE_LENGTH).map(Some),
        None => Ok(None),
    }
}

pub async fn decide(pool: &PgPool, actor: &Actor, revision_id: &str, input: ListingDecisionInput) -> Result<RevisionView> {
    actor.ensure_writable()?;
    crate::properties::scope::ensure_moderator(actor)?;
    let decision = Decision::parse(&input.decision)?;
    let reasons = reason_codes(decision, &input.reason_codes)?;
    let guidance = guidance(input.guidance)?;
    let property_id = queries::revision_property_id(pool, revision_id)
        .await?
        .ok_or_else(|| Error::not_found(REVISION_NOT_FOUND))?;
    let mut tx = pool.begin().await?;
    let property = property_queries::lock(&mut tx, &property_id)
        .await?
        .ok_or_else(|| Error::not_found(PROPERTY_NOT_FOUND))?;
    actor.ensure_city(kernel::City::from_code(&property.city)?)?;
    let revision = revision_queries::lock(&mut tx, &property_id, revision_id)
        .await?
        .ok_or_else(|| Error::not_found(REVISION_NOT_FOUND))?;
    ensure_status(&revision.summary.review_status, &["SUBMITTED"], "decide on")?;
    ensure_version(revision.summary.version, input.expected_version)?;
    let status = match decision {
        Decision::Approve => {
            ensure_not_archived(&property)?;
            let content = ListingContent::from_snapshot(&revision.snapshot)?;
            readiness::ensure_ready(&mut tx, &property.operator_id, &property_id, &content).await?;
            apply_content(&mut tx, &property_id, &content, Some(revision.summary.revision_number)).await?;
            "APPROVED"
        }
        Decision::RequestChanges => "CHANGES_REQUESTED",
        Decision::Reject => "REJECTED",
    };
    queries::set_revision_decision(&mut tx, revision_id, status, actor.user_id().as_str(), &reasons, guidance.as_deref())
        .await?;
    let event = format!("property.revision.{}", status.to_lowercase());
    let data = payload(
        &mut tx,
        &property_id,
        Some(revision_id),
        json!({ "revisionNumber": revision.summary.revision_number, "reasonCodes": reasons }),
    )
    .await?;
    outbox::enqueue(&mut tx, actor.city, "property", &property_id, &event, data).await?;
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "listing.revision.decide",
        resource_type: "property_revision",
        resource_id: revision_id,
        reason: reasons.first().map(String::as_str),
        change: json!({ "decision": input.decision, "status": status, "propertyId": property_id, "reasonCodes": reasons }),
    })
    .await?;
    tx.commit().await?;
    load(pool, revision_id).await
}

pub async fn suspend(pool: &PgPool, actor: &Actor, property_id: &str, input: ModerationInput) -> Result<PropertyView> {
    actor.ensure_writable()?;
    crate::properties::scope::ensure_moderator(actor)?;
    let reason = input.reason_code.ok_or_else(|| field_error(REASON_CODE_REQUIRED, "reasonCode"))?;
    if !SUSPENSION_REASON_CODES.contains(&reason.as_str()) {
        return Err(Error::bad_request(REASON_CODE_UNKNOWN).with_params(json!({ "code": reason })));
    }
    let mut tx = pool.begin().await?;
    let property = property_queries::lock(&mut tx, property_id)
        .await?
        .ok_or_else(|| Error::not_found(PROPERTY_NOT_FOUND))?;
    actor.ensure_city(kernel::City::from_code(&property.city)?)?;
    ensure_version(property.version, input.expected_version)?;
    ensure_status(&property.status, &["APPROVED", "PUBLISHED", "PAUSED"], "suspend")?;
    queries::set_property_status(&mut tx, &property.id, "SUSPENDED").await?;
    let data = payload(&mut tx, &property.id, None, json!({ "reasonCode": reason })).await?;
    outbox::enqueue(&mut tx, actor.city, "property", &property.id, "property.suspended", data).await?;
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "listing.suspend",
        resource_type: "property",
        resource_id: &property.id,
        reason: Some(&reason),
        change: json!({ "from": property.status, "to": "SUSPENDED" }),
    })
    .await?;
    tx.commit().await?;
    property_queries::find(pool, &property.id).await?.ok_or_else(|| Error::not_found(PROPERTY_NOT_FOUND))
}

pub async fn reinstate(pool: &PgPool, actor: &Actor, property_id: &str, input: ModerationInput) -> Result<PropertyView> {
    actor.ensure_writable()?;
    crate::properties::scope::ensure_moderator(actor)?;
    let mut tx = pool.begin().await?;
    let property = property_queries::lock(&mut tx, property_id)
        .await?
        .ok_or_else(|| Error::not_found(PROPERTY_NOT_FOUND))?;
    actor.ensure_city(kernel::City::from_code(&property.city)?)?;
    ensure_version(property.version, input.expected_version)?;
    ensure_status(&property.status, &["SUSPENDED"], "reinstate")?;
    queries::set_property_status(&mut tx, &property.id, "APPROVED").await?;
    let data = payload(&mut tx, &property.id, None, json!({ "status": "APPROVED" })).await?;
    outbox::enqueue(&mut tx, actor.city, "property", &property.id, "property.reinstated", data).await?;
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "listing.reinstate",
        resource_type: "property",
        resource_id: &property.id,
        reason: None,
        change: json!({ "from": "SUSPENDED", "to": "APPROVED" }),
    })
    .await?;
    tx.commit().await?;
    property_queries::find(pool, &property.id).await?.ok_or_else(|| Error::not_found(PROPERTY_NOT_FOUND))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approval_takes_no_reasons_and_other_decisions_need_known_ones() {
        assert!(reason_codes(Decision::Approve, &[]).unwrap().is_empty());
        assert!(reason_codes(Decision::Approve, &["OTHER".into()]).is_err());
        assert!(reason_codes(Decision::Reject, &[]).is_err());
        assert!(reason_codes(Decision::RequestChanges, &["MADE_UP".into()]).is_err());
        assert_eq!(reason_codes(Decision::Reject, &["OTHER".into()]).unwrap(), vec!["OTHER"]);
    }

    #[test]
    fn decisions_parse_from_wire_values() {
        assert_eq!(Decision::parse("REQUEST_CHANGES").unwrap(), Decision::RequestChanges);
        assert!(Decision::parse("MAYBE").is_err());
    }

    #[test]
    fn guidance_is_trimmed_bounded_and_optional() {
        assert_eq!(guidance(Some("  fix ".into())).unwrap().as_deref(), Some("fix"));
        assert_eq!(guidance(Some("  ".into())).unwrap(), None);
        assert!(guidance(Some("x".repeat(MAX_GUIDANCE_LENGTH + 1))).is_err());
    }
}
