use kernel::error::{codes, Error, Result};

use super::reservation_types::ReservationFilter;

pub struct Filters<'a> {
    pub state: Option<&'a str>,
    pub host: Option<&'a str>,
    pub property: Option<&'a str>,
    pub search: Option<String>,
}

fn non_empty(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}

pub fn validated(filter: &ReservationFilter) -> Result<Filters<'_>> {
    let state = non_empty(filter.state.as_deref());
    if state.is_some_and(|value| !crate::reservations::RESERVATION_STATES.contains(&value))
        || matches!((filter.check_in_from, filter.check_in_to), (Some(from), Some(to)) if to < from)
    {
        return Err(Error::bad_request(codes::MALFORMED_REQUEST));
    }
    let search = non_empty(filter.search.as_deref())
        .map(|value| format!("%{}%", value.replace('\\', "\\\\").replace('%', "\\%").replace('_', "\\_")));
    Ok(Filters { state, host: non_empty(filter.host_id.as_deref()), property: non_empty(filter.property_id.as_deref()), search })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_search_wildcards_and_rejects_unknown_state() {
        let query = ReservationFilter { search: Some("_x%".into()), ..Default::default() };
        assert_eq!(validated(&query).unwrap().search.as_deref(), Some("%\\_x\\%%"));
        let query = ReservationFilter { state: Some("UNKNOWN".into()), ..Default::default() };
        assert!(validated(&query).is_err());
    }
}
