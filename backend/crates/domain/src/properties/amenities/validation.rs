use kernel::error::{Error, ErrorCode, Result};

pub const NAME_INVALID: ErrorCode = ErrorCode::new("AMENITY_NAME_INVALID");
pub const CODE_INVALID: ErrorCode = ErrorCode::new("AMENITY_CODE_INVALID");
pub const ICON_INVALID: ErrorCode = ErrorCode::new("AMENITY_ICON_INVALID");
pub const LOCALE_INVALID: ErrorCode = ErrorCode::new("AMENITY_LOCALE_INVALID");
pub const CODE_EXHAUSTED: ErrorCode = ErrorCode::new("AMENITY_CODE_EXHAUSTED");
pub const GROUP_NOT_FOUND: ErrorCode = ErrorCode::new("AMENITY_GROUP_NOT_FOUND");
pub const AMENITY_NOT_FOUND: ErrorCode = ErrorCode::new("AMENITY_NOT_FOUND");

pub fn name(value: &str) -> Result<String> {
    let value = value.trim();
    if value.is_empty() || value.chars().count() > 80 {
        Err(Error::bad_request(NAME_INVALID))
    } else {
        Ok(value.to_owned())
    }
}

pub fn locale(value: &str) -> Result<String> {
    let value = value.trim();
    if value.is_empty() || value.len() > 35 || !value.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        Err(Error::bad_request(LOCALE_INVALID))
    } else {
        Ok(value.to_owned())
    }
}

pub fn icon(value: &str) -> Result<String> {
    let value = value.trim();
    if value.is_empty() || value.len() > 48 || !value.chars().all(|c| c.is_ascii_alphanumeric()) {
        Err(Error::bad_request(ICON_INVALID))
    } else {
        Ok(value.to_owned())
    }
}

pub fn code(value: &str) -> Result<String> {
    let value = value.trim();
    if value.is_empty() || value.len() > 48 || !value.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_') {
        Err(Error::bad_request(CODE_INVALID))
    } else {
        Ok(value.to_owned())
    }
}

pub fn code_from_name(value: &str) -> Result<String> {
    let mut code = String::new();
    for c in value.trim().chars() {
        if c.is_ascii_alphanumeric() {
            code.push(c.to_ascii_uppercase());
        } else if !code.is_empty() && !code.ends_with('_') {
            code.push('_');
        }
    }
    let truncated: String = code.trim_end_matches('_').chars().take(40).collect();
    let result = truncated.trim_end_matches('_');
    if result.is_empty() { Err(Error::bad_request(CODE_INVALID)) } else { Ok(result.to_owned()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_and_icons_keep_legacy_bounds() {
        assert_eq!(name(" Lake ").unwrap(), "Lake");
        assert!(name(&"x".repeat(81)).is_err());
        assert_eq!(icon(" Wifi ").unwrap(), "Wifi");
        assert!(icon("wi-fi").is_err());
    }

    #[test]
    fn generated_codes_match_legacy_rules() {
        assert_eq!(code_from_name(" Fire extinguisher ").unwrap(), "FIRE_EXTINGUISHER");
        assert_eq!(code_from_name("Wi-Fi 6!").unwrap(), "WI_FI_6");
        assert!(code_from_name("---").is_err());
        assert!(code("lowercase").is_err());
    }
}
