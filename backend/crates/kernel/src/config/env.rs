use std::env;
use std::fmt::Display;
use std::str::FromStr;

use anyhow::{anyhow, Result};

pub fn optional(key: &str) -> Option<String> {
    env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

pub fn required(key: &str) -> Result<String> {
    optional(key).ok_or_else(|| anyhow!("missing environment variable {key}"))
}

pub fn parsed_or<T>(key: &str, default: T) -> Result<T>
where
    T: FromStr,
    T::Err: Display,
{
    match optional(key) {
        Some(value) => value
            .parse::<T>()
            .map_err(|error| anyhow!("invalid {key}: {error}")),
        None => Ok(default),
    }
}

pub fn optional_positive(key: &str) -> Result<Option<i64>> {
    match optional(key) {
        Some(value) => {
            let days: i64 = value
                .parse()
                .map_err(|error| anyhow!("invalid {key}: {error}"))?;
            if days < 1 {
                return Err(anyhow!("{key} must be at least 1"));
            }
            Ok(Some(days))
        }
        None => Ok(None),
    }
}
