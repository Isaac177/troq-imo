use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

fn one() -> i32 {
    1
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct TemplateListQuery {
    pub host_id: Option<String>,
    pub search: Option<String>,
    pub status: Option<String>,
    pub category_code: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TemplateInput {
    pub host_id: String,
    pub category_code: String,
    pub inventory_model: String,
    pub name_en: String,
    pub name_fr: String,
    #[serde(default)]
    pub description_en: String,
    #[serde(default)]
    pub description_fr: String,
    pub max_adults: i32,
    #[serde(default)]
    pub max_children: i32,
    #[serde(default)]
    pub bedroom_count: i32,
    #[serde(default = "one")]
    pub bed_count: i32,
    #[serde(default = "one")]
    pub bathroom_count: i32,
    #[serde(default)]
    pub accessibility: Vec<String>,
    pub base_nightly_amount: String,
    pub status: Option<String>,
    pub expected_version: Option<i32>,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TemplateView {
    pub id: String,
    #[serde(rename = "hostId")]
    pub operator_id: String,
    pub city_id: String,
    pub category_code: String,
    pub inventory_model: String,
    pub name_en: String,
    pub name_fr: String,
    pub description_en: String,
    pub description_fr: String,
    pub max_adults: i32,
    pub max_children: i32,
    pub bedroom_count: i32,
    pub bed_count: i32,
    pub bathroom_count: i32,
    pub accessibility: serde_json::Value,
    pub base_nightly_amount: String,
    pub status: String,
    pub version: i32,
    pub unit_type_count: i64,
    pub image_count: i64,
    #[sqlx(skip)]
    pub cover_url: Option<String>,
    pub updated_at: DateTime<Utc>,
}
