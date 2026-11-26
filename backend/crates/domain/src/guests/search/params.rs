use chrono::NaiveDate;
use serde::Deserialize;
use utoipa::IntoParams;

use super::raw::RawSearch;

#[derive(Debug, Default, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct SearchParams {
    pub property_id: Option<String>,
    pub search: Option<String>,
    pub check_in: Option<String>,
    pub check_out: Option<String>,
    pub adults: Option<String>,
    pub children: Option<String>,
    pub child_ages: Option<String>,
    pub instant_booking: Option<String>,
    pub property_types: Option<String>,
    pub meal_plans: Option<String>,
    pub payment_collections: Option<String>,
    pub accessibility: Option<String>,
    pub amenities: Option<String>,
    pub categories: Option<String>,
    pub price_min: Option<String>,
    pub price_max: Option<String>,
    pub min_free_cancellation_hours: Option<String>,
    pub max_penalty_percent: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub max_distance_km: Option<String>,
    pub sort: Option<String>,
    pub page: Option<String>,
    pub page_size: Option<String>,
    pub analytics_consent: Option<String>,
    pub locale: Option<String>,
}

pub(crate) fn text(value: &Option<String>) -> Option<String> {
    value
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

pub(crate) fn number<T: std::str::FromStr>(value: &Option<String>) -> Option<T> {
    value.as_deref().and_then(|value| value.trim().parse::<T>().ok())
}

pub(crate) fn date(value: &Option<String>) -> Option<NaiveDate> {
    value
        .as_deref()
        .and_then(|value| NaiveDate::parse_from_str(value.trim(), "%Y-%m-%d").ok())
}

fn flag(value: &Option<String>) -> bool {
    matches!(value.as_deref(), Some("true" | "1"))
}

fn list(value: &Option<String>) -> Vec<String> {
    value
        .as_deref()
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn int_list(value: &Option<String>) -> Vec<i32> {
    value
        .as_deref()
        .map(|value| value.split(',').filter_map(|item| item.trim().parse().ok()).collect())
        .unwrap_or_default()
}

impl SearchParams {
    pub fn into_raw(self) -> RawSearch {
        RawSearch {
            property_id: text(&self.property_id),
            search: text(&self.search),
            check_in: date(&self.check_in),
            check_out: date(&self.check_out),
            adults: number(&self.adults),
            children: number(&self.children),
            child_ages: int_list(&self.child_ages),
            instant_booking: flag(&self.instant_booking),
            property_types: list(&self.property_types),
            meal_plans: list(&self.meal_plans),
            payment_collections: list(&self.payment_collections),
            accessibility: list(&self.accessibility),
            amenities: list(&self.amenities),
            categories: list(&self.categories),
            price_min: text(&self.price_min),
            price_max: text(&self.price_max),
            min_free_cancellation_hours: number(&self.min_free_cancellation_hours),
            max_penalty_percent: number(&self.max_penalty_percent),
            latitude: number(&self.latitude),
            longitude: number(&self.longitude),
            max_distance_km: number(&self.max_distance_km),
            sort: text(&self.sort),
            page: number(&self.page),
            page_size: number(&self.page_size),
            analytics_consent: flag(&self.analytics_consent),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_numbers_and_flags_follow_the_gateway_rules() {
        let params = SearchParams {
            amenities: Some("WIFI, POOL,,".into()),
            child_ages: Some("3, x ,7".into()),
            adults: Some("2".into()),
            children: Some("abc".into()),
            instant_booking: Some("1".into()),
            check_in: Some("2040-03-01".into()),
            check_out: Some("not a date".into()),
            ..SearchParams::default()
        };
        let raw = params.into_raw();
        assert_eq!(raw.amenities, vec!["WIFI", "POOL"]);
        assert_eq!(raw.child_ages, vec![3, 7]);
        assert_eq!(raw.adults, Some(2));
        assert_eq!(raw.children, None);
        assert!(raw.instant_booking);
        assert!(raw.check_in.is_some() && raw.check_out.is_none());
    }
}
