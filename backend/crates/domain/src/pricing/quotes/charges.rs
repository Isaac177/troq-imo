use chrono::NaiveDate;
use kernel::{Error, Result};

use crate::pricing::promotions::{best_length_of_stay, Discount, Promotion};
use crate::pricing::rates::{nightly_minor, RateRule};

pub struct StayTerms<'a> {
    pub rules: &'a [RateRule],
    pub guest_factor: i64,
    pub child_nightly_minor: i64,
    pub child_count: i64,
    pub promotions: &'a [Promotion],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NightCharge {
    pub date: NaiveDate,
    pub amount_minor: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PricedStay {
    pub nights: Vec<NightCharge>,
    pub subtotal_minor: i64,
    pub discount: Option<Discount>,
    pub total_minor: i64,
}

pub fn price_stay(nights: &[NaiveDate], terms: &StayTerms) -> Result<PricedStay> {
    let overflow = || Error::internal("quote amount overflow");
    let mut lines = Vec::with_capacity(nights.len());
    let mut subtotal: i64 = 0;
    for night in nights {
        let base = nightly_minor(*night, terms.rules)?
            .checked_mul(terms.guest_factor)
            .ok_or_else(overflow)?;
        let children = terms
            .child_nightly_minor
            .checked_mul(terms.child_count)
            .ok_or_else(overflow)?;
        let amount = base.checked_add(children).ok_or_else(overflow)?;
        subtotal = subtotal.checked_add(amount).ok_or_else(overflow)?;
        lines.push(NightCharge { date: *night, amount_minor: amount });
    }
    let stay_nights = i64::try_from(nights.len()).map_err(|_| overflow())?;
    let discount = best_length_of_stay(stay_nights, subtotal, terms.promotions);
    let discount_minor = discount.as_ref().map_or(0, |applied| applied.amount_minor);
    let total_minor = subtotal.checked_sub(discount_minor).ok_or_else(overflow)?;
    Ok(PricedStay { nights: lines, subtotal_minor: subtotal, discount, total_minor })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pricing::rates::Adjustment;

    fn date(text: &str) -> NaiveDate {
        NaiveDate::parse_from_str(text, "%Y-%m-%d").unwrap()
    }

    fn base_rules() -> Vec<RateRule> {
        vec![RateRule {
            id: "base".into(),
            start: None,
            end: None,
            weekday_mask: 127,
            adjustment: Adjustment::Override,
            amount_minor: 18_000_000,
            priority: 0,
        }]
    }

    fn nights(check_in: &str, count: i64) -> Vec<NaiveDate> {
        let start = date(check_in);
        (0..count).map(|offset| start + chrono::Duration::days(offset)).collect()
    }

    fn weekly(percent_minor: i64, cap_minor: Option<i64>) -> Promotion {
        Promotion {
            id: "weekly".into(),
            min_nights: 7,
            percent_minor,
            cap_minor,
            priority: 0,
            redemption_limit: None,
            redeemed: 0,
        }
    }

    #[test]
    fn child_charge_is_added_to_every_night() {
        let rules = base_rules();
        let terms = StayTerms {
            rules: &rules,
            guest_factor: 1,
            child_nightly_minor: 2_000_000,
            child_count: 2,
            promotions: &[],
        };
        let priced = price_stay(&nights("2026-12-20", 2), &terms).unwrap();
        assert_eq!(priced.nights[0].amount_minor, 22_000_000);
        assert_eq!(priced.subtotal_minor, 44_000_000);
        assert_eq!(priced.total_minor, 44_000_000);
        assert!(priced.discount.is_none());
    }

    #[test]
    fn per_guest_factor_multiplies_the_base_only() {
        let rules = base_rules();
        let terms = StayTerms {
            rules: &rules,
            guest_factor: 3,
            child_nightly_minor: 2_000_000,
            child_count: 1,
            promotions: &[],
        };
        let priced = price_stay(&nights("2026-12-20", 1), &terms).unwrap();
        assert_eq!(priced.nights[0].amount_minor, 56_000_000);
    }

    #[test]
    fn weekly_promotion_applies_at_seven_nights_with_cap() {
        let rules = base_rules();
        let promotions = [weekly(1_000, Some(5_000_000))];
        let terms = StayTerms {
            rules: &rules,
            guest_factor: 1,
            child_nightly_minor: 2_000_000,
            child_count: 1,
            promotions: &promotions,
        };
        let seven = price_stay(&nights("2026-12-20", 7), &terms).unwrap();
        assert_eq!(seven.subtotal_minor, 140_000_000);
        assert_eq!(seven.discount.as_ref().unwrap().amount_minor, 5_000_000);
        assert_eq!(seven.total_minor, 135_000_000);
        let six = price_stay(&nights("2026-12-20", 6), &terms).unwrap();
        assert!(six.discount.is_none());
        assert_eq!(six.total_minor, six.subtotal_minor);
    }

    #[test]
    fn missing_rate_on_a_night_fails_the_whole_stay() {
        let rules = vec![RateRule {
            id: "gap".into(),
            start: Some(date("2026-12-20")),
            end: Some(date("2026-12-20")),
            weekday_mask: 127,
            adjustment: Adjustment::Override,
            amount_minor: 18_000_000,
            priority: 0,
        }];
        let terms = StayTerms {
            rules: &rules,
            guest_factor: 1,
            child_nightly_minor: 0,
            child_count: 0,
            promotions: &[],
        };
        assert!(price_stay(&nights("2026-12-20", 2), &terms).is_err());
    }
}
