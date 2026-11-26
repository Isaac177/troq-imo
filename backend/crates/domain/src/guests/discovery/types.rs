use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};

use crate::guests::search::CancellationView;

#[derive(Debug, Default, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct LocaleQuery {
    pub locale: Option<String>,
}

#[derive(Debug, Default, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct AvailabilityQuery {
    pub check_in: Option<String>,
    pub check_out: Option<String>,
    pub adults: Option<String>,
    pub children: Option<String>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct CalendarQuery {
    pub unit_type_id: String,
    pub from: NaiveDate,
    pub to: NaiveDate,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DestinationView {
    pub city: String,
    pub property_count: i64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MediaView {
    pub id: String,
    pub alt: String,
    pub is_cover: bool,
    pub content_path: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ImageView {
    pub id: String,
    pub url: String,
    pub alt: String,
    pub is_cover: bool,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UnitTypeView {
    pub id: String,
    pub name: String,
    pub description: String,
    pub max_adults: i32,
    pub max_children: i32,
    pub bedroom_count: i32,
    pub bed_count: i32,
    pub bathroom_count: i32,
    pub images: Vec<ImageView>,
    pub cover_url: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RatePlanView {
    pub id: String,
    pub unit_type_id: String,
    pub occupancy_basis: String,
    pub meal_plan: String,
    pub booking_mode: String,
    pub nightly: Option<String>,
    pub cancellation: Option<serde_json::Value>,
    pub payment: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDetailView {
    pub id: String,
    pub city: String,
    pub timezone: String,
    pub name: String,
    pub description: String,
    pub verification_status: String,
    pub host_name: Option<String>,
    pub amenities: Vec<String>,
    pub media: Vec<MediaView>,
    pub unit_types: Vec<UnitTypeView>,
    pub rate_plans: Vec<RatePlanView>,
}

#[derive(Debug, Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct MediaContentView {
    pub media_id: String,
    pub mime_type: String,
    pub alt: String,
    pub url: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AvailabilityOptionView {
    pub unit_type_id: String,
    pub rate_plan_id: String,
    pub booking_mode: String,
    pub meal_plan: String,
    pub remaining_rooms: Option<i64>,
    pub total: Option<String>,
    pub cancellation: CancellationView,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AvailabilityView {
    pub property_id: String,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub nights: i64,
    pub currency: String,
    pub options: Vec<AvailabilityOptionView>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CalendarDayView {
    pub date: NaiveDate,
    pub available: bool,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CalendarView {
    pub unit_type_id: String,
    pub days: Vec<CalendarDayView>,
}
