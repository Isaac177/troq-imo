pub fn disabled_sql(operator_column: &str, city_column: &str) -> String {
    format!(
        r#"(COALESCE((SELECT cr."rateBps" FROM stay_operator_commission_rates cr
              WHERE cr."operatorId" = {operator_column} AND cr."effectiveFrom" <= now()
              ORDER BY cr."effectiveFrom" DESC, cr."createdAt" DESC, cr.id DESC LIMIT 1),
              COALESCE((SELECT ps."valueInt" FROM stay_platform_settings ps
                         WHERE ps.city_id = {city_column} AND ps.key = 'DEFAULT_COMMISSION_BPS'), 0)) > 0
          AND COALESCE((SELECT a.balance FROM stay_credit_accounts a WHERE a."operatorId" = {operator_column}), 0)
              <= COALESCE((SELECT ps."valueInt" FROM stay_platform_settings ps
                            WHERE ps.city_id = {city_column} AND ps.key = 'MIN_CREDIT_CDF'), 0))"#
    )
}

pub fn enabled_sql(operator_column: &str, city_column: &str) -> String {
    format!("(NOT {})", disabled_sql(operator_column, city_column))
}

pub fn is_disabled(rate_bps: i32, balance_minor: i64, min_credit_cdf: i32) -> bool {
    rate_bps > 0 && balance_minor <= i64::from(min_credit_cdf) * 100
}

pub fn is_low(balance_minor: i64, low_credit_cdf: i32) -> bool {
    balance_minor < i64::from(low_credit_cdf) * 100
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_rate_is_never_disabled() {
        assert!(!is_disabled(0, 0, 0));
    }

    #[test]
    fn rate_above_zero_with_no_credit_is_disabled() {
        assert!(is_disabled(1_000, 0, 0));
        assert!(is_disabled(1_000, -100, 0));
        assert!(!is_disabled(1_000, 1, 0));
    }

    #[test]
    fn low_balance_is_below_the_threshold() {
        assert!(is_low(4_999_99, 50_000));
        assert!(!is_low(5_000_000, 50_000));
    }
}
