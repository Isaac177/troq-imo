use chrono::NaiveTime;
use kernel::error::{Error, ErrorCode, Result};
use kernel::timezone::{offset_for_timezone, SUPPORTED_TIMEZONES};
use serde_json::json;

use super::messages::{
    AMENITY_MALFORMED, CHOICE_INVALID, CLOCK_INVALID, COORDINATES_INCOMPLETE,
    COORDINATES_OUT_OF_RANGE, EMAIL_INVALID, FIELD_REQUIRED, FIELD_TOO_LONG, PHONE_INVALID,
    TIMEZONE_UNSUPPORTED,
};

pub const MAX_NAME_LENGTH: usize = 200;
pub const MAX_ADDRESS_LENGTH: usize = 255;
pub const MAX_DESCRIPTION_LENGTH: usize = 5000;
pub const MAX_NEIGHBORHOOD_LENGTH: usize = 120;
pub const MAX_HOUSE_RULES_LENGTH: usize = 2000;
const MAX_CONTACT_PHONE_LENGTH: usize = 32;
const MAX_CONTACT_EMAIL_LENGTH: usize = 254;
const MIN_PHONE_DIGITS: usize = 6;
const MAX_PHONE_DIGITS: usize = 15;
const CLOCK_FORMAT: &str = "%H:%M";

pub fn field_error(code: ErrorCode, field: &str) -> Error {
    Error::bad_request(code).with_params(json!({ "field": field }))
}

pub fn required_text(field: &str, value: &str, max: usize) -> Result<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(field_error(FIELD_REQUIRED, field));
    }
    bounded(field, trimmed, max)
}

pub fn bounded(field: &str, value: &str, max: usize) -> Result<String> {
    let trimmed = value.trim();
    if trimmed.chars().count() > max {
        return Err(field_error(FIELD_TOO_LONG, field).with_params(json!({ "field": field, "max": max })));
    }
    Ok(trimmed.to_string())
}

pub fn optional_text(field: &str, value: Option<String>, max: usize) -> Result<Option<String>> {
    match value.map(|text| text.trim().to_string()).filter(|text| !text.is_empty()) {
        Some(text) => bounded(field, &text, max).map(Some),
        None => Ok(None),
    }
}

pub fn timezone(value: &str) -> Result<String> {
    let trimmed = value.trim().to_string();
    if offset_for_timezone(&trimmed).is_none() {
        return Err(Error::bad_request(TIMEZONE_UNSUPPORTED).with_params(json!({ "allowed": SUPPORTED_TIMEZONES })));
    }
    Ok(trimmed)
}

pub fn coordinates(latitude: Option<f64>, longitude: Option<f64>) -> Result<Option<(f64, f64)>> {
    match (latitude, longitude) {
        (None, None) => Ok(None),
        (Some(lat), Some(lng)) if (-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&lng) => {
            Ok(Some((lat, lng)))
        }
        (Some(_), Some(_)) => Err(Error::bad_request(COORDINATES_OUT_OF_RANGE)),
        _ => Err(Error::bad_request(COORDINATES_INCOMPLETE)),
    }
}

pub fn clock(field: &str, value: Option<String>) -> Result<Option<String>> {
    match value.map(|text| text.trim().to_string()).filter(|text| !text.is_empty()) {
        None => Ok(None),
        Some(text) => NaiveTime::parse_from_str(&text, CLOCK_FORMAT)
            .map(|time| Some(time.format(CLOCK_FORMAT).to_string()))
            .map_err(|_| field_error(CLOCK_INVALID, field)),
    }
}

pub fn choice(field: &str, value: Option<String>, allowed: &[&str]) -> Result<Option<String>> {
    match value.map(|text| text.trim().to_string()).filter(|text| !text.is_empty()) {
        None => Ok(None),
        Some(text) if allowed.contains(&text.as_str()) => Ok(Some(text)),
        Some(_) => Err(field_error(CHOICE_INVALID, field).with_params(json!({ "field": field, "allowed": allowed }))),
    }
}

pub fn phone(value: &str) -> Result<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(String::new());
    }
    let characters_ok = trimmed
        .chars()
        .all(|c| c.is_ascii_digit() || matches!(c, '+' | ' ' | '-' | '(' | ')'));
    let digits = trimmed.chars().filter(char::is_ascii_digit).count();
    if characters_ok
        && trimmed.chars().count() <= MAX_CONTACT_PHONE_LENGTH
        && (MIN_PHONE_DIGITS..=MAX_PHONE_DIGITS).contains(&digits)
    {
        Ok(trimmed.to_string())
    } else {
        Err(Error::bad_request(PHONE_INVALID))
    }
}

pub fn email(value: &str) -> Result<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(String::new());
    }
    let well_formed = trimmed.chars().count() <= MAX_CONTACT_EMAIL_LENGTH
        && !trimmed.chars().any(char::is_whitespace)
        && trimmed.split_once('@').is_some_and(|(local, domain)| {
            !local.is_empty() && domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
        });
    if well_formed {
        Ok(trimmed.to_string())
    } else {
        Err(Error::bad_request(EMAIL_INVALID))
    }
}

pub fn amenity_codes(values: Vec<String>) -> Result<Vec<String>> {
    let mut codes: Vec<String> = Vec::new();
    for value in values {
        let code = value.trim().to_string();
        let valid = !code.is_empty()
            && code.len() <= 48
            && code.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');
        if !valid {
            return Err(Error::bad_request(AMENITY_MALFORMED).with_params(json!({ "code": code })));
        }
        if !codes.contains(&code) {
            codes.push(code);
        }
    }
    Ok(codes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_and_bounds_text() {
        assert_eq!(required_text("name", " Villa ", 10).unwrap(), "Villa");
        assert!(required_text("name", "  ", 10).is_err());
        assert!(bounded("name", &"x".repeat(11), 10).is_err());
    }

    #[test]
    fn timezone_must_be_supported() {
        assert!(timezone(" Africa/Lubumbashi ").is_ok());
        assert!(timezone("Europe/Paris").is_err());
    }

    #[test]
    fn coordinates_come_in_pairs_within_range() {
        assert_eq!(coordinates(None, None).unwrap(), None);
        assert!(coordinates(Some(-1.6), None).is_err());
        assert!(coordinates(Some(91.0), Some(0.0)).is_err());
        assert!(coordinates(Some(-1.6), Some(29.2)).is_ok());
    }

    #[test]
    fn clock_is_normalised() {
        assert_eq!(clock("checkInFrom", Some("9:05".into())).unwrap().as_deref(), Some("09:05"));
        assert!(clock("checkInFrom", Some("25:00".into())).is_err());
        assert_eq!(clock("checkInFrom", Some("  ".into())).unwrap(), None);
    }

    #[test]
    fn contacts_follow_legacy_rules() {
        assert!(phone("+243 800 000 001").is_ok());
        assert!(phone("call me").is_err());
        assert_eq!(phone("  ").unwrap(), "");
        assert!(email("stays@example.com").is_ok());
        assert!(email("stays@example").is_err());
    }

    #[test]
    fn amenity_codes_are_unique_and_well_formed() {
        assert_eq!(amenity_codes(vec!["WIFI".into(), " WIFI ".into()]).unwrap(), vec!["WIFI"]);
        assert!(amenity_codes(vec!["not valid".into()]).is_err());
    }
}
