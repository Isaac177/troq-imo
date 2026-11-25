mod env;

use std::net::SocketAddr;
use std::str::FromStr;

use anyhow::{anyhow, Result};

use crate::city::City;
use crate::storage::StorageConfig;

pub use env::{optional, optional_positive, parsed_or, required};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OtpChannel {
    Email,
    Whatsapp,
}

impl FromStr for OtpChannel {
    type Err = String;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "email" => Ok(Self::Email),
            "whatsapp" => Ok(Self::Whatsapp),
            other => Err(format!("unsupported otp channel {other}")),
        }
    }
}

#[derive(Clone)]
pub struct Config {
    pub production: bool,
    pub api_port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub cities: Vec<City>,
    pub default_city: City,
    pub jwt_secret: String,
    pub jwt_refresh_secret: String,
    pub otp_channel: OtpChannel,
    pub guest_retention_days: Option<i64>,
    pub storage: StorageConfig,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let production = optional("APP_ENV").as_deref() == Some("production");
        if !production {
            dotenvy::dotenv().ok();
        }
        let cities = parse_cities(&parsed_or("CITY_CODES", "KIN,GOMA".to_string())?)?;
        let default_city = cities
            .iter()
            .copied()
            .find(|city| *city == City::GOMA)
            .unwrap_or(cities[0]);
        Ok(Self {
            production,
            api_port: parsed_or("API_PORT", 8080)?,
            database_url: required("DATABASE_URL")?,
            redis_url: required("REDIS_URL")?,
            cities,
            default_city,
            jwt_secret: required("JWT_SECRET")?,
            jwt_refresh_secret: required("JWT_REFRESH_SECRET")?,
            otp_channel: parsed_or("AUTH_OTP_CHANNEL", OtpChannel::Email)?,
            guest_retention_days: optional_positive("GUEST_RETENTION_DAYS")?,
            storage: StorageConfig::from_env()?,
        })
    }

    pub fn bind_addr(&self) -> SocketAddr {
        SocketAddr::from(([0, 0, 0, 0], self.api_port))
    }

    pub fn serves(&self, city: City) -> bool {
        self.cities.contains(&city)
    }
}

pub fn parse_cities(codes: &str) -> Result<Vec<City>> {
    let mut cities = Vec::new();
    for code in codes.split(',').map(str::trim).filter(|code| !code.is_empty()) {
        let city = City::from_code(code).map_err(|_| anyhow!("unsupported city code {code}"))?;
        if !cities.contains(&city) {
            cities.push(city);
        }
    }
    if cities.is_empty() {
        return Err(anyhow!("CITY_CODES must list at least one city"));
    }
    Ok(cities)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_city_codes_and_drops_duplicates() {
        let cities = parse_cities("KIN, goma,KIN").unwrap();
        assert_eq!(cities, vec![City::KIN, City::GOMA]);
    }

    #[test]
    fn rejects_empty_or_unknown_city_lists() {
        assert!(parse_cities("").is_err());
        assert!(parse_cities("PARIS").is_err());
    }

    #[test]
    fn parses_otp_channels() {
        assert_eq!("Email".parse::<OtpChannel>().unwrap(), OtpChannel::Email);
        assert_eq!("whatsapp".parse::<OtpChannel>().unwrap(), OtpChannel::Whatsapp);
        assert!("sms".parse::<OtpChannel>().is_err());
    }
}
