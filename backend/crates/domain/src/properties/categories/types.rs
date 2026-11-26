use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct CategoryListQuery {
    pub search: Option<String>,
    pub status: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CategoryInput {
    pub name_en: String,
    pub name_fr: String,
    pub icon: String,
    pub sort_order: Option<i32>,
    pub active: Option<bool>,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CategoryView {
    pub code: String,
    pub name_en: String,
    pub name_fr: String,
    pub icon: String,
    pub sort_order: i32,
    pub active: bool,
    pub revision: i64,
    pub unit_type_count: i64,
    pub updated_at: DateTime<Utc>,
}
