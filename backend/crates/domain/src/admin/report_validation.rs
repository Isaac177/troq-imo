use kernel::error::{codes, Error, Result};
use super::report_types::AnalyticsQuery;

pub fn validate(query: &AnalyticsQuery) -> Result<()> {
    if query.host_id.trim().is_empty() || query.property_id.as_ref().is_some_and(|id| id.trim().is_empty()) {
        return Err(Error::bad_request(codes::MALFORMED_REQUEST));
    }
    if query.from.zip(query.to).is_some_and(|(from, to)| from > to) {
        return Err(Error::bad_request(codes::MALFORMED_REQUEST));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn rejects_empty_ids_and_inverted_dates() {
        let mut query = AnalyticsQuery { host_id: "host".into(), property_id: None, from: None, to: None };
        assert!(validate(&query).is_ok());
        query.host_id = " ".into();
        assert!(validate(&query).is_err());
        query.host_id = "host".into();
        query.from = NaiveDate::from_ymd_opt(2026, 12, 2);
        query.to = NaiveDate::from_ymd_opt(2026, 12, 1);
        assert!(validate(&query).is_err());
    }
}
