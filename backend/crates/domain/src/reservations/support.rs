use chrono::{DateTime, NaiveDate, Utc};
use kernel::{Error, Money, PageRequest, Result};

pub fn parse_minor(value: &str) -> Result<i64> {
    Ok(Money::parse(value)?.minor())
}

pub fn wire_minor(minor: i64) -> String {
    Money::from_minor(minor).to_wire()
}

pub fn signed_wire_minor(minor: i64) -> String {
    let sign = if minor < 0 { "-" } else { "" };
    format!("{sign}{}", wire_minor(minor.abs()))
}

pub fn today_in(timezone: &str, now: DateTime<Utc>) -> Result<NaiveDate> {
    kernel::timezone::local_date(timezone, now).ok_or_else(|| Error::internal("property timezone has no offset"))
}

pub fn paging(page: Option<u32>, page_size: Option<u32>) -> Result<PageRequest> {
    let default = PageRequest::default();
    PageRequest {
        page: page.unwrap_or(default.page),
        page_size: page_size.unwrap_or(default.page_size),
    }
    .validated()
}

pub fn escape_like(value: &str) -> String {
    value.replace('\\', "\\\\").replace('%', "\\%").replace('_', "\\_")
}

pub fn trimmed(value: &Option<String>) -> Option<String> {
    value
        .as_ref()
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty())
}

pub fn nights_between(check_in: NaiveDate, check_out: NaiveDate) -> Vec<NaiveDate> {
    check_in.iter_days().take_while(|day| *day < check_out).collect()
}

pub fn unique_violation(error: &sqlx::Error, constraint: &str) -> bool {
    matches!(error, sqlx::Error::Database(database)
        if database.code().as_deref() == Some("23505") && database.constraint() == Some(constraint))
}

pub fn system_actor(city: kernel::City, name: &str) -> kernel::audit::AuditActor {
    kernel::audit::AuditActor {
        user_id: kernel::ids::UserId::new(format!("system:{name}")),
        city,
        permissions: Vec::new(),
        request_id: kernel::ids::new_event_id(),
    }
}

pub fn new_reference(city: kernel::City) -> String {
    let token = kernel::ids::new_id("ref");
    format!("STY-{}-{}", city.code(), token[4..12].to_uppercase())
}

pub fn retain_until(check_out: NaiveDate, retention_days: Option<i64>) -> NaiveDate {
    crate::guests::retain_until(retention_days, check_out)
}

pub fn city_of(code: &str) -> Result<kernel::City> {
    kernel::City::from_code(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signed_amounts_keep_two_decimals() {
        assert_eq!(signed_wire_minor(-12_550), "-125.50");
        assert_eq!(signed_wire_minor(0), "0.00");
        assert_eq!(wire_minor(410_000_00), "410000.00");
    }

    #[test]
    fn like_patterns_are_escaped() {
        assert_eq!(escape_like("50%_a"), "50\\%\\_a");
    }

    #[test]
    fn nights_exclude_the_departure_day() {
        let start = NaiveDate::from_ymd_opt(2026, 12, 30).unwrap();
        let nights = nights_between(start, NaiveDate::from_ymd_opt(2027, 1, 2).unwrap());
        assert_eq!(nights.len(), 3);
    }
}

