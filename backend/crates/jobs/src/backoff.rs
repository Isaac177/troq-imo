use std::time::Duration;

pub const MAX_ATTEMPTS: i32 = 12;
const BASE_SECONDS: u64 = 5;
const CEILING_SECONDS: u64 = 3600;

pub fn retry_delay(attempts: i32) -> Duration {
    let exponent = attempts.saturating_sub(1).clamp(0, 20) as u32;
    let seconds = BASE_SECONDS
        .saturating_mul(1u64 << exponent)
        .min(CEILING_SECONDS);
    Duration::from_secs(seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_retry_waits_the_base_delay_then_doubles() {
        assert_eq!(retry_delay(1), Duration::from_secs(5));
        assert_eq!(retry_delay(2), Duration::from_secs(10));
        assert_eq!(retry_delay(3), Duration::from_secs(20));
    }

    #[test]
    fn delay_never_exceeds_one_hour() {
        assert_eq!(retry_delay(MAX_ATTEMPTS), Duration::from_secs(3600));
        assert_eq!(retry_delay(500), Duration::from_secs(3600));
    }

    #[test]
    fn zero_or_negative_attempts_use_the_base_delay() {
        assert_eq!(retry_delay(0), Duration::from_secs(5));
        assert_eq!(retry_delay(-3), Duration::from_secs(5));
    }
}
