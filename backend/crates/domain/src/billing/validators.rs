use kernel::error::codes::AMOUNT_INVALID;
use kernel::{Error, Result};
use serde_json::json;

use super::messages::{
    ADJUST_REASON_UNKNOWN, ENTRY_TYPE_UNKNOWN, NOTE_TOO_LONG, PAYMENT_REF_TOO_LONG, RATE_INVALID,
    RATE_REASON_UNKNOWN, TOPUP_REASON_UNKNOWN,
};

pub const RATE_REASONS: [&str; 5] = [
    "CONTRACT_AGREED",
    "RENEGOTIATED",
    "LAUNCH_OFFER",
    "PLATFORM_POLICY",
    "ERROR_CORRECTED",
];

pub const TOPUP_REASONS: [&str; 4] = ["CASH_RECEIVED", "MOBILE_MONEY_RECEIVED", "BANK_TRANSFER", "PROMOTIONAL_CREDIT"];

pub const ADJUST_REASONS: [&str; 3] = ["ERROR_CORRECTED", "GOODWILL", "CHARGEBACK_REVERSAL"];

pub const ENTRY_TYPES: [&str; 4] = ["TOPUP", "COMMISSION", "REFUND", "ADJUSTMENT"];

pub const PROMOTIONAL_REASON: &str = "PROMOTIONAL_CREDIT";

pub const MAX_RATE_BPS: i32 = 10_000;
pub const MAX_CREDIT_CDF: i64 = 100_000_000;
pub const MAX_NOTE_CHARS: usize = 500;
pub const MAX_REF_CHARS: usize = 120;

fn reject(code: kernel::ErrorCode, reason: &str) -> Error {
    Error::bad_request(code).with_params(json!({ "reason": reason }))
}

pub fn ensure_rate_reason(reason: &str) -> Result<()> {
    if RATE_REASONS.contains(&reason) {
        Ok(())
    } else {
        Err(reject(RATE_REASON_UNKNOWN, reason))
    }
}

pub fn ensure_topup_reason(reason: &str) -> Result<()> {
    if TOPUP_REASONS.contains(&reason) {
        Ok(())
    } else {
        Err(reject(TOPUP_REASON_UNKNOWN, reason))
    }
}

pub fn ensure_adjust_reason(reason: &str) -> Result<()> {
    if ADJUST_REASONS.contains(&reason) {
        Ok(())
    } else {
        Err(reject(ADJUST_REASON_UNKNOWN, reason))
    }
}

pub fn ensure_entry_type(entry_type: &str) -> Result<()> {
    if ENTRY_TYPES.contains(&entry_type) {
        Ok(())
    } else {
        Err(reject(ENTRY_TYPE_UNKNOWN, entry_type))
    }
}

pub fn clean(value: &Option<String>) -> Option<&str> {
    value.as_deref().map(str::trim).filter(|text| !text.is_empty())
}

pub fn check_note(note: Option<&str>) -> Result<()> {
    if note.is_some_and(|text| text.chars().count() > MAX_NOTE_CHARS) {
        return Err(Error::bad_request(NOTE_TOO_LONG).with_params(json!({ "max": MAX_NOTE_CHARS })));
    }
    Ok(())
}

pub fn check_payment_ref(payment_ref: Option<&str>) -> Result<()> {
    if payment_ref.is_some_and(|text| text.chars().count() > MAX_REF_CHARS) {
        return Err(Error::bad_request(PAYMENT_REF_TOO_LONG).with_params(json!({ "max": MAX_REF_CHARS })));
    }
    Ok(())
}

fn split_decimal(text: &str) -> Option<(i64, i64)> {
    let (whole, fraction) = text.split_once('.').unwrap_or((text, ""));
    let digits = |part: &str| part.chars().all(|c| c.is_ascii_digit());
    if whole.is_empty() || fraction.len() > 2 || !digits(whole) || !digits(fraction) {
        return None;
    }
    let whole: i64 = whole.parse().ok()?;
    let hundredths: i64 = if fraction.is_empty() { 0 } else { format!("{fraction:0<2}").parse().ok()? };
    Some((whole, hundredths))
}

