use kernel::city::City;
use kernel::error::{Error, Result};
use kernel::money::CURRENCY;
use kernel::storage::Storage;
use sqlx::PgPool;

use super::queries;
use super::types::{AvailabilityOptionView, AvailabilityQuery, AvailabilityView};
use crate::guests::messages::{AVAILABILITY_DATES_REQUIRED, PROPERTY_NOT_FOUND};
use crate::guests::search::params::{date, number};
use crate::guests::search::{run_search, RawSearch, SearchQuery};

pub async fn availability(
    pool: &PgPool,
    storage: &Storage,
    city: City,
    property_id: &str,
    input: &AvailabilityQuery,
) -> Result<AvailabilityView> {
    if !queries::property_is_listed(pool, city, property_id).await? {
        return Err(Error::not_found(PROPERTY_NOT_FOUND));
    }
    let raw = RawSearch {
        property_id: Some(property_id.to_string()),
        check_in: date(&input.check_in),
        check_out: date(&input.check_out),
        adults: number(&input.adults),
        children: number(&input.children),
        ..RawSearch::default()
    };
    let query = SearchQuery::from_raw(raw)?;
    let (check_in, check_out, nights) = match (query.check_in, query.check_out, query.window()) {
        (Some(check_in), Some(check_out), Some(window)) => (check_in, check_out, window.nights),
        _ => return Err(Error::bad_request(AVAILABILITY_DATES_REQUIRED)),
    };
    let response = run_search(pool, storage, city, "en", query, false).await?;
    let options = response
        .items
        .into_iter()
        .flat_map(|hit| hit.offers)
        .map(|offer| AvailabilityOptionView {
            unit_type_id: offer.unit_type_id,
            rate_plan_id: offer.rate_plan_id,
            booking_mode: offer.booking_mode,
            meal_plan: offer.meal_plan,
            remaining_rooms: offer.remaining_rooms,
            total: offer.total,
            cancellation: offer.cancellation,
        })
        .collect();
    Ok(AvailabilityView {
        property_id: property_id.to_string(),
        check_in,
        check_out,
        nights,
        currency: CURRENCY.to_string(),
        options,
    })
}
