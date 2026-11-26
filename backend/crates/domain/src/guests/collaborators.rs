use std::collections::HashMap;

use chrono::NaiveDate;
use kernel::city::City;
use kernel::error::Result;
use sqlx::PgConnection;

pub use crate::inventory::nights::remaining_by_night;
pub use crate::inventory::restrictions::{violations_batch, StayRequest};
pub use crate::pricing::promotions::Promotion;
pub use crate::pricing::quotes::{guest_factor, MAX_STAY_NIGHTS};
pub use crate::reservations::links::pricing::StayTerms;
pub use crate::pricing::rates::RateRule;

pub fn nightly_amount_minor(date: NaiveDate, rules: &[RateRule]) -> Option<i64> {
    crate::pricing::rates::nightly_minor(date, rules).ok()
}

pub fn stay_total_minor(nights: &[NaiveDate], terms: &StayTerms<'_>) -> Option<i64> {
    crate::pricing::quotes::charges::price_stay(nights, terms)
        .ok()
        .map(|priced| priced.total_minor)
}

pub async fn rules_for_plans(
    connection: &mut PgConnection,
    city: City,
    plan_ids: &[String],
) -> Result<HashMap<String, Vec<RateRule>>> {
    crate::pricing::rates::rules_for_plans(connection, city, plan_ids).await
}

pub async fn active_promotions_for_properties(
    connection: &mut PgConnection,
    city: City,
    property_ids: &[String],
    check_in: NaiveDate,
    check_out: NaiveDate,
) -> Result<Vec<(String, Option<String>, Promotion)>> {
    crate::pricing::promotions::active_for_properties(connection, city, property_ids, check_in, check_out).await
}
