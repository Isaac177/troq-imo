use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::codes::CITY_UNSUPPORTED;
use crate::error::{Error, Result};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct CityInfo {
    pub code: &'static str,
    pub name: &'static str,
    pub timezone: &'static str,
}

const GOMA_INFO: CityInfo = CityInfo {
    code: "GOMA",
    name: "Goma",
    timezone: "Africa/Lubumbashi",
};

const KIN_INFO: CityInfo = CityInfo {
    code: "KIN",
    name: "Kinshasa",
    timezone: "Africa/Kinshasa",
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct City(&'static CityInfo);

impl City {
    pub const GOMA: City = City(&GOMA_INFO);
    pub const KIN: City = City(&KIN_INFO);
    pub const ALL: [City; 2] = [City::KIN, City::GOMA];

    pub fn code(self) -> &'static str {
        self.0.code
    }

    pub fn name(self) -> &'static str {
        self.0.name
    }

    pub fn timezone(self) -> &'static str {
        self.0.timezone
    }

    pub fn from_code(value: &str) -> Result<Self> {
        let code = value.trim().to_ascii_uppercase();
        City::ALL
            .into_iter()
            .find(|city| city.code() == code)
            .ok_or_else(|| {
                Error::bad_request(CITY_UNSUPPORTED).with_params(serde_json::json!({ "city": code }))
            })
    }
}

impl fmt::Display for City {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::str::FromStr for City {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        Self::from_code(value)
    }
}

impl Serialize for City {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.code())
    }
}

impl<'de> Deserialize<'de> for City {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        City::from_code(&value).map_err(serde::de::Error::custom)
    }
}

pub fn ensure_resource_in_route_city(route: City, resource: City) -> Result<()> {
    if route == resource {
        Ok(())
    } else {
        Err(Error::city_mismatch(route.code(), resource.code()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_city_codes_case_insensitively() {
        assert_eq!("goma".parse::<City>().unwrap(), City::GOMA);
        assert_eq!(" KIN ".parse::<City>().unwrap(), City::KIN);
    }

    #[test]
    fn rejects_unknown_city() {
        assert!("LUBUMBASHI".parse::<City>().is_err());
    }

    #[test]
    fn same_city_is_accepted() {
        assert!(ensure_resource_in_route_city(City::KIN, City::KIN).is_ok());
    }

    #[test]
    fn cross_city_resource_is_rejected() {
        assert!(ensure_resource_in_route_city(City::GOMA, City::KIN).is_err());
    }

    #[test]
    fn city_data_carries_name_and_timezone() {
        assert_eq!(City::GOMA.name(), "Goma");
        assert_eq!(City::KIN.timezone(), "Africa/Kinshasa");
    }

    #[test]
    fn serialises_as_the_code() {
        assert_eq!(serde_json::to_string(&City::GOMA).unwrap(), "\"GOMA\"");
        assert_eq!(serde_json::from_str::<City>("\"kin\"").unwrap(), City::KIN);
    }
}
