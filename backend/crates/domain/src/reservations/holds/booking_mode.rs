use chrono::{DateTime, Duration, Utc};
use kernel::{Error, Result};

use crate::reservations::lifecycle::state::{CONFIRMED, REQUESTED};

pub fn initial_state(booking_mode: &str) -> Result<&'static str> {
    match booking_mode {
        "INSTANT" => Ok(CONFIRMED),
        "REQUEST" => Ok(REQUESTED),
        other => Err(Error::internal(format!("unknown booking mode {other}"))),
    }
}

pub fn approval_deadline(booking_mode: &str, now: DateTime<Utc>, window_hours: i64) -> Result<Option<DateTime<Utc>>> {
    match booking_mode {
        "INSTANT" => Ok(None),
        "REQUEST" => Ok(Some(now + Duration::hours(window_hours))),
        other => Err(Error::internal(format!("unknown booking mode {other}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn instant_bookings_are_confirmed_at_creation() {
        assert_eq!(initial_state("INSTANT").unwrap(), "CONFIRMED");
        let now = Utc.with_ymd_and_hms(2026, 10, 12, 10, 0, 0).unwrap();
        assert_eq!(approval_deadline("INSTANT", now, 24).unwrap(), None);
    }

    #[test]
    fn request_bookings_wait_for_approval_for_the_default_window() {
        assert_eq!(initial_state("REQUEST").unwrap(), "REQUESTED");
        let now = Utc.with_ymd_and_hms(2026, 10, 12, 10, 0, 0).unwrap();
        assert_eq!(approval_deadline("REQUEST", now, 24).unwrap(), Some(now + Duration::hours(24)));
    }

    #[test]
    fn unknown_booking_mode_is_rejected() {
        assert!(initial_state("BOTH").is_err());
        assert!(approval_deadline("BOTH", Utc::now(), 24).is_err());
    }
}
