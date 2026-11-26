use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use utoipa::ToSchema;

pub const MAX_SAVED_SEARCHES_PER_GUEST: i64 = 20;
pub const MAX_PARAMETER_BYTES: usize = 2048;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateSavedSearch {
    pub parameters: Value,
    #[serde(default)]
    pub alert_consent: bool,
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SavedSearchView {
    pub id: String,
    pub parameters: Value,
    pub alert_consent: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SavedSearchRemoved {
    pub saved_search_id: String,
    pub removed_at: DateTime<Utc>,
}
