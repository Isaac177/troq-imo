use chrono::{NaiveDate, Utc};
use kernel::auth::Actor;
use kernel::error::{Error, Result};
use kernel::permissions::names::PROPERTIES_READ;
use kernel::timezone::local_date;
use serde_json::json;
use sqlx::PgPool;

use super::types::{AvailabilityQuery, AvailabilityView, NightAvailability};
use crate::inventory::messages::{AVAILABILITY_RANGE_INVALID, DATE_RANGE_TOO_LONG, UNIT_TYPE_NOT_FOUND};
use crate::inventory::nights::remaining_by_night;
use crate::inventory::restrictions::{violations, StayRequest};
use crate::inventory::scope::{host_property, unit_type_belongs};

const MAX_SEARCH_NIGHTS: i64 = 365;

fn nights_between(check_in: NaiveDate, check_out: NaiveDate) -> Vec<NaiveDate> {
    check_in.iter_days().take_while(|night| *night < check_out).collect()
}

pub async fn get_availability(
    pool: &PgPool,
    actor: &Actor,
    input: AvailabilityQuery,
) -> Result<AvailabilityView> {
    if input.check_out <= input.check_in {
        return Err(Error::bad_request(AVAILABILITY_RANGE_INVALID));
    }
    if (input.check_out - input.check_in).num_days() > MAX_SEARCH_NIGHTS {
        return Err(Error::bad_request(DATE_RANGE_TOO_LONG).with_params(json!({ "maxNights": MAX_SEARCH_NIGHTS })));
    }
    let property = host_property(pool, actor, &input.property_id, PROPERTIES_READ).await?;
    if !unit_type_belongs(pool, &property.id, &input.unit_type_id).await? {
        return Err(Error::not_found(UNIT_TYPE_NOT_FOUND));
    }
    let booking_day = local_date(&property.timezone, Utc::now())
        .ok_or_else(|| Error::internal("property timezone has no offset"))?;
    let remaining = remaining_by_night(pool, &input.unit_type_id, input.check_in, input.check_out).await?;
    let nights: Vec<NightAvailability> = nights_between(input.check_in, input.check_out)
        .into_iter()
        .map(|date| NightAvailability { date, remaining: remaining.get(&date).copied().unwrap_or(0).max(0) })
        .collect();
    let rule_violations = violations(
        pool,
        &property.id,
        &StayRequest {
            unit_type_id: input.unit_type_id.clone(),
            check_in: input.check_in,
            check_out: input.check_out,
            booking_day,
        },
    )
    .await?;
    let bookable = rule_violations.is_empty() && nights.iter().all(|night| night.remaining > 0);
    Ok(AvailabilityView { bookable, nights, violations: rule_violations })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nights_exclude_the_departure_day() {
        let from = NaiveDate::from_ymd_opt(2026, 12, 20).unwrap();
        let to = NaiveDate::from_ymd_opt(2026, 12, 23).unwrap();
        assert_eq!(nights_between(from, to).len(), 3);
    }
}
