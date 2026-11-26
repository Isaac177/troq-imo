use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::inventory::restrictions::Violation;

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct AvailabilityQuery {
    pub property_id: String,
    pub unit_type_id: String,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NightAvailability {
    pub date: NaiveDate,
    pub remaining: i64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AvailabilityView {
    pub bookable: bool,
    pub nights: Vec<NightAvailability>,
    pub violations: Vec<Violation>,
}
