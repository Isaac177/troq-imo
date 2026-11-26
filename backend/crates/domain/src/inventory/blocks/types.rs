use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBlockInput {
    pub property_id: String,
    pub unit_type_id: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub reason: String,
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RemoveBlockInput {
    pub expected_version: i32,
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct BlockListQuery {
    pub property_id: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockView {
    pub id: String,
    pub property_id: String,
    pub unit_type_id: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub reason: String,
    pub origin: String,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub removed_at: Option<DateTime<Utc>>,
}

pub const BLOCK_COLUMNS: &str = r#"id, "propertyId" AS property_id, "unitTypeId" AS unit_type_id,
    "startDate" AS start_date, "endDate" AS end_date, reason, origin, version,
    "createdAt" AS created_at, "removedAt" AS removed_at"#;
