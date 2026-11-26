use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Default)]
pub struct GroupFilters {
    pub active: Option<bool>,
    pub search: Option<String>,
}

#[derive(Debug, Default)]
pub struct AmenityFilters {
    pub active: Option<bool>,
    pub group_code: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AmenityGroup {
    pub code: String,
    pub name: String,
    pub sort_order: i32,
    pub active: bool,
    pub revision: i64,
    pub amenity_count: i64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, sqlx::FromRow, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Amenity {
    pub code: String,
    pub group_code: String,
    pub name: String,
    pub icon: String,
    pub sort_order: i32,
    pub active: bool,
    pub revision: i64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupWrite {
    pub locale: String,
    pub name: String,
    pub sort_order: Option<i32>,
    pub active: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmenityWrite {
    pub locale: String,
    pub name: String,
    pub group_code: String,
    pub icon: String,
    pub sort_order: Option<i32>,
    pub active: Option<bool>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedCatalogItem {
    pub code: String,
}
