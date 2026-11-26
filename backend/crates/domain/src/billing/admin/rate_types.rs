use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RateInput {
    pub rate: String,
    pub reason_code: String,
    pub note: Option<String>,
}

#[derive(Debug, FromRow, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RateRow {
    pub id: String,
    pub rate_bps: i32,
    pub effective_from: DateTime<Utc>,
    pub reason_code: String,
    pub note: Option<String>,
    pub changed_by_user_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CurrentRate {
    pub host_id: String,
    pub rate_bps: i32,
    pub configured: bool,
    pub effective_from: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RateSet {
    pub host_id: String,
    pub rate_bps: i32,
    pub id: String,
}
