use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct AnalyticsQuery {
    pub host_id: String,
    pub property_id: Option<String>,
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsSummary {
    pub currency: &'static str,
    pub reservations: i64,
    pub confirmed: i64,
    pub checked_in: i64,
    pub checked_out: i64,
    pub cancelled: i64,
    pub no_show: i64,
    pub gross_revenue: String,
    pub approved_reviews: i64,
    pub average_rating: Option<f64>,
}

#[derive(sqlx::FromRow)]
pub struct ReservationTotals {
    pub reservations: i64,
    pub confirmed: i64,
    pub checked_in: i64,
    pub checked_out: i64,
    pub cancelled: i64,
    pub no_show: i64,
    pub gross_revenue: String,
}

#[derive(sqlx::FromRow)]
pub struct ReviewTotals {
    pub approved_reviews: i64,
    pub average_rating: Option<f64>,
}
