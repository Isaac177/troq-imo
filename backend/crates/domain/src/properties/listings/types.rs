use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::properties::property::types::PropertyView;
use crate::properties::revisions::content::ListingContent;
use crate::properties::revisions::types::RevisionView;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LifecycleInput {
    pub action: String,
    pub expected_version: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DecisionInput {
    pub decision: String,
    #[serde(default)]
    pub reason_codes: Vec<String>,
    pub guidance: Option<String>,
    pub expected_version: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ModerationInput {
    pub expected_version: i32,
    pub reason_code: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminUpdateInput {
    pub expected_version: i32,
    pub content: ListingContent,
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct QueueQuery {
    pub status: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct AdminPropertyQuery {
    pub status: Option<String>,
    pub search: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReadinessView {
    pub ready: bool,
    pub blockers: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminRevisionView {
    pub revision: RevisionView,
    pub live: ListingContent,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ReadinessFacts {
    pub has_sellable_unit: bool,
    pub has_base_rate: bool,
    pub has_cover_photo: bool,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PropertyFacts {
    pub cover_media_id: Option<String>,
    pub cover_media_version: Option<i32>,
    pub cover_media_mime: Option<String>,
    pub photo_count: i64,
    pub unit_type_count: i64,
    pub unit_count: i64,
    pub from_amount: Option<String>,
    pub open_reservations: i64,
    pub pending_requests: i64,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct QueueItem {
    pub revision_id: String,
    pub property_id: String,
    pub property_name_en: String,
    pub host_id: String,
    pub host_name: String,
    pub city: String,
    pub neighborhood: String,
    pub revision_number: i32,
    pub review_status: String,
    pub submitted_at: Option<DateTime<Utc>>,
    pub is_first_submission: bool,
    pub changed_fields: i64,
    #[sqlx(flatten)]
    #[serde(flatten)]
    pub facts: PropertyFacts,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminPropertyItem {
    pub id: String,
    pub name_en: String,
    pub name_fr: String,
    pub host_id: String,
    pub host_name: String,
    pub city: String,
    pub neighborhood: String,
    pub status: String,
    pub version: i32,
    pub address_line: String,
    pub host_status: String,
    pub published_revision: i32,
    pub open_revisions: i64,
    pub updated_at: DateTime<Utc>,
    #[sqlx(flatten)]
    #[serde(flatten)]
    pub facts: PropertyFacts,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminPropertyDetail {
    #[serde(flatten)]
    pub property: PropertyView,
    pub host_name: String,
    pub host_status: String,
}
