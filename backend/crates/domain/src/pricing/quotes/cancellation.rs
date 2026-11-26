use chrono::{DateTime, Duration, FixedOffset, NaiveDate, NaiveTime, TimeZone, Utc};
use kernel::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::pricing::amounts::percent_of;
use crate::pricing::messages::CANCELLATION_POLICY_INVALID;

pub const MAX_FREE_CANCELLATION_HOURS: i64 = 24 * 366;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancellationTerms {
    pub free_until_hours: i64,
    pub penalty_percent: i64,
}

impl CancellationTerms {
    pub fn from_content(content: &Value) -> Result<Self> {
        let terms: Self = serde_json::from_value(content.clone())
            .map_err(|_| Error::conflict(CANCELLATION_POLICY_INVALID))?;
        if !(0..=MAX_FREE_CANCELLATION_HOURS).contains(&terms.free_until_hours)
            || !(0..=100).contains(&terms.penalty_percent)
        {
            return Err(Error::conflict(CANCELLATION_POLICY_INVALID));
        }
        Ok(terms)
    }
}

pub fn free_cancellation_deadline(
    arrival: NaiveDate,
    offset: FixedOffset,
    terms: CancellationTerms,
) -> Result<DateTime<Utc>> {
    let local_arrival = offset
        .from_local_datetime(&arrival.and_time(NaiveTime::MIN))
        .single()
        .ok_or_else(|| Error::internal("arrival time is ambiguous"))?;
    Ok((local_arrival - Duration::hours(terms.free_until_hours)).with_timezone(&Utc))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefundEvaluation {
    pub reason: &'static str,
    pub refund_minor: i64,
    pub penalty_minor: i64,
}

pub fn evaluate(
    terms: CancellationTerms,
    deadline: DateTime<Utc>,
    now: DateTime<Utc>,
    paid_minor: i64,
) -> RefundEvaluation {
    if now <= deadline {
        return RefundEvaluation {
            reason: "FREE_CANCELLATION",
            refund_minor: paid_minor,
            penalty_minor: 0,
        };
    }
    let penalty_minor = percent_of(paid_minor, terms.penalty_percent * 100);
    RefundEvaluation {
        reason: "PENALTY_APPLIES",
        refund_minor: paid_minor - penalty_minor,
        penalty_minor,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn kinshasa() -> FixedOffset {
        FixedOffset::east_opt(60 * 60).unwrap()
    }

    fn strict() -> CancellationTerms {
        CancellationTerms { free_until_hours: 168, penalty_percent: 50 }
    }

    fn at(text: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(text).unwrap().with_timezone(&Utc)
    }

    #[test]
    fn terms_parse_the_seeded_policy_shape() {
        let terms = CancellationTerms::from_content(&json!({"freeUntilHours": 48, "penaltyPercent": 0})).unwrap();
        assert_eq!(terms.free_until_hours, 48);
        assert_eq!(terms.penalty_percent, 0);
    }

    #[test]
    fn terms_reject_out_of_range_values() {
        assert!(CancellationTerms::from_content(&json!({"freeUntilHours": -1, "penaltyPercent": 0})).is_err());
        assert!(CancellationTerms::from_content(&json!({"freeUntilHours": 48, "penaltyPercent": 101})).is_err());
        assert!(CancellationTerms::from_content(&json!({"penaltyPercent": 0})).is_err());
    }

    #[test]
    fn deadline_is_hours_before_local_arrival_midnight() {
        let arrival = NaiveDate::from_ymd_opt(2026, 12, 20).unwrap();
        let deadline = free_cancellation_deadline(arrival, kinshasa(), strict()).unwrap();
        assert_eq!(deadline, at("2026-12-12T23:00:00Z"));
    }

    #[test]
    fn free_cancellation_refunds_everything_up_to_the_deadline() {
        let evaluation = evaluate(strict(), at("2026-12-13T23:00:00Z"), at("2026-12-13T22:59:59Z"), 36_000_000);
        assert_eq!(evaluation.reason, "FREE_CANCELLATION");
        assert_eq!(evaluation.refund_minor, 36_000_000);
        assert_eq!(evaluation.penalty_minor, 0);
    }

    #[test]
    fn late_cancellation_keeps_the_penalty_percentage() {
        let evaluation = evaluate(strict(), at("2026-12-13T23:00:00Z"), at("2026-12-14T00:00:00Z"), 36_000_000);
        assert_eq!(evaluation.reason, "PENALTY_APPLIES");
        assert_eq!(evaluation.penalty_minor, 18_000_000);
        assert_eq!(evaluation.refund_minor, 18_000_000);
    }

    #[test]
    fn nonrefundable_policy_keeps_all_money_after_the_deadline() {
        let terms = CancellationTerms { free_until_hours: 0, penalty_percent: 100 };
        let evaluation = evaluate(terms, at("2026-12-13T23:00:00Z"), at("2026-12-13T23:00:01Z"), 1_000_001);
        assert_eq!(evaluation.refund_minor, 0);
        assert_eq!(evaluation.penalty_minor, 1_000_001);
    }
}
