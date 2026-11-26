use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Default, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase", default)]
#[into_params(parameter_in = Query)]
pub struct ClaimFilter {
    pub status: Option<String>,
    pub host_id: Option<String>,
    pub search: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StatementInput {
    pub party: String,
    pub body: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DecideInput {
    pub expected_version: i32,
    pub decision: String,
    pub approved_amount: Option<String>,
    pub reason_code: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PayoutInput {
    pub payout_state: String,
    pub payout_ref: Option<String>,
}

#[derive(Debug, sqlx::FromRow, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClaimRow {
    pub case_id: String,
    pub reservation_reference: Option<String>,
    pub operator_id: String,
    pub operator_name: String,
    pub property_name_en: Option<String>,
    pub property_name_fr: Option<String>,
    pub guest_name: Option<String>,
    pub check_out: Option<NaiveDate>,
    pub claim_amount: String,
    pub case_state: String,
    pub decision: Option<String>,
    pub approved_amount: Option<String>,
    pub payout_state: Option<String>,
    pub opened_at: DateTime<Utc>,
    pub deadline_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClaimSummary {
    pub total: i64,
    pub undecided: i64,
    pub claimed: String,
    pub approved: String,
    pub payout_pending: String,
    pub paid: String,
}

#[derive(Debug, sqlx::FromRow, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClaimCore {
    pub case_id: String,
    pub version: i32,
    pub case_state: String,
    pub reason: String,
    pub operator_id: String,
    pub operator_name: String,
    pub reservation_id: Option<String>,
    pub reservation_reference: Option<String>,
    pub property_name_en: Option<String>,
    pub property_name_fr: Option<String>,
    pub guest_name: Option<String>,
    pub check_in: Option<NaiveDate>,
    pub check_out: Option<NaiveDate>,
    pub stay_total: Option<String>,
    pub claim_amount: String,
    pub opened_at: DateTime<Utc>,
    pub deadline_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceRow {
    pub id: String,
    pub file_name: String,
    pub mime_type: String,
    pub size_bytes: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StatementRow {
    pub id: String,
    pub party: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DecisionRow {
    pub decision: String,
    pub approved_amount: String,
    pub reason_code: String,
    pub decided_at: DateTime<Utc>,
    pub payout_state: String,
    pub payout_ref: Option<String>,
    pub payout_updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClaimDetail {
    #[serde(flatten)]
    pub core: ClaimCore,
    pub evidence: Vec<EvidenceRow>,
    pub statements: Vec<StatementRow>,
    pub decision: Option<DecisionRow>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClaimDecisionResult {
    pub case_id: String,
    pub decision: String,
    pub approved_amount: String,
    pub payout_state: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PayoutResult {
    pub case_id: String,
    pub payout_state: String,
}
