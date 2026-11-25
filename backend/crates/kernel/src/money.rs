use std::fmt;

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::codes::AMOUNT_INVALID;
use crate::error::Error;

pub const CURRENCY: &str = "CDF";

const SCALE: i64 = 100;
const PERCENT_SCALE: i128 = 10_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Money(i64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoneyError {
    Negative(String),
    TooPrecise(String),
    Invalid(String),
    OutOfRange(String),
}

impl fmt::Display for MoneyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Negative(value) => write!(formatter, "negative amount {value}"),
            Self::TooPrecise(value) => write!(formatter, "amount {value} exceeds two decimals"),
            Self::Invalid(value) => write!(formatter, "invalid amount {value}"),
            Self::OutOfRange(value) => write!(formatter, "amount {value} out of range"),
        }
    }
}

impl std::error::Error for MoneyError {}

impl From<MoneyError> for Error {
    fn from(error: MoneyError) -> Self {
        Error::internal(error.to_string())
    }
}

impl Money {
    pub const ZERO: Money = Money(0);

    pub fn from_minor(minor: i64) -> Self {
        Self(minor)
    }

    pub fn minor(self) -> i64 {
        self.0
    }

    pub fn parse(value: &str) -> Result<Self, MoneyError> {
        let trimmed = value.trim();
        if trimmed.starts_with('-') {
            return Err(MoneyError::Negative(trimmed.to_string()));
        }
        let (whole, fraction) = trimmed.split_once('.').unwrap_or((trimmed, ""));
        if fraction.len() > 2 {
            return Err(MoneyError::TooPrecise(trimmed.to_string()));
        }
        let invalid = || MoneyError::Invalid(trimmed.to_string());
        let whole: i64 = whole.parse().map_err(|_| invalid())?;
        let cents: i64 = if fraction.is_empty() {
            0
        } else {
            format!("{fraction:0<2}").parse().map_err(|_| invalid())?
        };
        whole
            .checked_mul(SCALE)
            .and_then(|value| value.checked_add(cents))
            .map(Self)
            .ok_or_else(|| MoneyError::OutOfRange(trimmed.to_string()))
    }

    pub fn parse_input(value: &str) -> Result<Self, Error> {
        Self::parse(value).map_err(|error| {
            Error::bad_request(AMOUNT_INVALID).with_params(serde_json::json!({ "value": value, "reason": error.to_string() }))
        })
    }

    pub fn to_wire(self) -> String {
        format!("{}.{:02}", self.0 / SCALE, self.0 % SCALE)
    }

    pub fn percent_of(self, percent_minor: i64) -> Self {
        Self((i128::from(self.0) * i128::from(percent_minor) / PERCENT_SCALE) as i64)
    }

    pub fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
    }

    pub fn checked_sub(self, other: Self) -> Option<Self> {
        self.0.checked_sub(other.0).map(Self)
    }

    pub fn checked_mul(self, factor: i64) -> Option<Self> {
        self.0.checked_mul(factor).map(Self)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_wire())
    }
}

impl Serialize for Money {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_wire())
    }
}

impl<'de> Deserialize<'de> for Money {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Money::parse(&value).map_err(D::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_two_decimal_amounts_exactly() {
        assert_eq!(Money::parse("180000.00").unwrap().minor(), 18_000_000);
        assert_eq!(Money::parse("0.5").unwrap().minor(), 50);
        assert_eq!(Money::parse("12").unwrap().minor(), 1_200);
    }

    #[test]
    fn rejects_negative_and_over_precise_amounts() {
        assert!(Money::parse("-1.00").is_err());
        assert!(Money::parse("1.005").is_err());
    }

    #[test]
    fn formats_back_to_two_decimals() {
        assert_eq!(Money::from_minor(20_500_000).to_wire(), "205000.00");
        assert_eq!(Money::from_minor(5).to_wire(), "0.05");
    }

    #[test]
    fn percentage_uses_hundredths_of_a_percent() {
        assert_eq!(Money::from_minor(18_000_000).percent_of(1_000).minor(), 1_800_000);
    }

    #[test]
    fn wire_form_is_a_decimal_string() {
        assert_eq!(serde_json::to_string(&Money::from_minor(150_000)).unwrap(), "\"1500.00\"");
        assert_eq!(serde_json::from_str::<Money>("\"1500.00\"").unwrap().minor(), 150_000);
    }

    #[test]
    fn user_input_failures_are_bad_requests() {
        assert_eq!(Money::parse_input("x").unwrap_err().code().as_str(), "AMOUNT_INVALID");
    }
}
