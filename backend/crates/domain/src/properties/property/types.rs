use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub const PROPERTY_STATUSES: [&str; 6] = ["DRAFT", "APPROVED", "PUBLISHED", "PAUSED", "SUSPENDED", "ARCHIVED"];

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct PropertyListQuery {
    pub status: Option<String>,
    pub host_id: Option<String>,
    pub search: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreatePropertyInput {
    pub host_id: String,
    pub timezone: String,
    pub name_en: String,
    pub name_fr: String,
    #[serde(default)]
    pub description_en: String,
    #[serde(default)]
    pub description_fr: String,
    #[serde(default)]
    pub address_line: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PropertyView {
    pub id: String,
    #[serde(rename = "hostId")]
    pub operator_id: String,
    pub city: String,
    pub timezone: String,
    pub status: String,
    pub published_revision: i32,
    pub name_en: String,
    pub name_fr: String,
    pub description_en: String,
    pub description_fr: String,
    pub address_line: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub neighborhood: String,
    pub check_in_from: Option<String>,
    pub check_in_until: Option<String>,
    pub check_out_by: Option<String>,
    pub house_rules_en: String,
    pub house_rules_fr: String,
    pub pets_policy: Option<String>,
    pub smoking_policy: Option<String>,
    pub guest_contact_phone: String,
    pub guest_contact_email: String,
    pub effective_guest_contact_phone: String,
    pub effective_guest_contact_email: String,
    pub amenities: serde_json::Value,
    pub verification_status: String,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub cover_media_id: Option<String>,
    pub cover_media_version: Option<i32>,
    pub cover_media_mime: Option<String>,
    pub unit_type_count: i64,
    pub unit_count: i64,
    pub from_amount: Option<String>,
    pub photo_count: i64,
    pub open_reservations: i64,
    pub pending_requests: i64,
}
