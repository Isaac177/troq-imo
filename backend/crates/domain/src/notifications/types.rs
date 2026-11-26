use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotificationView {
    pub id: String,
    pub r#type: String,
    pub title: String,
    pub body: String,
    pub priority: String,
    pub deep_link: Option<String>,
    pub event_key: Option<String>,
    pub category: Option<String>,
    pub audience: Option<String>,
    pub params: Option<Value>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub read: bool,
    pub read_at: Option<DateTime<Utc>>,
    pub seen_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UnreadCount {
    pub unread: i64,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MarkedCount {
    pub updated: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PreferenceView {
    pub event_key: String,
    pub in_app: bool,
    pub push: bool,
    pub email: bool,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PreferenceList {
    pub items: Vec<PreferenceView>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushTokenInput {
    pub device_id: String,
    pub token: String,
    pub platform: String,
    pub app_version: Option<String>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PushTokenRemoval {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DocumentView {
    pub id: String,
    pub kind: String,
    pub locale: String,
    pub entity_type: String,
    pub entity_id: String,
    pub document_number: Option<String>,
    pub file_name: String,
    pub sha256_prefix: String,
    pub page_count: i32,
    pub bytes: i32,
    pub created_at: DateTime<Utc>,
}

