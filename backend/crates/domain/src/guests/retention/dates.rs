use chrono::{Duration, NaiveDate};

pub const REDACTED: &str = "REDACTED";

pub fn retain_until(retention_days: Option<i64>, check_out: NaiveDate) -> NaiveDate {
    match retention_days {
        Some(days) => check_out + Duration::days(days),
        None => check_out,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn without_a_configured_period_retention_ends_at_check_out() {
        let check_out = NaiveDate::from_ymd_opt(2026, 12, 24).unwrap();
        assert_eq!(retain_until(None, check_out), check_out);
    }

    #[test]
    fn a_configured_period_extends_from_check_out() {
        let check_out = NaiveDate::from_ymd_opt(2026, 12, 24).unwrap();
        assert_eq!(retain_until(Some(30), check_out), NaiveDate::from_ymd_opt(2027, 1, 23).unwrap());
    }
}
