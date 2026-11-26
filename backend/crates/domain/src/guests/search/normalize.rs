use chrono::NaiveDate;
use kernel::error::{Error, Result};
use kernel::Money;
use serde_json::json;

use crate::guests::collaborators::MAX_STAY_NIGHTS;
use super::query::{Origin, Window, FLEXIBLE_FREE_HOURS, PUBLIC_COORDINATE_DECIMALS, SORTS};
use crate::guests::messages::{
    SEARCH_CODE_MALFORMED, SEARCH_CODE_UNKNOWN, SEARCH_COORDINATES_INCOMPLETE,
    SEARCH_COORDINATES_RANGE, SEARCH_DATES_INCOMPLETE, SEARCH_DATES_ORDER, SEARCH_SORT_INVALID,
    SEARCH_STAY_TOO_LONG,
};

pub const MAX_CODE_LENGTH: usize = 48;

pub fn cancellation_class(free_hours: Option<i64>, penalty_percent: Option<i64>) -> &'static str {
    match (free_hours, penalty_percent) {
        (_, Some(penalty)) if penalty >= 100 => "NON_REFUNDABLE",
        (Some(hours), Some(0)) if hours >= FLEXIBLE_FREE_HOURS => "FLEXIBLE",
        _ => "MODERATE",
    }
}

pub fn parse_price(text: &str) -> Result<i64> {
    Ok(Money::parse_input(text)?.minor())
}

pub fn window_from(check_in: Option<NaiveDate>, check_out: Option<NaiveDate>) -> Result<Option<Window>> {
    match (check_in, check_out) {
        (None, None) => Ok(None),
        (Some(check_in), Some(check_out)) => {
            let nights = (check_out - check_in).num_days();
            if nights < 1 {
                return Err(Error::bad_request(SEARCH_DATES_ORDER));
            }
            if nights > MAX_STAY_NIGHTS {
                return Err(Error::bad_request(SEARCH_STAY_TOO_LONG).with_params(json!({ "max": MAX_STAY_NIGHTS })));
            }
            Ok(Some(Window { check_in, check_out, nights }))
        }
        _ => Err(Error::bad_request(SEARCH_DATES_INCOMPLETE)),
    }
}

pub fn origin_from(latitude: Option<f64>, longitude: Option<f64>) -> Result<Option<Origin>> {
    match (latitude, longitude) {
        (None, None) => Ok(None),
        (Some(latitude), Some(longitude)) => {
            if !(-90.0..=90.0).contains(&latitude) || !(-180.0..=180.0).contains(&longitude) {
                return Err(Error::bad_request(SEARCH_COORDINATES_RANGE));
            }
            Ok(Some(Origin {
                latitude: round_to(latitude, PUBLIC_COORDINATE_DECIMALS),
                longitude: round_to(longitude, PUBLIC_COORDINATE_DECIMALS),
            }))
        }
        _ => Err(Error::bad_request(SEARCH_COORDINATES_INCOMPLETE)),
    }
}

fn round_to(value: f64, decimals: i32) -> f64 {
    let factor = 10f64.powi(decimals);
    (value * factor).round() / factor
}

pub fn normalize_sort(value: Option<&str>) -> Result<String> {
    let sort = value.unwrap_or("relevance").trim().to_lowercase();
    if SORTS.contains(&sort.as_str()) {
        Ok(sort)
    } else {
        Err(Error::bad_request(SEARCH_SORT_INVALID).with_params(json!({ "allowed": SORTS })))
    }
}

pub fn normalize_codes(values: Vec<String>, allowed: &[&str], field: &str) -> Result<Vec<String>> {
    let mut codes: Vec<String> = values
        .into_iter()
        .map(|value| value.trim().to_uppercase())
        .filter(|value| !value.is_empty())
        .collect();
    if let Some(unknown) = codes.iter().find(|code| !allowed.contains(&code.as_str())) {
        return Err(Error::bad_request(SEARCH_CODE_UNKNOWN)
            .with_params(json!({ "field": field, "value": unknown, "allowed": allowed })));
    }
    codes.sort();
    codes.dedup();
    Ok(codes)
}

pub fn is_valid_code(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_CODE_LENGTH
        && value
            .chars()
            .all(|character| character.is_ascii_uppercase() || character.is_ascii_digit() || character == '_')
}

pub fn normalize_code_format(values: Vec<String>, field: &str) -> Result<Vec<String>> {
    let mut codes: Vec<String> = values
        .into_iter()
        .map(|value| value.trim().to_uppercase())
        .filter(|value| !value.is_empty())
        .collect();
    if let Some(bad) = codes.iter().find(|code| !is_valid_code(code)) {
        return Err(Error::bad_request(SEARCH_CODE_MALFORMED).with_params(json!({ "field": field, "value": bad })));
    }
    codes.sort();
    codes.dedup();
    Ok(codes)
}
