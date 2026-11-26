use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub const APPLICATION_KINDS: &[&str] = &["INDIVIDUAL", "BUSINESS"];
pub const DOCUMENT_KINDS: &[&str] = &[
    "BUSINESS_REGISTRATION",
    "TAX_CERTIFICATE",
    "OWNER_IDENTITY",
    "PROPERTY_PROOF",
    "INSURANCE",
    "SAFETY_CERTIFICATE",
];
pub const APPROVE_REASONS: &[&str] = &["DOCUMENTS_VERIFIED", "IDENTITY_VERIFIED", "PROPERTY_VISITED"];
pub const REJECT_REASONS: &[&str] = &[
    "DOCUMENTS_INCOMPLETE",
    "IDENTITY_UNVERIFIED",
    "PROPERTY_UNVERIFIED",
    "POLICY_VIOLATION",
    "DUPLICATE_APPLICATION",
];
pub const SUSPEND_REASONS: &[&str] = &[
    "POLICY_VIOLATION",
    "FRAUD_SUSPECTED",
    "GUEST_SAFETY",
    "UNPAID_FEES",
    "DOCUMENTS_EXPIRED",
    "OWNER_REQUEST",
];
pub const REINSTATE_REASONS: &[&str] = &["ISSUE_RESOLVED", "APPEAL_ACCEPTED", "DOCUMENTS_UPDATED", "ERROR_CORRECTED"];
pub const DOCUMENT_REJECT_REASONS: &[&str] = &["UNREADABLE", "EXPIRED", "MISMATCH", "INCOMPLETE"];

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationInput {
    pub display_name: String,
    pub kind: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddDocumentInput {
    pub kind: String,
    pub file_name: String,
    pub mime_type: String,
    pub content_base64: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HostDecisionInput {
    pub decision: String,
    pub reason_code: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HostStatusInput {
    pub action: String,
    pub reason_code: String,
}

#[derive(Debug, Deserialize, Default, ToSchema)]
#[serde(rename_all = "camelCase", default)]
pub struct ApplicationFilter {
    pub status: Option<String>,
    pub search: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HostDocumentView {
    pub id: String,
    pub kind: String,
    pub file_name: String,
    pub mime_type: String,
    pub size_bytes: i32,
    pub status: String,
    pub reason_code: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationView {
    pub id: String,
    pub applicant_user_id: String,
    pub city_id: String,
    pub display_name: String,
    pub kind: String,
    pub status: String,
    pub reason_code: Option<String>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub decided_at: Option<DateTime<Utc>>,
    pub host_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub documents: Vec<HostDocumentView>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminApplicationView {
    pub id: String,
    pub applicant_user_id: String,
    pub applicant_name: Option<String>,
    pub applicant_email: Option<String>,
    pub city_id: String,
    pub display_name: String,
    pub kind: String,
    pub status: String,
    pub reason_code: Option<String>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub decided_at: Option<DateTime<Utc>>,
    pub host_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminHostView {
    pub id: String,
    pub city_id: String,
    pub display_name: String,
    pub kind: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub guest_contact_phone: String,
    pub guest_contact_email: String,
    pub member_count: i64,
    pub property_count: i64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OperationNotice {
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HostStatusView {
    pub host_id: String,
    pub status: String,
}

pub fn kind(value: &str) -> Option<&'static str> {
    let value = value.trim();
    APPLICATION_KINDS.iter().copied().find(|kind| kind.eq_ignore_ascii_case(value))
}

pub fn document_kind(value: &str) -> Option<&'static str> {
    let value = value.trim();
    DOCUMENT_KINDS.iter().copied().find(|kind| kind.eq_ignore_ascii_case(value))
}

pub fn reason(value: &str, allowed: &[&'static str]) -> Option<&'static str> {
    let value = value.trim();
    allowed.iter().copied().find(|candidate| candidate.eq_ignore_ascii_case(value))
}
