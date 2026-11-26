use serde_json::json;

use super::validators::check_parameters;

#[test]
fn parameters_must_be_an_object() {
    assert!(check_parameters(&json!({ "city": "GOMA", "adults": 2 })).is_ok());
    assert!(check_parameters(&json!(["GOMA"])).is_err());
    assert!(check_parameters(&json!("GOMA")).is_err());
}

#[test]
fn parameters_are_limited_to_two_kilobytes() {
    let large = json!({ "text": "x".repeat(2100) });
    assert!(check_parameters(&large).is_err());
}
