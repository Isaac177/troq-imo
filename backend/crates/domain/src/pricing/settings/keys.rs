use kernel::{Error, Result};
use serde_json::json;

use crate::pricing::messages::{SETTING_OUT_OF_RANGE, SETTING_UNKNOWN};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingKey {
    DefaultCommissionBps,
    QuoteTtlMinutes,
    ApprovalWindowHours,
    AmendmentWindowHours,
    ClaimWindowDays,
    CaseResponseDays,
    MinCreditCdf,
    LowCreditCdf,
}

pub const ALL_KEYS: [SettingKey; 8] = [
    SettingKey::DefaultCommissionBps,
    SettingKey::QuoteTtlMinutes,
    SettingKey::ApprovalWindowHours,
    SettingKey::AmendmentWindowHours,
    SettingKey::ClaimWindowDays,
    SettingKey::CaseResponseDays,
    SettingKey::MinCreditCdf,
    SettingKey::LowCreditCdf,
];

pub const UPDATE_REASONS: [&str; 3] = ["PLATFORM_POLICY", "MARKET_CONDITIONS", "ERROR_CORRECTED"];

impl SettingKey {
    pub fn code(self) -> &'static str {
        match self {
            Self::DefaultCommissionBps => "DEFAULT_COMMISSION_BPS",
            Self::QuoteTtlMinutes => "QUOTE_TTL_MINUTES",
            Self::ApprovalWindowHours => "APPROVAL_WINDOW_HOURS",
            Self::AmendmentWindowHours => "AMENDMENT_WINDOW_HOURS",
            Self::ClaimWindowDays => "CLAIM_WINDOW_DAYS",
            Self::CaseResponseDays => "CASE_RESPONSE_DAYS",
            Self::MinCreditCdf => "MIN_CREDIT_CDF",
            Self::LowCreditCdf => "LOW_CREDIT_CDF",
        }
    }

    pub fn parse(code: &str) -> Result<Self> {
        ALL_KEYS
            .into_iter()
            .find(|key| key.code() == code)
            .ok_or_else(|| Error::bad_request(SETTING_UNKNOWN).with_params(json!({ "key": code })))
    }

    pub fn default_value(self) -> i32 {
        match self {
            Self::DefaultCommissionBps => 0,
            Self::QuoteTtlMinutes => 15,
            Self::ApprovalWindowHours => 24,
            Self::AmendmentWindowHours => 24,
            Self::ClaimWindowDays => 7,
            Self::CaseResponseDays => 7,
            Self::MinCreditCdf => 0,
            Self::LowCreditCdf => 50_000,
        }
    }

    pub fn range(self) -> (i32, i32) {
        match self {
            Self::DefaultCommissionBps => (0, 10_000),
            Self::QuoteTtlMinutes => (5, 120),
            Self::ApprovalWindowHours | Self::AmendmentWindowHours => (1, 168),
            Self::ClaimWindowDays | Self::CaseResponseDays => (1, 60),
            Self::MinCreditCdf | Self::LowCreditCdf => (0, 100_000_000),
        }
    }

    pub fn unit(self) -> &'static str {
        match self {
            Self::DefaultCommissionBps => "BPS",
            Self::QuoteTtlMinutes => "MINUTES",
            Self::ApprovalWindowHours | Self::AmendmentWindowHours => "HOURS",
            Self::ClaimWindowDays | Self::CaseResponseDays => "DAYS",
            Self::MinCreditCdf | Self::LowCreditCdf => "CDF",
        }
    }

    pub fn ensure_in_range(self, value: i32) -> Result<()> {
        let (min, max) = self.range();
        if (min..=max).contains(&value) {
            Ok(())
        } else {
            Err(Error::bad_request(SETTING_OUT_OF_RANGE)
                .with_params(json!({ "key": self.code(), "min": min, "max": max })))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_default_sits_inside_its_range() {
        for key in ALL_KEYS {
            assert!(key.ensure_in_range(key.default_value()).is_ok());
        }
    }

    #[test]
    fn codes_round_trip() {
        for key in ALL_KEYS {
            assert_eq!(SettingKey::parse(key.code()).unwrap(), key);
        }
        assert!(SettingKey::parse("NOPE").is_err());
    }

    #[test]
    fn out_of_range_values_are_rejected() {
        assert!(SettingKey::QuoteTtlMinutes.ensure_in_range(4).is_err());
        assert!(SettingKey::DefaultCommissionBps.ensure_in_range(10_001).is_err());
        assert!(SettingKey::ClaimWindowDays.ensure_in_range(60).is_ok());
    }
}
