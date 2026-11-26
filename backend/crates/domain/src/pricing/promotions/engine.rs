use std::cmp::Reverse;

use crate::pricing::amounts::percent_of;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Promotion {
    pub id: String,
    pub min_nights: i64,
    pub percent_minor: i64,
    pub cap_minor: Option<i64>,
    pub priority: i32,
    pub redemption_limit: Option<i64>,
    pub redeemed: i64,
}

impl Promotion {
    pub fn has_budget(&self) -> bool {
        self.redemption_limit.map_or(true, |limit| self.redeemed < limit)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Discount {
    pub promotion_id: String,
    pub amount_minor: i64,
}

pub fn best_length_of_stay(
    nights: i64,
    subtotal_minor: i64,
    promotions: &[Promotion],
) -> Option<Discount> {
    promotions
        .iter()
        .filter(|promotion| nights >= promotion.min_nights && promotion.has_budget())
        .map(|promotion| (discount_for(subtotal_minor, promotion), promotion))
        .max_by_key(|(amount, promotion)| (*amount, promotion.priority, Reverse(promotion.id.clone())))
        .filter(|(amount, _)| *amount > 0)
        .map(|(amount_minor, promotion)| Discount {
            promotion_id: promotion.id.clone(),
            amount_minor,
        })
}

fn discount_for(subtotal_minor: i64, promotion: &Promotion) -> i64 {
    let percentage = percent_of(subtotal_minor, promotion.percent_minor);
    let capped = promotion.cap_minor.map_or(percentage, |cap| percentage.min(cap));
    capped.clamp(0, subtotal_minor.max(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn promotion(id: &str, min_nights: i64, percent_minor: i64, cap_minor: Option<i64>, priority: i32) -> Promotion {
        Promotion {
            id: id.into(),
            min_nights,
            percent_minor,
            cap_minor,
            priority,
            redemption_limit: None,
            redeemed: 0,
        }
    }

    #[test]
    fn exhausted_promotion_is_not_applied() {
        let mut spent = promotion("spent", 1, 1_000, None, 0);
        spent.redemption_limit = Some(5);
        spent.redeemed = 5;
        assert!(best_length_of_stay(1, 100_000, &[spent.clone()]).is_none());
        spent.redeemed = 4;
        assert!(best_length_of_stay(1, 100_000, &[spent]).is_some());
    }

    #[test]
    fn promotion_needs_its_minimum_nights() {
        let promotions = [promotion("weekly", 7, 1_000, None, 0)];
        assert!(best_length_of_stay(6, 100_000_000, &promotions).is_none());
        assert_eq!(best_length_of_stay(7, 100_000_000, &promotions).unwrap().amount_minor, 10_000_000);
    }

    #[test]
    fn cap_limits_the_discount() {
        let promotions = [promotion("capped", 3, 1_000, Some(5_000_000), 0)];
        assert_eq!(best_length_of_stay(3, 140_000_000, &promotions).unwrap().amount_minor, 5_000_000);
    }

    #[test]
    fn largest_discount_wins_then_priority_then_id() {
        let promotions = [promotion("small", 2, 500, None, 9), promotion("big", 2, 1_000, None, 0)];
        assert_eq!(best_length_of_stay(2, 100_000_000, &promotions).unwrap().promotion_id, "big");
        let tied = [
            promotion("beta", 2, 1_000, None, 1),
            promotion("alpha", 2, 1_000, None, 1),
            promotion("gamma", 2, 1_000, None, 0),
        ];
        assert_eq!(best_length_of_stay(2, 100_000_000, &tied).unwrap().promotion_id, "alpha");
    }

    #[test]
    fn discount_rounds_down_to_the_minor_unit() {
        let promotions = [promotion("tiny", 1, 1_000, None, 0)];
        assert_eq!(best_length_of_stay(1, 15, &promotions).unwrap().amount_minor, 1);
        assert!(best_length_of_stay(1, 9, &promotions).is_none());
    }

    #[test]
    fn discount_never_exceeds_the_subtotal() {
        let promotions = [promotion("huge", 1, 10_000, None, 0)];
        assert_eq!(best_length_of_stay(1, 4_200, &promotions).unwrap().amount_minor, 4_200);
    }
}
