use chrono::{DateTime, FixedOffset, NaiveDate, Utc};

pub const SUPPORTED_TIMEZONES: [&str; 2] = ["Africa/Kinshasa", "Africa/Lubumbashi"];

pub fn offset_for_timezone(timezone: &str) -> Option<FixedOffset> {
    match timezone {
        "Africa/Kinshasa" => FixedOffset::east_opt(60 * 60),
        "Africa/Lubumbashi" => FixedOffset::east_opt(2 * 60 * 60),
        _ => None,
    }
}

pub fn to_local(timezone: &str, instant: DateTime<Utc>) -> Option<DateTime<FixedOffset>> {
    offset_for_timezone(timezone).map(|offset| instant.with_timezone(&offset))
}

pub fn local_date(timezone: &str, instant: DateTime<Utc>) -> Option<NaiveDate> {
    to_local(timezone, instant).map(|local| local.date_naive())
}

pub fn format_local(timezone: &str, instant: DateTime<Utc>) -> Option<String> {
    to_local(timezone, instant).map(|local| local.format("%d/%m/%Y %H:%M").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::city::City;
    use chrono::TimeZone;

    #[test]
    fn every_supported_timezone_has_an_offset() {
        for timezone in SUPPORTED_TIMEZONES {
            assert!(offset_for_timezone(timezone).is_some());
        }
    }

    #[test]
    fn unknown_timezone_has_no_offset() {
        assert!(offset_for_timezone("Europe/Paris").is_none());
    }

    #[test]
    fn every_city_timezone_is_supported() {
        for city in City::ALL {
            assert!(SUPPORTED_TIMEZONES.contains(&city.timezone()));
        }
    }

    #[test]
    fn local_date_rolls_over_per_city_offset() {
        let instant = Utc.with_ymd_and_hms(2026, 11, 25, 23, 30, 0).unwrap();
        assert_eq!(local_date(City::GOMA.timezone(), instant), NaiveDate::from_ymd_opt(2026, 11, 26));
        assert_eq!(local_date(City::KIN.timezone(), instant), NaiveDate::from_ymd_opt(2026, 11, 26));
        let earlier = Utc.with_ymd_and_hms(2026, 11, 25, 22, 30, 0).unwrap();
        assert_eq!(local_date(City::KIN.timezone(), earlier), NaiveDate::from_ymd_opt(2026, 11, 25));
        assert_eq!(local_date(City::GOMA.timezone(), earlier), NaiveDate::from_ymd_opt(2026, 11, 26));
    }

    #[test]
    fn formats_day_first_in_the_city_zone() {
        let instant = Utc.with_ymd_and_hms(2026, 11, 25, 8, 5, 0).unwrap();
        assert_eq!(format_local("Africa/Lubumbashi", instant).unwrap(), "25/11/2026 10:05");
    }
}
