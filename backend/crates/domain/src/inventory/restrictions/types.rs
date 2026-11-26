use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RestrictionOperation {
    pub action: String,
    pub restriction_id: Option<String>,
    pub kind: Option<String>,
    pub basis: Option<String>,
    pub unit_type_id: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub value: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BulkRestrictionInput {
    pub property_id: String,
    pub operations: Vec<RestrictionOperation>,
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RestrictionView {
    pub id: String,
    pub property_id: String,
    pub unit_type_id: Option<String>,
    pub kind: String,
    pub basis: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub value: Option<i32>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub removed_at: Option<DateTime<Utc>>,
}

pub const RESTRICTION_COLUMNS: &str = r#"id, "propertyId" AS property_id, "unitTypeId" AS unit_type_id, kind, basis,
    "startDate" AS start_date, "endDate" AS end_date, value, version,
    "createdAt" AS created_at, "removedAt" AS removed_at"#;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BulkRestrictionResult {
    pub applied: Vec<RestrictionView>,
    pub removed: Vec<String>,
    pub affected_nights: i64,
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct RestrictionListQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}
