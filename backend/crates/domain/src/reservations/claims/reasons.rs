use kernel::{Error, Result};
use serde_json::json;

use crate::reservations::codes::{CLAIM_PARTIAL_AMOUNT_INVALID, CLAIM_REASON_UNKNOWN, CLAIM_STATUS_UNKNOWN, PAYOUT_STATE_UNKNOWN};

pub const STATUSES: [&str; 4] = ["UNDECIDED", "APPROVED", "PARTIAL", "REJECTED"];

pub fn reasons_for(decision: &str) -> Result<&'static [&'static str]> {
    match decision {
        "APPROVED" => Ok(&["DAMAGE_CONFIRMED", "EVIDENCE_SUFFICIENT"]),
        "PARTIAL" => Ok(&["PARTIAL_EVIDENCE", "WEAR_AND_TEAR", "AMOUNT_ADJUSTED"]),
        "REJECTED" => Ok(&["NO_EVIDENCE", "PRE_EXISTING_DAMAGE", "GUEST_DISPUTE_UPHELD"]),
        other => Err(Error::bad_request(CLAIM_STATUS_UNKNOWN).with_params(json!({ "value": other }))),
    }
}

pub fn ensure_reason(decision: &str, reason: &str) -> Result<()> {
    if reasons_for(decision)?.contains(&reason) {
        Ok(())
    } else {
        Err(Error::bad_request(CLAIM_REASON_UNKNOWN).with_params(json!({ "value": reason })))
    }
}

pub fn ensure_status(status: Option<&str>) -> Result<()> {
    match status {
        Some(status) if !STATUSES.contains(&status) => {
            Err(Error::bad_request(CLAIM_STATUS_UNKNOWN).with_params(json!({ "value": status })))
        }
        _ => Ok(()),
    }
}

pub fn approved_minor(decision: &str, claim_minor: i64, requested: Option<i64>) -> Result<i64> {
    match decision {
        "APPROVED" => Ok(claim_minor),
        "REJECTED" => Ok(0),
        _ => match requested {
            Some(amount) if amount > 0 && amount < claim_minor => Ok(amount),
            _ => Err(Error::bad_request(CLAIM_PARTIAL_AMOUNT_INVALID)),
        },
    }
}

pub fn ensure_payout_state(state: &str) -> Result<()> {
    if ["PENDING", "PAID", "FAILED"].contains(&state) {
        Ok(())
    } else {
        Err(Error::bad_request(PAYOUT_STATE_UNKNOWN).with_params(json!({ "value": state })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_amount_stays_between_zero_and_the_claim() {
        assert_eq!(approved_minor("PARTIAL", 1000, Some(400)).unwrap(), 400);
        assert!(approved_minor("PARTIAL", 1000, Some(1000)).is_err());
        assert!(approved_minor("PARTIAL", 1000, None).is_err());
        assert_eq!(approved_minor("APPROVED", 1000, None).unwrap(), 1000);
        assert_eq!(approved_minor("REJECTED", 1000, Some(5)).unwrap(), 0);
    }

    #[test]
    fn reasons_belong_to_their_decision() {
        assert!(ensure_reason("REJECTED", "NO_EVIDENCE").is_ok());
        assert!(ensure_reason("APPROVED", "NO_EVIDENCE").is_err());
    }

    #[test]
    fn payout_states_and_statuses_are_closed_lists() {
        assert!(ensure_payout_state("PAID").is_ok());
        assert!(ensure_payout_state("NOT_DUE").is_err());
        assert!(ensure_status(Some("UNDECIDED")).is_ok());
        assert!(ensure_status(Some("OTHER")).is_err());
    }
}
