use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Default, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct ReservationFilter {
    pub state: Option<String>,
    pub host_id: Option<String>,
    pub property_id: Option<String>,
    pub check_in_from: Option<NaiveDate>,
    pub check_in_to: Option<NaiveDate>,
    pub search: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReservationRow {
    pub id: String,
    pub reference: String,
    pub state: String,
    pub booking_mode: String,
    pub source: String,
    pub city: String,
    pub operator_id: String,
    pub operator_name: String,
    pub property_id: String,
    pub property_name_en: String,
    pub property_name_fr: String,
    pub unit_type_id: Option<String>,
    pub unit_type_name_en: Option<String>,
    pub unit_type_name_fr: Option<String>,
    #[sqlx(skip)]
    pub unit_type_cover_url: Option<String>,
    pub guest_name: Option<String>,
    pub check_in: Option<NaiveDate>,
    pub check_out: Option<NaiveDate>,
    pub nights: Option<i32>,
    pub total_amount: String,
    pub commission_amount: String,
    pub operator_net_amount: String,
    pub open_cases: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReservationSummary {
    pub total: i64,
    pub revenue: String,
    pub requested: i64,
    pub awaiting_payment: i64,
    pub confirmed: i64,
    pub checked_in: i64,
    pub cancelled: i64,
    pub no_show: i64,
}
