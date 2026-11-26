use kernel::error::{Error, Result};
use serde_json::json;

use super::types::{MAX_ID_CHARS, MAX_NAME_CHARS, MAX_PROPERTY_FILTER};
use crate::guests::messages::{
    WISHLIST_FILTER_INVALID, WISHLIST_NAME_REQUIRED, WISHLIST_NAME_TOO_LONG,
    WISHLIST_VISIBILITY_INVALID,
};

pub fn clean_name(name: &str) -> Result<String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err(Error::bad_request(WISHLIST_NAME_REQUIRED));
    }
    if trimmed.chars().count() > MAX_NAME_CHARS {
        return Err(Error::bad_request(WISHLIST_NAME_TOO_LONG).with_params(json!({ "max": MAX_NAME_CHARS })));
    }
    Ok(trimmed.to_string())
}

pub fn clean_visibility(value: Option<&str>) -> Result<String> {
    match value.unwrap_or("PRIVATE") {
        "PRIVATE" => Ok("PRIVATE".into()),
        "SHARED" => Ok("SHARED".into()),
        other => Err(Error::bad_request(WISHLIST_VISIBILITY_INVALID).with_params(json!({ "value": other }))),
    }
}

pub fn parse_property_ids(value: &str) -> Result<Vec<String>> {
    let ids: Vec<String> = value
        .split(',')
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(str::to_owned)
        .collect();
    if ids.is_empty() || ids.len() > MAX_PROPERTY_FILTER || ids.iter().any(|id| id.len() > MAX_ID_CHARS) {
        return Err(Error::bad_request(WISHLIST_FILTER_INVALID));
    }
    Ok(ids)
}
