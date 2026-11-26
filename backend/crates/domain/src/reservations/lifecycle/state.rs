use kernel::{Error, Result};
use serde_json::json;

use crate::reservations::codes::{RESERVATION_STATE_INVALID, RESERVATION_STATE_UNKNOWN};

pub const PENDING_PAYMENT: &str = "PENDING_PAYMENT";
pub const REQUESTED: &str = "REQUESTED";
pub const CONFIRMED: &str = "CONFIRMED";
pub const DECLINED: &str = "DECLINED";
pub const CANCELLED: &str = "CANCELLED";
pub const EXPIRED: &str = "EXPIRED";
pub const CHECKED_IN: &str = "CHECKED_IN";
pub const CHECKED_OUT: &str = "CHECKED_OUT";
pub const NO_SHOW: &str = "NO_SHOW";

pub const RESERVATION_STATES: [&str; 9] = [
    PENDING_PAYMENT,
    REQUESTED,
    CONFIRMED,
    DECLINED,
    CANCELLED,
    EXPIRED,
    CHECKED_IN,
    CHECKED_OUT,
    NO_SHOW,
];

pub fn can_move(from: &str, to: &str) -> bool {
    matches!(
        (from, to),
        (PENDING_PAYMENT, CANCELLED)
            | (PENDING_PAYMENT, EXPIRED)
            | (REQUESTED, CONFIRMED)
            | (REQUESTED, DECLINED)
            | (REQUESTED, CANCELLED)
            | (REQUESTED, EXPIRED)
            | (CONFIRMED, CANCELLED)
            | (CONFIRMED, CHECKED_IN)
            | (CONFIRMED, NO_SHOW)
            | (CHECKED_IN, CHECKED_OUT)
    )
}

pub fn ensure_move(from: &str, to: &str) -> Result<()> {
    if can_move(from, to) {
        Ok(())
    } else {
        Err(Error::conflict(RESERVATION_STATE_INVALID).with_params(json!({ "state": from, "target": to })))
    }
}

pub fn ensure_known(state: &str) -> Result<()> {
    if RESERVATION_STATES.contains(&state) {
        Ok(())
    } else {
        Err(Error::bad_request(RESERVATION_STATE_UNKNOWN).with_params(json!({ "state": state })))
    }
}

pub fn invalid_state(state: &str) -> Error {
    Error::conflict(RESERVATION_STATE_INVALID).with_params(json!({ "state": state }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requests_resolve_to_confirmed_declined_cancelled_or_expired() {
        for to in [CONFIRMED, DECLINED, CANCELLED, EXPIRED] {
            assert!(can_move(REQUESTED, to));
        }
        assert!(!can_move(REQUESTED, CHECKED_IN));
    }

    #[test]
    fn a_confirmed_stay_arrives_cancels_or_no_shows() {
        assert!(can_move(CONFIRMED, CHECKED_IN));
        assert!(can_move(CONFIRMED, CANCELLED));
        assert!(can_move(CONFIRMED, NO_SHOW));
        assert!(!can_move(CONFIRMED, CHECKED_OUT));
    }

    #[test]
    fn terminal_states_never_move() {
        for from in [DECLINED, CANCELLED, EXPIRED, CHECKED_OUT, NO_SHOW] {
            for to in RESERVATION_STATES {
                assert!(!can_move(from, to));
            }
        }
    }

    #[test]
    fn unknown_states_are_rejected() {
        assert!(ensure_known("DONE").is_err());
        assert!(ensure_known(NO_SHOW).is_ok());
    }
}
