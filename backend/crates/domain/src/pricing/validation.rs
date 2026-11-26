use chrono::NaiveDate;
use kernel::error::codes::AMOUNT_INVALID;
use kernel::{Error, Money, Result};
use serde_json::json;

use crate::pricing::messages::*;

pub const MAX_MONEY_MINOR: i64 = 999_999_999_999;
pub const MAX_PERCENT_MINOR: i64 = 10_000;
pub const MAX_PRIORITY: i32 = 1_000;
pub const MAX_MIN_NIGHTS: i64 = 365;
pub const MAX_REDEMPTION_LIMIT: i32 = 1_000_000;

pub fn money_minor(text: &str, field: &str) -> Result<i64> {
    let money = Money::parse(text)
        .map_err(|_| Error::bad_request(AMOUNT_INVALID).with_params(json!({ "field": field })))?;
    if money.minor() > MAX_MONEY_MINOR {
        return Err(Error::bad_request(AMOUNT_TOO_LARGE).with_params(json!({ "field": field })));
    }
    Ok(money.minor())
}

pub fn choice(value: &str, allowed: &[&str], field: &str) -> Result<()> {
    if allowed.contains(&value) {
        Ok(())
    } else {
        Err(Error::bad_request(CHOICE_INVALID)
            .with_params(json!({ "field": field, "allowed": allowed })))
    }
}

pub fn period(start: Option<NaiveDate>, end: Option<NaiveDate>) -> Result<()> {
    match (start, end) {
        (Some(start), Some(end)) if end < start => Err(Error::bad_request(PERIOD_INVALID)),
        _ => Ok(()),
    }
}

pub fn priority(value: i32) -> Result<i32> {
    if (-MAX_PRIORITY..=MAX_PRIORITY).contains(&value) {
        Ok(value)
    } else {
        Err(Error::bad_request(PRIORITY_OUT_OF_RANGE).with_params(json!({ "max": MAX_PRIORITY })))
    }
}

pub fn weekday_mask(weekdays: &[u8]) -> Result<u8> {
    if weekdays.is_empty() || weekdays.iter().any(|day| !(1..=7).contains(day)) {
        return Err(Error::bad_request(WEEKDAYS_INVALID));
    }
    Ok(crate::pricing::rates::types::weekday_mask(weekdays))
}

pub fn min_nights(value: i64) -> Result<i64> {
    if (1..=MAX_MIN_NIGHTS).contains(&value) {
        Ok(value)
    } else {
        Err(Error::bad_request(MIN_NIGHTS_OUT_OF_RANGE).with_params(json!({ "max": MAX_MIN_NIGHTS })))
    }
}

pub fn promotion_percent(text: &str) -> Result<i64> {
    let minor = money_minor(text, "discountPercent")?;
    if minor <= 0 || minor > MAX_PERCENT_MINOR {
        return Err(Error::bad_request(PERCENT_OUT_OF_RANGE).with_params(json!({ "field": "discountPercent" })));
    }
    Ok(minor)
}

pub fn redemption_limit(value: Option<i32>) -> Result<Option<i32>> {
    match value {
        Some(limit) if !(1..=MAX_REDEMPTION_LIMIT).contains(&limit) => Err(Error::bad_request(
            REDEMPTION_LIMIT_OUT_OF_RANGE,
        )
        .with_params(json!({ "max": MAX_REDEMPTION_LIMIT }))),
        other => Ok(other),
    }
}
