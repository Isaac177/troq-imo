use std::collections::BTreeSet;

use kernel::{Error, Result};
use serde_json::{json, Value};

use crate::reservations::codes::{
    GUEST_CONTACT_REQUIRED, GUEST_NAME_REQUIRED, POLICY_ACCEPTANCE_MISMATCH, POLICY_ACCEPTANCE_REQUIRED,
    SPECIAL_REQUEST_TOO_LONG,
};
use crate::reservations::lifecycle::types::GuestInput;
use crate::reservations::support::trimmed;

pub const MAX_SPECIAL_REQUEST_CHARS: usize = 500;

pub fn policy_ids(policies: &Value) -> BTreeSet<String> {
    policies
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.get("id").and_then(Value::as_str).map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

pub fn validate_acceptance(accepted: &[String], quote_policy_ids: &BTreeSet<String>) -> Result<Vec<String>> {
    let accepted_set: BTreeSet<String> = accepted.iter().cloned().collect();
    if accepted_set.is_empty() {
        return Err(Error::bad_request(POLICY_ACCEPTANCE_REQUIRED));
    }
    if &accepted_set != quote_policy_ids {
        return Err(Error::bad_request(POLICY_ACCEPTANCE_MISMATCH));
    }
    Ok(accepted_set.into_iter().collect())
}

pub fn clean_special_request(text: Option<&str>) -> Result<Option<String>> {
    let Some(text) = text.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(None);
    };
    if text.chars().count() > MAX_SPECIAL_REQUEST_CHARS {
        return Err(Error::bad_request(SPECIAL_REQUEST_TOO_LONG).with_params(json!({ "max": MAX_SPECIAL_REQUEST_CHARS })));
    }
    Ok(Some(text.to_string()))
}

pub fn validate_guest(guest: &GuestInput) -> Result<()> {
    if guest.full_name.trim().is_empty() {
        return Err(Error::bad_request(GUEST_NAME_REQUIRED));
    }
    if trimmed(&guest.contact_phone).is_none() && trimmed(&guest.contact_email).is_none() {
        return Err(Error::bad_request(GUEST_CONTACT_REQUIRED));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids(values: &[&str]) -> BTreeSet<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn policy_ids_are_read_from_the_quote_snapshot() {
        let policies = json!([
            { "id": "policy-b", "kind": "PAYMENT", "content": {} },
            { "id": "policy-a", "kind": "CANCELLATION", "content": {} }
        ]);
        assert_eq!(policy_ids(&policies), ids(&["policy-a", "policy-b"]));
        assert!(policy_ids(&Value::Null).is_empty());
    }

    #[test]
    fn acceptance_must_match_the_quote_exactly() {
        let quote = ids(&["policy-a", "policy-b"]);
        assert!(validate_acceptance(&[], &quote).is_err());
        assert!(validate_acceptance(&["policy-a".into()], &quote).is_err());
        assert!(validate_acceptance(&["policy-a".into(), "policy-b".into(), "policy-c".into()], &quote).is_err());
        let accepted = validate_acceptance(&["policy-b".into(), "policy-a".into(), "policy-a".into()], &quote).unwrap();
        assert_eq!(accepted, vec!["policy-a".to_string(), "policy-b".to_string()]);
    }

    #[test]
    fn special_request_is_trimmed_and_bounded() {
        assert_eq!(clean_special_request(None).unwrap(), None);
        assert_eq!(clean_special_request(Some("   ")).unwrap(), None);
        assert_eq!(clean_special_request(Some("  late arrival  ")).unwrap().as_deref(), Some("late arrival"));
        assert!(clean_special_request(Some(&"x".repeat(MAX_SPECIAL_REQUEST_CHARS + 1))).is_err());
    }

    #[test]
    fn guests_need_a_name_and_one_contact() {
        let guest = |name: &str, phone: Option<&str>, email: Option<&str>| GuestInput {
            full_name: name.into(),
            contact_phone: phone.map(str::to_string),
            contact_email: email.map(str::to_string),
        };
        assert!(validate_guest(&guest(" ", Some("+243"), None)).is_err());
        assert!(validate_guest(&guest("Ada", Some(" "), None)).is_err());
        assert!(validate_guest(&guest("Ada", None, Some("a@b.cd"))).is_ok());
    }
}
