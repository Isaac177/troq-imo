use kernel::error::{Error, Result};
use serde_json::{json, Value};

use super::types::MAX_PARAMETER_BYTES;
use crate::guests::messages::{SAVED_SEARCH_PARAMETERS_INVALID, SAVED_SEARCH_TOO_LARGE};

pub fn check_parameters(parameters: &Value) -> Result<()> {
    if !parameters.is_object() {
        return Err(Error::bad_request(SAVED_SEARCH_PARAMETERS_INVALID));
    }
    if parameters.to_string().len() > MAX_PARAMETER_BYTES {
        return Err(Error::bad_request(SAVED_SEARCH_TOO_LARGE).with_params(json!({ "maxBytes": MAX_PARAMETER_BYTES })));
    }
    Ok(())
}
