use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingInput {
    pub value: i32,
    pub reason_code: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SettingView {
    pub key: &'static str,
    pub value: i32,
    pub default_value: i32,
    pub min: i32,
    pub max: i32,
    pub unit: &'static str,
    pub changed: bool,
    pub updated_by_user_id: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SettingsView {
    pub city: String,
    pub items: Vec<SettingView>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdatedSetting {
    pub key: &'static str,
    pub value: i32,
}