pub fn parse_rate_bps(rate: &str) -> Result<i32> {
    let invalid = || Error::bad_request(RATE_INVALID).with_params(json!({ "min": 0, "max": 100, "decimals": 2 }));
    let (whole, hundredths) = split_decimal(rate.trim()).ok_or_else(invalid)?;
    let bps = whole
        .checked_mul(100)
        .and_then(|value| value.checked_add(hundredths))
        .ok_or_else(invalid)?;
    if (0..=i64::from(MAX_RATE_BPS)).contains(&bps) {
        Ok(bps as i32)
    } else {
        Err(invalid())
    }
}

pub fn parse_amount_minor(amount: &str) -> Result<i64> {
    let invalid = || {
        Error::bad_request(AMOUNT_INVALID)
            .with_params(json!({ "value": amount, "max": MAX_CREDIT_CDF, "decimals": 2 }))
    };
    let (whole, hundredths) = split_decimal(amount.trim()).ok_or_else(invalid)?;
    if whole > MAX_CREDIT_CDF {
        return Err(invalid());
    }
    let minor = whole * 100 + hundredths;
    if minor > 0 && minor <= MAX_CREDIT_CDF * 100 {
        Ok(minor)
    } else {
        Err(invalid())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rate_parses_to_hundredths_of_a_percent() {
        assert_eq!(parse_rate_bps("12.5").unwrap(), 1250);
        assert_eq!(parse_rate_bps("0").unwrap(), 0);
        assert_eq!(parse_rate_bps("100").unwrap(), 10_000);
        assert_eq!(parse_rate_bps("7.05").unwrap(), 705);
    }

    #[test]
    fn rate_rejects_out_of_range_and_over_precise_values() {
        assert!(parse_rate_bps("100.01").is_err());
        assert!(parse_rate_bps("-1").is_err());
        assert!(parse_rate_bps("1.234").is_err());
        assert!(parse_rate_bps("abc").is_err());
        assert!(parse_rate_bps("").is_err());
    }

    #[test]
    fn amount_parses_to_minor_units() {
        assert_eq!(parse_amount_minor("50000").unwrap(), 5_000_000);
        assert_eq!(parse_amount_minor("12.5").unwrap(), 1_250);
        assert_eq!(parse_amount_minor("0.01").unwrap(), 1);
    }

    #[test]
    fn amount_rejects_zero_negative_and_oversized_values() {
        assert!(parse_amount_minor("0").is_err());
        assert!(parse_amount_minor("-5").is_err());
        assert!(parse_amount_minor("100000000.01").is_err());
        assert!(parse_amount_minor("1.234").is_err());
        assert!(parse_amount_minor("").is_err());
    }

    #[test]
    fn reasons_are_checked_against_their_lists() {
        assert!(ensure_rate_reason("LAUNCH_OFFER").is_ok());
        assert!(ensure_rate_reason("OTHER").is_err());
        assert!(ensure_topup_reason("BANK_TRANSFER").is_ok());
        assert!(ensure_topup_reason("GOODWILL").is_err());
        assert!(ensure_adjust_reason("GOODWILL").is_ok());
        assert!(ensure_entry_type("REFUND").is_ok());
        assert!(ensure_entry_type("PAYOUT").is_err());
    }

    #[test]
    fn long_notes_and_references_are_rejected() {
        assert!(check_note(Some(&"a".repeat(MAX_NOTE_CHARS))).is_ok());
        assert!(check_note(Some(&"a".repeat(MAX_NOTE_CHARS + 1))).is_err());
        assert!(check_payment_ref(Some(&"a".repeat(MAX_REF_CHARS + 1))).is_err());
    }
}
