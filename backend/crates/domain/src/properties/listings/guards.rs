use kernel::error::{Error, Result};
use serde_json::json;

use crate::properties::messages::{LISTING_ARCHIVED, LISTING_STATUS_INVALID, LISTING_VERSION_CONFLICT};
use crate::properties::property::types::PropertyView;

pub fn ensure_version(current: i32, expected: i32) -> Result<()> {
    if current == expected {
        Ok(())
    } else {
        Err(Error::conflict(LISTING_VERSION_CONFLICT).with_params(json!({ "current": current })))
    }
}

pub fn ensure_status(current: &str, allowed: &[&str], action: &str) -> Result<()> {
    if allowed.contains(&current) {
        Ok(())
    } else {
        Err(Error::conflict(LISTING_STATUS_INVALID).with_params(json!({ "status": current, "action": action })))
    }
}

pub fn ensure_not_archived(property: &PropertyView) -> Result<()> {
    if property.status == "ARCHIVED" {
        Err(Error::conflict(LISTING_ARCHIVED))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_must_match_exactly() {
        assert!(ensure_version(3, 3).is_ok());
        assert!(ensure_version(4, 3).is_err());
    }

    #[test]
    fn status_must_be_in_the_allowed_set() {
        assert!(ensure_status("DRAFT", &["DRAFT", "CHANGES_REQUESTED"], "submit").is_ok());
        assert!(ensure_status("SUBMITTED", &["DRAFT"], "submit").is_err());
    }
}
