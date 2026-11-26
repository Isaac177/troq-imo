use kernel::error::{Error, Result};

use super::types::CategoryInput;
use crate::properties::messages::{CATEGORY_CODE_REQUIRED, CATEGORY_ICON_INVALID, CATEGORY_NAME_INVALID};

const MAX_NAME_LENGTH: usize = 80;
const MAX_CODE_LENGTH: usize = 48;
const MAX_GENERATED_LENGTH: usize = 40;

pub struct ValidatedCategory {
    pub name_en: String,
    pub name_fr: String,
    pub icon: String,
}

fn name(value: &str) -> Result<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.chars().count() > MAX_NAME_LENGTH {
        return Err(Error::bad_request(CATEGORY_NAME_INVALID));
    }
    Ok(trimmed.to_string())
}

fn icon(value: &str) -> Result<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.len() > MAX_CODE_LENGTH || !trimmed.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(Error::bad_request(CATEGORY_ICON_INVALID));
    }
    Ok(trimmed.to_string())
}

pub fn validate(input: &CategoryInput) -> Result<ValidatedCategory> {
    Ok(ValidatedCategory {
        name_en: name(&input.name_en)?,
        name_fr: name(&input.name_fr)?,
        icon: icon(&input.icon)?,
    })
}

pub fn code_from_name(value: &str) -> Result<String> {
    let mut code = String::new();
    for character in value.trim().chars() {
        if character.is_ascii_alphanumeric() {
            code.push(character.to_ascii_uppercase());
        } else if !code.ends_with('_') && !code.is_empty() {
            code.push('_');
        }
    }
    let code: String = code.trim_end_matches('_').chars().take(MAX_GENERATED_LENGTH).collect();
    let code = code.trim_end_matches('_').to_string();
    if code.is_empty() {
        return Err(Error::bad_request(CATEGORY_NAME_INVALID));
    }
    Ok(code)
}

pub fn existing_code(value: &str) -> Result<String> {
    let trimmed = value.trim();
    let valid = !trimmed.is_empty()
        && trimmed.len() <= MAX_CODE_LENGTH
        && trimmed.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');
    if valid {
        Ok(trimmed.to_string())
    } else {
        Err(Error::bad_request(CATEGORY_CODE_REQUIRED))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_upper_snake_codes_from_names() {
        assert_eq!(code_from_name(" Guest house ").unwrap(), "GUEST_HOUSE");
        assert_eq!(code_from_name("Wi-Fi 6!").unwrap(), "WI_FI_6");
        assert!(code_from_name("---").is_err());
    }

    #[test]
    fn icons_are_alphanumeric_and_names_bounded() {
        assert!(icon("BedDouble").is_ok());
        assert!(icon("bed-double").is_err());
        assert!(name(&"x".repeat(81)).is_err());
    }

    #[test]
    fn existing_codes_must_be_upper_snake() {
        assert!(existing_code("HOTEL_ROOM").is_ok());
        assert!(existing_code("hotel room").is_err());
    }
}
