use chrono::Utc;
use kernel::city::City;
use kernel::error::{Error, Result};
use sqlx::PgPool;

use super::queries;
use super::types::{CalendarDayView, CalendarQuery, CalendarView};
use crate::guests::collaborators::remaining_by_night;
use crate::guests::messages::{CALENDAR_RANGE_INVALID, PROPERTY_NOT_FOUND, UNIT_TYPE_NOT_FOUND};

pub const MAX_CALENDAR_DAYS: i64 = 186;

pub fn check_span(from: chrono::NaiveDate, to: chrono::NaiveDate) -> Result<()> {
    let span = (to - from).num_days();
    if span < 1 || span > MAX_CALENDAR_DAYS {
        return Err(Error::bad_request(CALENDAR_RANGE_INVALID)
            .with_params(serde_json::json!({ "max": MAX_CALENDAR_DAYS })));
    }
    Ok(())
}

pub async fn calendar(pool: &PgPool, city: City, property_id: &str, input: &CalendarQuery) -> Result<CalendarView> {
    check_span(input.from, input.to)?;
    if !queries::property_is_listed(pool, city, property_id).await? {
        return Err(Error::not_found(PROPERTY_NOT_FOUND));
    }
    let units = queries::active_unit_types(pool, property_id, "en").await?;
    if !units.iter().any(|unit| unit.id == input.unit_type_id) {
        return Err(Error::not_found(UNIT_TYPE_NOT_FOUND));
    }
    let remaining = remaining_by_night(pool, &input.unit_type_id, input.from, input.to).await?;
    let today = Utc::now().date_naive();
    let days = input
        .from
        .iter_days()
        .take_while(|day| *day < input.to)
        .map(|date| CalendarDayView {
            date,
            available: date >= today && remaining.get(&date).copied().unwrap_or(0) > 0,
        })
        .collect();
    Ok(CalendarView { unit_type_id: input.unit_type_id.clone(), days })
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;

    fn day(text: &str) -> NaiveDate {
        NaiveDate::parse_from_str(text, "%Y-%m-%d").unwrap()
    }

    #[test]
    fn the_calendar_covers_one_to_186_days() {
        assert!(check_span(day("2040-01-01"), day("2040-01-01")).is_err());
        assert!(check_span(day("2040-01-01"), day("2040-01-02")).is_ok());
        assert!(check_span(day("2040-01-01"), day("2040-07-05")).is_ok());
        assert!(check_span(day("2040-01-01"), day("2040-07-06")).is_err());
    }
}
