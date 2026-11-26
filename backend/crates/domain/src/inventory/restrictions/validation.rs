use chrono::NaiveDate;
use kernel::error::{Error, Result};
use serde_json::json;

use crate::inventory::messages::{
    RESTRICTION_BASIS_INVALID, RESTRICTION_KIND_UNKNOWN, RESTRICTION_VALUE_INVALID,
    RESTRICTION_VALUE_NOT_ALLOWED, RESTRICTION_WINDOW_INVALID, RESTRICTION_WINDOW_TOO_LONG,
};

pub const MAX_BULK_OPERATIONS: usize = 50;
pub const MAX_RESTRICTION_SPAN_DAYS: i64 = 730;

pub const NUMERIC_KINDS: [&str; 5] = [
    "MIN_NIGHTS",
    "MAX_NIGHTS",
    "ADVANCE_NOTICE_DAYS",
    "MAX_HORIZON_DAYS",
    "TURNOVER_BUFFER_NIGHTS",
];
pub const DATE_KINDS: [&str; 2] = ["CLOSED_TO_ARRIVAL", "CLOSED_TO_DEPARTURE"];

pub fn validate_kind(kind: &str) -> Result<()> {
    if NUMERIC_KINDS.contains(&kind) || DATE_KINDS.contains(&kind) {
        Ok(())
    } else {
        Err(Error::bad_request(RESTRICTION_KIND_UNKNOWN).with_params(json!({ "kind": kind })))
    }
}

pub fn validate_basis(kind: &str, basis: &str) -> Result<()> {
    match basis {
        "STAY_THROUGH" => Ok(()),
        "ARRIVAL" if kind == "MIN_NIGHTS" => Ok(()),
        _ => Err(Error::bad_request(RESTRICTION_BASIS_INVALID)
            .with_params(json!({ "kind": kind, "basis": basis }))),
    }
}

pub fn validate_value(kind: &str, value: Option<i32>) -> Result<()> {
    let numeric = NUMERIC_KINDS.contains(&kind);
    match value {
        Some(number) if numeric && (0..=365).contains(&number) => Ok(()),
        None if !numeric => Ok(()),
        Some(_) if !numeric => {
            Err(Error::bad_request(RESTRICTION_VALUE_NOT_ALLOWED).with_params(json!({ "kind": kind })))
        }
        _ => Err(Error::bad_request(RESTRICTION_VALUE_INVALID)
            .with_params(json!({ "kind": kind, "min": 0, "max": 365 }))),
    }
}

pub fn validate_window(start: Option<NaiveDate>, end: Option<NaiveDate>) -> Result<()> {
    if let (Some(from), Some(to)) = (start, end) {
        if to <= from {
            return Err(Error::bad_request(RESTRICTION_WINDOW_INVALID));
        }
        if (to - from).num_days() > MAX_RESTRICTION_SPAN_DAYS {
            return Err(Error::bad_request(RESTRICTION_WINDOW_TOO_LONG)
                .with_params(json!({ "maxDays": MAX_RESTRICTION_SPAN_DAYS })));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 12, day).unwrap()
    }

    #[test]
    fn kinds_are_limited_to_the_catalogue() {
        assert!(validate_kind("MIN_NIGHTS").is_ok());
        assert!(validate_kind("CLOSED_TO_ARRIVAL").is_ok());
        assert!(validate_kind("FREE_NIGHTS").is_err());
    }

    #[test]
    fn arrival_basis_is_only_for_minimum_nights() {
        assert!(validate_basis("MIN_NIGHTS", "ARRIVAL").is_ok());
        assert!(validate_basis("MAX_NIGHTS", "ARRIVAL").is_err());
        assert!(validate_basis("MIN_NIGHTS", "STAY_THROUGH").is_ok());
    }

    #[test]
    fn values_match_the_kind() {
        assert!(validate_value("MIN_NIGHTS", Some(3)).is_ok());
        assert!(validate_value("MIN_NIGHTS", None).is_err());
        assert!(validate_value("CLOSED_TO_ARRIVAL", None).is_ok());
        assert!(validate_value("CLOSED_TO_ARRIVAL", Some(1)).is_err());
        assert!(validate_value("MAX_HORIZON_DAYS", Some(400)).is_err());
    }

    #[test]
    fn windows_are_ordered_and_bounded() {
        assert!(validate_window(Some(date(10)), Some(date(12))).is_ok());
        assert!(validate_window(Some(date(12)), Some(date(10))).is_err());
        assert!(validate_window(None, Some(date(10))).is_ok());
    }
}
