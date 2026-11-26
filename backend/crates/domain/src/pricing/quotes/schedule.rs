use kernel::Money;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaymentSchedule {
    #[schema(value_type = String)]
    pub due_now: Money,
    #[schema(value_type = String)]
    pub due_later: Money,
    #[schema(value_type = String)]
    pub due_at_property: Money,
}

pub fn pay_at_property(total: Money) -> PaymentSchedule {
    PaymentSchedule {
        due_now: Money::ZERO,
        due_later: Money::ZERO,
        due_at_property: total,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whole_total_is_due_at_the_property() {
        let schedule = pay_at_property(Money::from_minor(410_000_00));
        assert_eq!(schedule.due_now.to_wire(), "0.00");
        assert_eq!(schedule.due_later.to_wire(), "0.00");
        assert_eq!(schedule.due_at_property.to_wire(), "410000.00");
    }
}
