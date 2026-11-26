use kernel::error::{Error, Result};

use super::codes::{
    REVIEW_HIDE_REASON_REQUIRED, REVIEW_MODERATION_STATUS_INVALID, REVIEW_RATING_INVALID,
    REVIEW_STATUS_INVALID, REVIEW_TEXT_REQUIRED, REVIEW_TEXT_TOO_LONG,
};
use super::types::{HIDDEN, HIDE_REASON_CODES, MODERATION_STATUSES, REVIEW_STATUSES};

pub const COMMENT_LIMIT: usize = 2000;
pub const REPLY_LIMIT: usize = 2000;

pub fn rating(value: i16) -> Result<i16> {
    if (1..=5).contains(&value) {
        Ok(value)
    } else {
        Err(Error::bad_request(REVIEW_RATING_INVALID))
    }
}

pub fn required_text(value: String, limit: usize) -> Result<String> {
    let trimmed = bounded(value, limit)?;
    if trimmed.is_empty() {
        return Err(Error::bad_request(REVIEW_TEXT_REQUIRED));
    }
    Ok(trimmed)
}

pub fn optional_text(value: Option<String>, limit: usize) -> Result<Option<String>> {
    let Some(value) = value else {
        return Ok(None);
    };
    let trimmed = bounded(value, limit)?;
    Ok((!trimmed.is_empty()).then_some(trimmed))
}

fn bounded(value: String, limit: usize) -> Result<String> {
    let trimmed = value.trim().to_string();
    if trimmed.chars().count() > limit {
        return Err(Error::bad_request(REVIEW_TEXT_TOO_LONG)
            .with_params(serde_json::json!({ "limit": limit })));
    }
    Ok(trimmed)
}

pub fn review_status(value: &str) -> Result<&str> {
    if REVIEW_STATUSES.contains(&value) {
        Ok(value)
    } else {
        Err(Error::bad_request(REVIEW_STATUS_INVALID)
            .with_params(serde_json::json!({ "status": value })))
    }
}

pub fn optional_review_status(value: Option<&str>) -> Result<Option<&str>> {
    value.map(review_status).transpose()
}

pub fn moderation_status(value: &str) -> Result<&str> {
    if MODERATION_STATUSES.contains(&value) {
        Ok(value)
    } else {
        Err(Error::bad_request(REVIEW_MODERATION_STATUS_INVALID))
    }
}

pub fn hide_reason<'a>(status: &str, reason_code: Option<&'a str>) -> Result<Option<&'a str>> {
    match (status, reason_code) {
        (HIDDEN, Some(code)) if HIDE_REASON_CODES.contains(&code) => Ok(Some(code)),
        (HIDDEN, _) => Err(Error::bad_request(REVIEW_HIDE_REASON_REQUIRED)),
        _ => Ok(None),
    }
}

pub fn clean_search(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}

pub fn escape_like(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rating_accepts_one_to_five_only() {
        assert!(rating(1).is_ok());
        assert!(rating(5).is_ok());
        assert_eq!(rating(0).unwrap_err().code().as_str(), "REVIEW_RATING_INVALID");
        assert!(rating(6).is_err());
    }

    #[test]
    fn required_text_is_trimmed_and_non_empty() {
        assert_eq!(required_text("  thanks ".into(), 10).unwrap(), "thanks");
        assert_eq!(required_text("   ".into(), 10).unwrap_err().code().as_str(), "REVIEW_TEXT_REQUIRED");
    }

    #[test]
    fn text_is_bounded_by_characters() {
        assert!(required_text("é".repeat(10), 10).is_ok());
        assert_eq!(required_text("é".repeat(11), 10).unwrap_err().code().as_str(), "REVIEW_TEXT_TOO_LONG");
    }

    #[test]
    fn optional_text_drops_blank_values() {
        assert_eq!(optional_text(None, 5).unwrap(), None);
        assert_eq!(optional_text(Some("  ".into()), 5).unwrap(), None);
        assert_eq!(optional_text(Some(" ok ".into()), 5).unwrap(), Some("ok".to_string()));
        assert!(optional_text(Some("toolong".into()), 5).is_err());
    }

    #[test]
    fn statuses_are_validated() {
        assert!(review_status("PENDING").is_ok());
        assert_eq!(review_status("REJECTED").unwrap_err().code().as_str(), "REVIEW_STATUS_INVALID");
        assert!(optional_review_status(None).unwrap().is_none());
        assert!(moderation_status("APPROVED").is_ok());
        assert!(moderation_status("PENDING").is_err());
    }

    #[test]
    fn hiding_needs_a_known_reason_code() {
        assert_eq!(hide_reason("HIDDEN", Some("SPAM")).unwrap(), Some("SPAM"));
        assert!(hide_reason("HIDDEN", Some("RUDE")).is_err());
        assert!(hide_reason("HIDDEN", None).is_err());
        assert_eq!(hide_reason("APPROVED", Some("SPAM")).unwrap(), None);
    }

    #[test]
    fn like_patterns_are_escaped() {
        assert_eq!(escape_like("50%_a\\"), "50\\%\\_a\\\\");
        assert_eq!(clean_search(Some("  ")), None);
    }
}
