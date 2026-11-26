use chrono::NaiveDate;
use kernel::error::{Error, Result};
use serde_json::json;

use crate::inventory::messages::{BLOCK_REASON_UNKNOWN, DATE_RANGE_INVALID, DATE_RANGE_TOO_LONG};

pub const MAX_BLOCK_NIGHTS: i64 = 365;
pub const PLATFORM_REASON: &str = "PLATFORM";
pub const BLOCK_REASONS: [&str; 7] = [
    "MAINTENANCE",
    "OWNER_USE",
    "OPERATIONS",
    "OTHER",
    "EXTERNAL_BOOKING",
    "MANUAL_CLOSURE",
    PLATFORM_REASON,
];

pub fn validate_range(start: NaiveDate, end: NaiveDate) -> Result<i64> {
    let nights = (end - start).num_days();
    if nights < 1 {
        return Err(Error::bad_request(DATE_RANGE_INVALID));
    }
    if nights > MAX_BLOCK_NIGHTS {
        return Err(Error::bad_request(DATE_RANGE_TOO_LONG).with_params(json!({ "maxNights": MAX_BLOCK_NIGHTS })));
    }
    Ok(nights)
}

pub fn validate_reason(reason: &str) -> Result<()> {
    if BLOCK_REASONS.contains(&reason) {
        Ok(())
    } else {
        Err(Error::bad_request(BLOCK_REASON_UNKNOWN).with_params(json!({ "reason": reason })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 12, day).unwrap()
    }

    #[test]
    fn range_must_have_at_least_one_night() {
        assert_eq!(validate_range(date(10), date(12)).unwrap(), 2);
        assert!(validate_range(date(12), date(12)).is_err());
        assert!(validate_range(date(12), date(10)).is_err());
    }

    #[test]
    fn range_is_bounded() {
        let far = NaiveDate::from_ymd_opt(2027, 12, 20).unwrap();
        assert!(validate_range(date(10), far).is_err());
    }

    #[test]
    fn reasons_are_limited_to_the_catalogue() {
        assert!(validate_reason("MAINTENANCE").is_ok());
        assert!(validate_reason("EXTERNAL_BOOKING").is_ok());
        assert!(validate_reason("SOMETHING_ELSE").is_err());
    }
}
