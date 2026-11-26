use chrono::NaiveDate;
use kernel::{Error, Result};
use serde_json::json;

use crate::reservations::codes::{
    CASE_CLOSED, CASE_REASON_REQUIRED, CASE_REASON_TOO_LONG, CASE_TYPE_NEEDS_PAYMENT_RULES, CASE_TYPE_NOT_ALLOWED,
    CLAIM_AMOUNT_INVALID, CLAIM_AMOUNT_TOO_HIGH, CLAIM_BEFORE_CHECKOUT, CLAIM_WINDOW_CLOSED,
};

pub const DISPUTE_WINDOW_DAYS: i64 = 7;
pub const MAX_REASON_LENGTH: usize = 2000;
pub const OPENABLE_BY_GUEST: [&str; 2] = ["GUEST_COMPLAINT", "GENERAL"];
pub const OPENABLE_BY_OPERATOR: [&str; 3] = ["OPERATOR_COMPLAINT", "GENERAL", "DAMAGE_CLAIM"];
pub const CASE_STATES: [&str; 4] = ["OPEN", "AWAITING_RESPONSE", "RESOLVED", "CLOSED"];
pub const CASE_TYPES: [&str; 6] = [
    "GUEST_COMPLAINT",
    "OPERATOR_COMPLAINT",
    "DAMAGE_CLAIM",
    "REFUND_DISPUTE",
    "NO_SHOW_DISPUTE",
    "GENERAL",
];
pub const PLATFORM_CASE_TYPES: [&str; 3] = ["GUEST_COMPLAINT", "OPERATOR_COMPLAINT", "GENERAL"];

pub fn ensure_openable(case_type: &str, by_guest: bool) -> Result<()> {
    let allowed: &[&str] = if by_guest { &OPENABLE_BY_GUEST } else { &OPENABLE_BY_OPERATOR };
    if allowed.contains(&case_type) {
        return Ok(());
    }
    if matches!(case_type, "REFUND_DISPUTE" | "NO_SHOW_DISPUTE") {
        return Err(Error::conflict(CASE_TYPE_NEEDS_PAYMENT_RULES).with_params(json!({ "caseType": case_type })));
    }
    Err(Error::bad_request(CASE_TYPE_NOT_ALLOWED).with_params(json!({ "caseType": case_type })))
}

pub fn ensure_claim_window(check_out: NaiveDate, today: NaiveDate, window_days: i64) -> Result<()> {
    let elapsed = (today - check_out).num_days();
    if elapsed < 0 {
        return Err(Error::conflict(CLAIM_BEFORE_CHECKOUT));
    }
    if elapsed > window_days {
        return Err(Error::conflict(CLAIM_WINDOW_CLOSED).with_params(json!({ "days": window_days })));
    }
    Ok(())
}

pub fn ensure_claim_amount(claim_minor: i64, total_minor: i64) -> Result<()> {
    if claim_minor <= 0 {
        return Err(Error::bad_request(CLAIM_AMOUNT_INVALID));
    }
    if claim_minor > total_minor {
        return Err(Error::bad_request(CLAIM_AMOUNT_TOO_HIGH));
    }
    Ok(())
}

pub fn ensure_closable(state: &str) -> Result<()> {
    match state {
        "OPEN" | "AWAITING_RESPONSE" => Ok(()),
        other => Err(Error::conflict(CASE_CLOSED).with_params(json!({ "state": other }))),
    }
}

pub fn validate_reason(reason: &str) -> Result<String> {
    let trimmed = reason.trim();
    if trimmed.is_empty() {
        return Err(Error::bad_request(CASE_REASON_REQUIRED));
    }
    if trimmed.chars().count() > MAX_REASON_LENGTH {
        return Err(Error::bad_request(CASE_REASON_TOO_LONG).with_params(json!({ "max": MAX_REASON_LENGTH })));
    }
    Ok(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guests_can_open_complaints_and_general_cases() {
        assert!(ensure_openable("GUEST_COMPLAINT", true).is_ok());
        assert!(ensure_openable("GENERAL", true).is_ok());
    }

    #[test]
    fn guests_cannot_open_operator_complaints() {
        assert!(ensure_openable("OPERATOR_COMPLAINT", true).is_err());
    }

    #[test]
    fn operators_open_damage_claims_and_refund_disputes_stay_refused() {
        assert!(ensure_openable("DAMAGE_CLAIM", false).is_ok());
        assert!(ensure_openable("DAMAGE_CLAIM", true).is_err());
        assert!(matches!(ensure_openable("REFUND_DISPUTE", true).unwrap_err(), Error::Conflict(_)));
    }

    #[test]
    fn damage_claims_follow_the_window_and_amount_rules() {
        let out = NaiveDate::from_ymd_opt(2026, 11, 5).unwrap();
        assert!(ensure_claim_window(out, out, 7).is_ok());
        assert!(ensure_claim_window(out, out + chrono::Duration::days(7), 7).is_ok());
        assert!(ensure_claim_window(out, out + chrono::Duration::days(8), 7).is_err());
        assert!(ensure_claim_window(out, out - chrono::Duration::days(1), 7).is_err());
        assert!(ensure_claim_amount(1, 100).is_ok());
        assert!(ensure_claim_amount(0, 100).is_err());
        assert!(ensure_claim_amount(101, 100).is_err());
    }

    #[test]
    fn only_open_cases_can_close() {
        assert!(ensure_closable("OPEN").is_ok());
        assert!(ensure_closable("AWAITING_RESPONSE").is_ok());
        assert!(ensure_closable("CLOSED").is_err());
        assert!(ensure_closable("RESOLVED").is_err());
    }

    #[test]
    fn reason_is_trimmed_and_bounded() {
        assert_eq!(validate_reason("  Late check-in  ").unwrap(), "Late check-in");
        assert!(validate_reason("   ").is_err());
        assert!(validate_reason(&"x".repeat(MAX_REASON_LENGTH + 1)).is_err());
    }
}
