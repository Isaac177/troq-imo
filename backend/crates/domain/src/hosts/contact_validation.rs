use kernel::error::{Error, Result};

use super::messages::{CONTACT_EMAIL_INVALID, CONTACT_PHONE_INVALID};

pub fn phone(value: &str) -> Result<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(String::new());
    }
    let allowed = trimmed.chars().all(|c| c.is_ascii_digit() || matches!(c, '+' | ' ' | '-' | '(' | ')'));
    let digits = trimmed.chars().filter(char::is_ascii_digit).count();
    if allowed && trimmed.chars().count() <= 32 && (6..=15).contains(&digits) {
        Ok(trimmed.to_owned())
    } else {
        Err(Error::bad_request(CONTACT_PHONE_INVALID))
    }
}

pub fn email(value: &str) -> Result<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(String::new());
    }
    let valid = trimmed.chars().count() <= 254
        && !trimmed.chars().any(char::is_whitespace)
        && trimmed.split_once('@').is_some_and(|(local, domain)| {
            !local.is_empty() && domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
        });
    if valid {
        Ok(trimmed.to_owned())
    } else {
        Err(Error::bad_request(CONTACT_EMAIL_INVALID))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_blank_and_trims_valid_contact_values() {
        assert_eq!(phone("  ").unwrap(), "");
        assert_eq!(phone(" +243 800 000 001 ").unwrap(), "+243 800 000 001");
        assert_eq!(email(" stays@example.com ").unwrap(), "stays@example.com");
    }

    #[test]
    fn rejects_malformed_contacts() {
        assert!(phone("call me").is_err());
        assert!(phone("12345").is_err());
        assert!(email("stays@example").is_err());
        assert!(email("a b@example.com").is_err());
    }
}
