use chrono::NaiveDate;
use kernel::{Money, Result};
use serde::Serialize;
use utoipa::ToSchema;

use super::engine::{nightly_minor, Adjustment, RateRule};
use crate::pricing::amounts::money;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RuleUse {
    pub rule_id: String,
    pub adjustment: &'static str,
    #[schema(value_type = String)]
    pub amount: Money,
    pub priority: i32,
    pub status: &'static str,
    pub reason: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NightExplanation {
    pub date: NaiveDate,
    #[schema(value_type = String)]
    pub base_amount: Money,
    pub rules: Vec<RuleUse>,
}

pub fn explain_night(date: NaiveDate, rules: &[RateRule]) -> Result<NightExplanation> {
    let base_amount = nightly_minor(date, rules)?;
    let base_rule_id = rules
        .iter()
        .filter(|rule| rule.applies_on(date) && rule.adjustment == Adjustment::Override)
        .max_by_key(|rule| rule.priority)
        .map(|rule| rule.id.as_str());
    let mut ordered: Vec<&RateRule> = rules.iter().collect();
    ordered.sort_by(|left, right| right.priority.cmp(&left.priority).then(left.id.cmp(&right.id)));
    let rules_used = ordered
        .into_iter()
        .map(|rule| {
            let (status, reason) = if !rule.within_dates(date) {
                ("EXCLUDED", Some("OUTSIDE_DATE_RANGE"))
            } else if !rule.on_weekday(date) {
                ("EXCLUDED", Some("WEEKDAY_NOT_SELECTED"))
            } else if rule.adjustment == Adjustment::Override && Some(rule.id.as_str()) != base_rule_id {
                ("EXCLUDED", Some("SUPERSEDED_BY_HIGHER_PRIORITY_BASE"))
            } else {
                ("APPLIED", None)
            };
            RuleUse {
                rule_id: rule.id.clone(),
                adjustment: rule.adjustment.code(),
                amount: money(rule.amount_minor),
                priority: rule.priority,
                status,
                reason,
            }
        })
        .collect();
    Ok(NightExplanation { date, base_amount: money(base_amount), rules: rules_used })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(text: &str) -> NaiveDate {
        NaiveDate::parse_from_str(text, "%Y-%m-%d").unwrap()
    }

    fn rule(id: &str, adjustment: Adjustment, amount_minor: i64, priority: i32) -> RateRule {
        RateRule {
            id: id.into(),
            start: None,
            end: None,
            weekday_mask: 127,
            adjustment,
            amount_minor,
            priority,
        }
    }

    #[test]
    fn explains_applied_and_excluded_rules() {
        let mut december = rule("december", Adjustment::Amount, 2_500_000, 10);
        december.start = Some(date("2026-12-15"));
        december.end = Some(date("2027-01-05"));
        let mut weekend = rule("weekend", Adjustment::Amount, 1_000_000, 5);
        weekend.weekday_mask = 1 << 5;
        let rules = [rule("base", Adjustment::Override, 18_000_000, 0), december, weekend];
        let explained = explain_night(date("2026-11-20"), &rules).unwrap();
        assert_eq!(explained.base_amount.to_wire(), "180000.00");
        let status_of = |id: &str| explained.rules.iter().find(|item| item.rule_id == id).unwrap();
        assert_eq!(status_of("base").status, "APPLIED");
        assert_eq!(status_of("december").reason, Some("OUTSIDE_DATE_RANGE"));
        assert_eq!(status_of("weekend").reason, Some("WEEKDAY_NOT_SELECTED"));
    }

    #[test]
    fn lower_priority_base_is_explained_as_superseded() {
        let rules = [
            rule("low", Adjustment::Override, 18_000_000, 0),
            rule("high", Adjustment::Override, 25_000_000, 9),
        ];
        let explained = explain_night(date("2026-12-20"), &rules).unwrap();
        assert_eq!(explained.base_amount.to_wire(), "250000.00");
        let low = explained.rules.iter().find(|item| item.rule_id == "low").unwrap();
        assert_eq!(low.reason, Some("SUPERSEDED_BY_HIGHER_PRIORITY_BASE"));
    }
}
