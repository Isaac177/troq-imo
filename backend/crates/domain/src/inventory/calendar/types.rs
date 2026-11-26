use chrono::NaiveDate;
use kernel::error::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::{IntoParams, ToSchema};

use crate::inventory::messages::{DATE_RANGE_INVALID, DATE_RANGE_TOO_LONG};
use crate::inventory::restrictions::RestrictionView;

pub const MAX_RANGE_DAYS: i64 = 93;

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct CalendarQuery {
    pub property_id: String,
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct CalendarFeedQuery {
    pub property_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CalendarDay {
    pub date: NaiveDate,
    pub capacity: i32,
    pub allocated: i32,
    pub price: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CalendarUnit {
    pub unit_type_id: String,
    pub name_en: String,
    pub name_fr: String,
    pub inventory_model: String,
    pub default_nightly_capacity: bool,
    pub inventory_version: Option<i32>,
    pub unit_type_cover_url: Option<String>,
    pub days: Vec<CalendarDay>,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CalendarReservation {
    pub id: String,
    pub reference: String,
    pub state: String,
    pub version: i32,
    pub total: String,
    pub adult_count: i32,
    pub child_count: i32,
    pub unit_type_id: String,
    pub physical_unit_id: Option<String>,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub guest_name: Option<String>,
    pub pending_amendment: Option<serde_json::Value>,
    #[sqlx(skip)]
    pub unit_type_cover_url: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CalendarBlock {
    pub id: String,
    pub unit_type_id: Option<String>,
    pub physical_unit_id: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub reason: String,
    pub version: i32,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CalendarView {
    pub property_id: String,
    pub timezone: String,
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub units: Vec<CalendarUnit>,
    pub total_units: i64,
    pub page: u32,
    pub page_size: u32,
    pub reservations: Vec<CalendarReservation>,
    pub blocks: Vec<CalendarBlock>,
    pub restrictions: Vec<RestrictionView>,
}

pub fn validate_range(from: NaiveDate, to: NaiveDate) -> Result<()> {
    let days = (to - from).num_days();
    if days < 1 {
        return Err(Error::bad_request(DATE_RANGE_INVALID));
    }
    if days > MAX_RANGE_DAYS {
        return Err(Error::bad_request(DATE_RANGE_TOO_LONG).with_params(json!({ "maxDays": MAX_RANGE_DAYS })));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, month, day).unwrap()
    }

    #[test]
    fn ranges_need_one_day_and_at_most_ninety_three() {
        assert!(validate_range(date(11, 1), date(12, 1)).is_ok());
        assert!(validate_range(date(11, 1), date(11, 1)).is_err());
        assert!(validate_range(date(11, 1), date(2, 1)).is_err());
        assert!(validate_range(date(1, 1), date(6, 1)).is_err());
    }
}
