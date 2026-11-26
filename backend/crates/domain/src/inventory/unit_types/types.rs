use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};

fn default_bed_count() -> i32 {
    1
}

fn default_bathroom_count() -> i32 {
    1
}

fn default_sellable_quantity() -> i32 {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ImageView {
    pub id: String,
    pub url: String,
    pub alt_en: String,
    pub alt_fr: String,
    pub is_cover: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UnitTypeView {
    pub id: String,
    pub inventory_model: String,
    pub category_code: String,
    pub template_id: Option<String>,
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
    pub sellable_quantity: i32,
    pub base_nightly_amount: Option<String>,
    pub status: String,
    pub version: i32,
    #[sqlx(skip)]
    #[serde(default)]
    pub images: Vec<ImageView>,
    #[sqlx(skip)]
    #[serde(default)]
    pub cover_url: Option<String>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct UnitTypeListQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateUnitTypeInput {
    pub idempotency_key: Option<String>,
    pub inventory_model: String,
    pub category_code: String,
    #[serde(default)]
    pub template_id: Option<String>,
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
    #[serde(default = "default_bed_count")]
    pub bed_count: i32,
    #[serde(default = "default_bathroom_count")]
    pub bathroom_count: i32,
    #[serde(default)]
    pub accessibility: Vec<String>,
    #[serde(default = "default_sellable_quantity")]
    pub sellable_quantity: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUnitTypeInput {
    pub idempotency_key: Option<String>,
    pub expected_version: i32,
    pub category_code: String,
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
    #[serde(default = "default_bed_count")]
    pub bed_count: i32,
    #[serde(default = "default_bathroom_count")]
    pub bathroom_count: i32,
    #[serde(default)]
    pub accessibility: Vec<String>,
    pub sellable_quantity: i32,
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct ArchiveUnitTypeQuery {
    #[serde(default)]
    pub expected_version: i32,
}
