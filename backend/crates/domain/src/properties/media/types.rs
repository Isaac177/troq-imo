use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

use super::kind::MediaKind;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddMediaInput {
    pub client_key: String,
    pub file_name: String,
    pub mime_type: String,
    pub content_base64: String,
    #[serde(default)]
    pub alt_en: String,
    #[serde(default)]
    pub alt_fr: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMediaInput {
    pub expected_version: i32,
    pub alt_en: Option<String>,
    pub alt_fr: Option<String>,
    pub is_cover: Option<bool>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RemoveMediaInput {
    pub expected_version: i32,
}

#[derive(Debug, Clone, Serialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MediaView {
    pub id: String,
    pub owner_id: String,
    pub status: String,
    pub sort_order: i32,
    pub is_cover: bool,
    pub alt_en: String,
    pub alt_fr: String,
    pub file_name: String,
    pub mime_type: String,
    pub size_bytes: i32,
    pub rejection_reason: Option<String>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MediaContent {
    pub media: MediaView,
    pub url: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MediaRemoved {
    pub media_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Default)]
pub struct MediaChanges {
    pub alt_en: Option<String>,
    pub alt_fr: Option<String>,
    pub is_cover: Option<bool>,
    pub sort_order: Option<i32>,
    pub remove: bool,
}

pub fn media_columns(kind: MediaKind) -> String {
    format!(
        r#"id, {} AS owner_id, status, "sortOrder" AS sort_order, "isCover" AS is_cover, "altEn" AS alt_en,
    "altFr" AS alt_fr, "fileName" AS file_name, "mimeType" AS mime_type, "sizeBytes" AS size_bytes,
    "rejectionReason" AS rejection_reason, version, "createdAt" AS created_at, "updatedAt" AS updated_at"#,
        kind.owner_column()
    )
}
