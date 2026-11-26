use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Default, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct DashboardQuery {
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
    pub host_id: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DashboardFigures {
    pub bookings: i64,
    pub revenue: String,
    pub cancellations: i64,
    pub nights_sold: i64,
    pub nights_available: i64,
    pub pending_requests: i64,
    pub open_cases: i64,
    pub commission_upcoming: String,
    pub commission_completed: String,
    pub commission_taken: String,
    pub credits_topped_up: String,
    pub credit_balance: String,
    pub disabled_businesses: i64,
    pub payouts_pending: String,
    pub payouts_paid: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CityFigures {
    pub city: String,
    #[serde(flatten)]
    pub figures: DashboardFigures,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DashboardDay {
    pub date: NaiveDate,
    pub bookings: i64,
    pub revenue: String,
    pub cancellations: i64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DashboardSummary {
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub totals: DashboardFigures,
    pub cities: Vec<CityFigures>,
    pub series: Vec<DashboardDay>,
}
