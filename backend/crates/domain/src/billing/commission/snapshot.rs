use kernel::{Error, Money, Result};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub const MAX_RATE_BPS: i32 = 10_000;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CommissionSnapshot {
    pub rate_bps: i32,
    #[schema(value_type = String)]
    pub amount: Money,
    #[schema(value_type = String)]
    pub net: Money,
}

impl CommissionSnapshot {
    pub fn compute(rate_bps: i32, total: Money) -> Result<Self> {
        if !(0..=MAX_RATE_BPS).contains(&rate_bps) {
            return Err(Error::internal(format!("commission rate {rate_bps} out of range")));
        }
        let commission = total.percent_of(i64::from(rate_bps));
        Ok(Self {
            rate_bps,
            amount: commission,
            net: Money::from_minor(total.minor() - commission.minor()),
        })
    }

    pub fn without_commission(total: Money) -> Result<Self> {
        Self::compute(0, total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_total_by_rate_with_two_decimals() {
        let snapshot = CommissionSnapshot::compute(1_250, Money::from_minor(18_000_000)).unwrap();
        assert_eq!(snapshot.amount.to_wire(), "22500.00");
        assert_eq!(snapshot.net.to_wire(), "157500.00");
    }

    #[test]
    fn zero_rate_keeps_the_whole_total() {
        let snapshot = CommissionSnapshot::without_commission(Money::parse("410000.00").unwrap()).unwrap();
        assert_eq!(snapshot.amount.to_wire(), "0.00");
        assert_eq!(snapshot.net.to_wire(), "410000.00");
    }

    #[test]
    fn rejects_rates_above_one_hundred_percent() {
        assert!(CommissionSnapshot::compute(10_001, Money::from_minor(100)).is_err());
    }
}
