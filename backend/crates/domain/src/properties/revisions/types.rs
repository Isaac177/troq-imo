use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{IntoParams, ToSchema};

use super::content::ListingContent;

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct RevisionListQuery { pub page: Option<u32>, pub page_size: Option<u32> }

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateRevision { pub content: Option<ListingContent> }

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRevision { pub expected_version: i32, pub content: ListingContent }

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RevisionAction { pub expected_version: i32 }

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RevisionSummary {
    pub id: String,
    pub property_id: String,
    pub revision_number: i32,
    pub review_status: String,
    pub version: i32,
    pub author_user_id: String,
    pub submitted_at: Option<DateTime<Utc>>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub review_reasons: Value,
    pub review_guidance: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RevisionView {
    #[serde(flatten)]
    pub summary: RevisionSummary,
    pub content: ListingContent,
}

#[derive(sqlx::FromRow)]
pub struct RevisionRow {
    #[sqlx(flatten)]
    pub summary: RevisionSummary,
    pub snapshot: Value,
}
