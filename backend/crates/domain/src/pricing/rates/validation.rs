use kernel::{Error, Result};

use super::engine::{Adjustment, RateRule};
use crate::pricing::messages::{BASE_RATE_NOT_POSITIVE, CHOICE_INVALID, PERCENT_OUT_OF_RANGE};
use crate::pricing::validation::{money_minor, MAX_PERCENT_MINOR};

pub const OCCUPANCY_BASES: [&str; 2] = ["PER_UNIT", "PER_GUEST"];
pub const MEAL_PLANS: [&str; 4] = ["ROOM_ONLY", "BREAKFAST", "HALF_BOARD", "FULL_BOARD"];
pub const BOOKING_MODES: [&str; 2] = ["INSTANT", "REQUEST"];
pub const PLAN_STATUSES: [&str; 4] = ["DRAFT", "ACTIVE", "PAUSED", "ARCHIVED"];

pub fn adjustment(value: &str) -> Result<Adjustment> {
    match value {
        "OVERRIDE" => Ok(Adjustment::Override),
        "AMOUNT" => Ok(Adjustment::Amount),
        "PERCENTAGE" => Ok(Adjustment::Percentage),
        _ => Err(Error::bad_request(CHOICE_INVALID).with_params(serde_json::json!({
            "field": "adjustmentType",
            "allowed": ["OVERRIDE", "AMOUNT", "PERCENTAGE"],
        }))),
    }
}

pub fn rule_amount(adjustment: Adjustment, text: &str) -> Result<i64> {
    let minor = money_minor(text, "amount")?;
    match adjustment {
        Adjustment::Override if minor <= 0 => Err(Error::bad_request(BASE_RATE_NOT_POSITIVE)),
        Adjustment::Percentage if minor <= 0 || minor > MAX_PERCENT_MINOR => {
            Err(Error::bad_request(PERCENT_OUT_OF_RANGE).with_params(serde_json::json!({ "field": "amount" })))
        }
        _ => Ok(minor),
    }
}

pub fn overlapping_base<'a>(existing: &'a [RateRule], candidate: &RateRule) -> Option<&'a RateRule> {
    if candidate.adjustment != Adjustment::Override {
        return None;
    }
    existing.iter().find(|rule| {
        rule.adjustment == Adjustment::Override
            && rule.priority == candidate.priority
            && rule.weekday_mask & candidate.weekday_mask != 0
            && dates_overlap(rule, candidate)
    })
}

fn dates_overlap(left: &RateRule, right: &RateRule) -> bool {
    let left_starts_before_right_ends = match (left.start, right.end) {
        (Some(start), Some(end)) => start <= end,
        _ => true,
    };
    let right_starts_before_left_ends = match (right.start, left.end) {
        (Some(start), Some(end)) => start <= end,
        _ => true,
    };
    left_starts_before_right_ends && right_starts_before_left_ends
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pricing::validation::{money_minor, period, weekday_mask, MAX_MONEY_MINOR};
    use chrono::NaiveDate;

    fn date(text: &str) -> NaiveDate {
        NaiveDate::parse_from_str(text, "%Y-%m-%d").unwrap()
    }

    fn rule(priority: i32, mask: u8, start: Option<NaiveDate>, end: Option<NaiveDate>) -> RateRule {
        RateRule {
            id: format!("r{priority}"),
            start,
            end,
            weekday_mask: mask,
            adjustment: Adjustment::Override,
            amount_minor: 1_000,
            priority,
        }
    }

    #[test]
    fn money_rejects_negative_overprecise_and_huge_values() {
        assert!(money_minor("-1.00", "amount").is_err());
        assert!(money_minor("1.005", "amount").is_err());
        assert!(money_minor("10000000000.00", "amount").is_err());
        assert_eq!(money_minor("9999999999.99", "amount").unwrap(), MAX_MONEY_MINOR);
    }

    #[test]
    fn percentages_must_be_above_zero_and_at_most_one_hundred() {
        assert!(rule_amount(Adjustment::Percentage, "0").is_err());
        assert!(rule_amount(Adjustment::Percentage, "100.01").is_err());
        assert_eq!(rule_amount(Adjustment::Percentage, "10.00").unwrap(), 1_000);
        assert!(rule_amount(Adjustment::Override, "0").is_err());
        assert_eq!(rule_amount(Adjustment::Amount, "0").unwrap(), 0);
    }

    #[test]
    fn period_must_not_end_before_it_starts() {
        assert!(period(Some(date("2026-12-20")), Some(date("2026-12-19"))).is_err());
        assert!(period(Some(date("2026-12-20")), Some(date("2026-12-20"))).is_ok());
        assert!(period(None, Some(date("2026-12-19"))).is_ok());
    }

    #[test]
    fn weekdays_must_be_chosen_and_in_range() {
        assert!(weekday_mask(&[]).is_err());
        assert!(weekday_mask(&[0]).is_err());
        assert!(weekday_mask(&[8]).is_err());
        assert_eq!(weekday_mask(&[1, 5]).unwrap(), 1 | (1 << 4));
    }

    #[test]
    fn overlapping_base_rates_at_the_same_priority_are_found() {
        let existing = [rule(0, 127, Some(date("2026-12-01")), Some(date("2026-12-31")))];
        let inside = rule(0, 127, Some(date("2026-12-20")), Some(date("2027-01-02")));
        assert!(overlapping_base(&existing, &inside).is_some());
        let later = rule(0, 127, Some(date("2027-01-01")), None);
        assert!(overlapping_base(&existing, &later).is_none());
        let other_priority = rule(1, 127, Some(date("2026-12-20")), None);
        assert!(overlapping_base(&existing, &other_priority).is_none());
        let other_days = rule(0, 1, Some(date("2026-12-20")), None);
        assert!(overlapping_base(&[rule(0, 2, None, None)], &other_days).is_none());
    }

    #[test]
    fn open_ended_ranges_overlap_everything_on_matching_days() {
        let existing = [rule(0, 127, None, None)];
        let candidate = rule(0, 127, Some(date("2030-01-01")), Some(date("2030-01-02")));
        assert!(overlapping_base(&existing, &candidate).is_some());
    }
}
