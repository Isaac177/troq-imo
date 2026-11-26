use chrono::{Datelike, NaiveDate};
use kernel::{Error, Result};
use serde_json::json;

use crate::pricing::amounts::percent_of;
use crate::pricing::messages::{BASE_RATE_CONFLICT, NO_BASE_RATE};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Adjustment {
    Override,
    Amount,
    Percentage,
}

impl Adjustment {
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "OVERRIDE" => Ok(Self::Override),
            "AMOUNT" => Ok(Self::Amount),
            "PERCENTAGE" => Ok(Self::Percentage),
            other => Err(Error::internal(format!("unknown adjustment {other}"))),
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Self::Override => "OVERRIDE",
            Self::Amount => "AMOUNT",
            Self::Percentage => "PERCENTAGE",
        }
    }
}

#[derive(Debug, Clone)]
pub struct RateRule {
    pub id: String,
    pub start: Option<NaiveDate>,
    pub end: Option<NaiveDate>,
    pub weekday_mask: u8,
    pub adjustment: Adjustment,
    pub amount_minor: i64,
    pub priority: i32,
}

impl RateRule {
    pub fn within_dates(&self, date: NaiveDate) -> bool {
        self.start.map_or(true, |start| date >= start) && self.end.map_or(true, |end| date <= end)
    }

    pub fn on_weekday(&self, date: NaiveDate) -> bool {
        let weekday_bit = 1u8 << date.weekday().num_days_from_monday();
        self.weekday_mask & weekday_bit != 0
    }

    pub fn applies_on(&self, date: NaiveDate) -> bool {
        self.within_dates(date) && self.on_weekday(date)
    }
}

pub fn nightly_minor(date: NaiveDate, rules: &[RateRule]) -> Result<i64> {
    let mut applicable: Vec<&RateRule> =
        rules.iter().filter(|rule| rule.applies_on(date)).collect();
    applicable.sort_by(|left, right| right.priority.cmp(&left.priority));
    let base = base_rate(date, &applicable)?;
    applicable
        .iter()
        .try_fold(base, |sum, rule| match rule.adjustment {
            Adjustment::Override => Some(sum),
            Adjustment::Amount => sum.checked_add(rule.amount_minor),
            Adjustment::Percentage => sum.checked_add(percent_of(base, rule.amount_minor)),
        })
        .ok_or_else(|| Error::internal(format!("rate overflow on {date}")))
}

fn base_rate(date: NaiveDate, applicable: &[&RateRule]) -> Result<i64> {
    let top = applicable
        .iter()
        .filter(|rule| rule.adjustment == Adjustment::Override)
        .map(|rule| rule.priority)
        .max()
        .ok_or_else(|| Error::conflict(NO_BASE_RATE).with_params(json!({ "date": date })))?;
    let bases: Vec<i64> = applicable
        .iter()
        .filter(|rule| rule.adjustment == Adjustment::Override && rule.priority == top)
        .map(|rule| rule.amount_minor)
        .collect();
    match bases.as_slice() {
        [only] => Ok(*only),
        _ => Err(Error::conflict(BASE_RATE_CONFLICT).with_params(json!({ "date": date }))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(text: &str) -> NaiveDate {
        NaiveDate::parse_from_str(text, "%Y-%m-%d").unwrap()
    }

    fn rule(adjustment: Adjustment, amount_minor: i64, priority: i32) -> RateRule {
        RateRule {
            id: format!("rule-{priority}-{amount_minor}"),
            start: None,
            end: None,
            weekday_mask: 127,
            adjustment,
            amount_minor,
            priority,
        }
    }

    #[test]
    fn base_override_is_the_nightly_price() {
        let rules = [rule(Adjustment::Override, 18_000_000, 0)];
        assert_eq!(nightly_minor(date("2026-12-20"), &rules).unwrap(), 18_000_000);
    }

    #[test]
    fn missing_base_rate_is_a_conflict() {
        let rules = [rule(Adjustment::Amount, 2_500_000, 10)];
        assert!(nightly_minor(date("2026-12-20"), &rules).is_err());
    }

    #[test]
    fn seasonal_amount_adds_on_matching_dates_only() {
        let mut seasonal = rule(Adjustment::Amount, 2_500_000, 10);
        seasonal.start = Some(date("2026-12-15"));
        seasonal.end = Some(date("2027-01-05"));
        let rules = [rule(Adjustment::Override, 18_000_000, 0), seasonal];
        assert_eq!(nightly_minor(date("2026-12-20"), &rules).unwrap(), 20_500_000);
        assert_eq!(nightly_minor(date("2026-11-20"), &rules).unwrap(), 18_000_000);
    }

    #[test]
    fn weekday_mask_limits_a_rule_to_its_days() {
        let mut weekend = rule(Adjustment::Amount, 1_000_000, 5);
        let saturday = date("2026-12-19");
        weekend.weekday_mask = 1u8 << saturday.weekday().num_days_from_monday();
        let rules = [rule(Adjustment::Override, 18_000_000, 0), weekend];
        assert_eq!(nightly_minor(saturday, &rules).unwrap(), 19_000_000);
        assert_eq!(nightly_minor(saturday.succ_opt().unwrap(), &rules).unwrap(), 18_000_000);
    }

    #[test]
    fn highest_priority_override_is_the_base() {
        let rules = [
            rule(Adjustment::Override, 18_000_000, 0),
            rule(Adjustment::Override, 25_000_000, 9),
        ];
        assert_eq!(nightly_minor(date("2026-12-20"), &rules).unwrap(), 25_000_000);
    }

    #[test]
    fn equal_priority_base_rates_are_rejected_not_resolved_by_order() {
        let rules = [
            rule(Adjustment::Override, 18_000_000, 4),
            rule(Adjustment::Override, 25_000_000, 4),
        ];
        assert!(nightly_minor(date("2026-12-20"), &rules).is_err());
        let reversed = [rules[1].clone(), rules[0].clone()];
        assert!(nightly_minor(date("2026-12-20"), &reversed).is_err());
    }

    #[test]
    fn percentage_applies_to_the_base() {
        let rules = [
            rule(Adjustment::Override, 18_000_000, 0),
            rule(Adjustment::Percentage, 1_000, 3),
        ];
        assert_eq!(nightly_minor(date("2026-12-20"), &rules).unwrap(), 19_800_000);
    }
}
