use chrono::NaiveDate;
use kernel::error::{Error, Result};
use serde::Serialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use utoipa::ToSchema;

use super::normalize::{
    normalize_code_format, normalize_codes, normalize_sort, origin_from, parse_price, window_from,
};
pub use super::normalize::cancellation_class;
pub use super::raw::RawSearch;
use crate::guests::messages::{
    SEARCH_CANCELLATION_HOURS_RANGE, SEARCH_CHILD_AGES_MISSING, SEARCH_CHILD_AGE_RANGE,
    SEARCH_DISTANCE_NEEDS_ORIGIN, SEARCH_DISTANCE_RANGE, SEARCH_GUESTS_RANGE,
    SEARCH_PAGE_OFFSET_TOO_LARGE, SEARCH_PENALTY_RANGE, SEARCH_PRICE_NEEDS_DATES,
    SEARCH_PRICE_RANGE_INVERTED, SEARCH_PROPERTY_ID_EMPTY, SEARCH_SORT_NEEDS_DATES,
    SEARCH_SORT_NEEDS_ORIGIN, SEARCH_TEXT_TOO_LONG,
};

pub const ENGINE_VERSION: u32 = 3;
pub const PUBLIC_COORDINATE_DECIMALS: i32 = 2;
pub const MAX_CHILD_AGE: i32 = 17;
pub const MAX_ADULTS: i32 = 20;
pub const MAX_CHILDREN: i32 = 10;
pub const MAX_DISTANCE_KM: f64 = 500.0;
pub const MAX_FREE_CANCELLATION_HOURS: i64 = 8784;
pub const FLEXIBLE_FREE_HOURS: i64 = 48;
pub const MAX_SEARCH_CHARS: usize = 120;
pub const PROPERTY_TYPES: &[&str] = &["ENTIRE_PLACE", "ROOM"];
pub const MEAL_PLANS: &[&str] = &["ROOM_ONLY", "BREAKFAST", "HALF_BOARD", "FULL_BOARD"];
pub const PAYMENT_COLLECTIONS: &[&str] = &["PREPAY", "PAY_AT_PROPERTY"];
pub const ACCESSIBILITY_CODES: &[&str] = &[
    "STEP_FREE_ENTRY",
    "GROUND_FLOOR",
    "ACCESSIBLE_BATHROOM",
    "WIDE_DOORWAYS",
    "HEARING_LOOP",
];
pub const SORTS: &[&str] = &["relevance", "name", "price", "distance"];
pub const CANCELLATION_CLASSES: &[&str] = &["FLEXIBLE", "MODERATE", "NON_REFUNDABLE"];

#[derive(Debug, Clone, PartialEq, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Origin {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SearchQuery {
    pub property_id: Option<String>,
    pub search: Option<String>,
    pub check_in: Option<NaiveDate>,
    pub check_out: Option<NaiveDate>,
    pub adults: i32,
    pub children: i32,
    pub child_ages: Vec<i32>,
    pub instant_booking: bool,
    pub property_types: Vec<String>,
    pub meal_plans: Vec<String>,
    pub payment_collections: Vec<String>,
    pub accessibility: Vec<String>,
    pub amenities: Vec<String>,
    pub categories: Vec<String>,
    pub price_min_minor: Option<i64>,
    pub price_max_minor: Option<i64>,
    pub min_free_cancellation_hours: Option<i64>,
    pub max_penalty_percent: Option<i64>,
    pub origin: Option<Origin>,
    pub max_distance_km: Option<f64>,
    pub sort: String,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Window {
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub nights: i64,
}

fn guests_and_ages(raw: &RawSearch) -> Result<(i32, i32, Vec<i32>)> {
    let adults = raw.adults.unwrap_or(1);
    let children = raw.children.unwrap_or(0);
    if !(1..=MAX_ADULTS).contains(&adults) || !(0..=MAX_CHILDREN).contains(&children) {
        return Err(Error::bad_request(SEARCH_GUESTS_RANGE)
            .with_params(json!({ "maxAdults": MAX_ADULTS, "maxChildren": MAX_CHILDREN })));
    }
    let mut child_ages = raw.child_ages.clone();
    if child_ages.len() != children as usize {
        return Err(Error::bad_request(SEARCH_CHILD_AGES_MISSING));
    }
    if child_ages.iter().any(|age| !(0..=MAX_CHILD_AGE).contains(age)) {
        return Err(Error::bad_request(SEARCH_CHILD_AGE_RANGE).with_params(json!({ "max": MAX_CHILD_AGE })));
    }
    child_ages.sort_unstable();
    Ok((adults, children, child_ages))
}

fn distance_limit(raw: &RawSearch, has_origin: bool) -> Result<Option<f64>> {
    let limit = match raw.max_distance_km {
        Some(km) if km > 0.0 && km <= MAX_DISTANCE_KM => Some(km),
        Some(_) => {
            return Err(Error::bad_request(SEARCH_DISTANCE_RANGE).with_params(json!({ "max": MAX_DISTANCE_KM })))
        }
        None => None,
    };
    if limit.is_some() && !has_origin {
        return Err(Error::bad_request(SEARCH_DISTANCE_NEEDS_ORIGIN));
    }
    Ok(limit)
}

fn normalize_search(value: Option<String>) -> Result<Option<String>> {
    match value.map(|text| text.trim().to_string()).filter(|text| !text.is_empty()) {
        Some(text) if text.chars().count() > MAX_SEARCH_CHARS => {
            Err(Error::bad_request(SEARCH_TEXT_TOO_LONG).with_params(json!({ "max": MAX_SEARCH_CHARS })))
        }
        other => Ok(other),
    }
}

fn price_range(raw: &RawSearch, dated: bool) -> Result<(Option<i64>, Option<i64>)> {
    let min = raw.price_min.as_deref().map(parse_price).transpose()?;
    let max = raw.price_max.as_deref().map(parse_price).transpose()?;
    if (min.is_some() || max.is_some()) && !dated {
        return Err(Error::bad_request(SEARCH_PRICE_NEEDS_DATES));
    }
    if let (Some(min), Some(max)) = (min, max) {
        if min > max {
            return Err(Error::bad_request(SEARCH_PRICE_RANGE_INVERTED));
        }
    }
    Ok((min, max))
}

fn cancellation_limits(raw: &RawSearch) -> Result<()> {
    if let Some(hours) = raw.min_free_cancellation_hours {
        if !(0..=MAX_FREE_CANCELLATION_HOURS).contains(&hours) {
            return Err(Error::bad_request(SEARCH_CANCELLATION_HOURS_RANGE)
                .with_params(json!({ "max": MAX_FREE_CANCELLATION_HOURS })));
        }
    }
    if let Some(percent) = raw.max_penalty_percent {
        if !(0..=100).contains(&percent) {
            return Err(Error::bad_request(SEARCH_PENALTY_RANGE));
        }
    }
    Ok(())
}

fn paging(raw: &RawSearch) -> Result<(i64, i64)> {
    let page = raw.page.unwrap_or(1);
    let page_size = raw.page_size.unwrap_or(20);
    let in_range = |value: i64| u32::try_from(value).unwrap_or(0);
    let request = kernel::paging::PageRequest {
        page: if page > i64::from(u32::MAX) { 1 } else { in_range(page) },
        page_size: in_range(page_size),
    }
    .validated()?;
    if page > i64::from(u32::MAX) || (page - 1).checked_mul(page_size).is_none() {
        return Err(Error::bad_request(SEARCH_PAGE_OFFSET_TOO_LARGE));
    }
    Ok((i64::from(request.page), i64::from(request.page_size)))
}

impl SearchQuery {
    pub fn from_raw(raw: RawSearch) -> Result<Self> {
        let window = window_from(raw.check_in, raw.check_out)?;
        let (adults, children, child_ages) = guests_and_ages(&raw)?;
        let (price_min_minor, price_max_minor) = price_range(&raw, window.is_some())?;
        cancellation_limits(&raw)?;
        let origin = origin_from(raw.latitude, raw.longitude)?;
        let max_distance_km = distance_limit(&raw, origin.is_some())?;
        let sort = normalize_sort(raw.sort.as_deref())?;
        if sort == "price" && window.is_none() {
            return Err(Error::bad_request(SEARCH_SORT_NEEDS_DATES));
        }
        if sort == "distance" && origin.is_none() {
            return Err(Error::bad_request(SEARCH_SORT_NEEDS_ORIGIN));
        }
        let (page, page_size) = paging(&raw)?;
        let property_id = match raw.property_id {
            Some(id) if id.trim().is_empty() => return Err(Error::bad_request(SEARCH_PROPERTY_ID_EMPTY)),
            Some(id) => Some(id.trim().to_string()),
            None => None,
        };
        Ok(Self {
            property_id,
            search: normalize_search(raw.search)?,
            check_in: window.map(|window| window.check_in),
            check_out: window.map(|window| window.check_out),
            adults,
            children,
            child_ages,
            instant_booking: raw.instant_booking,
            property_types: normalize_codes(raw.property_types, PROPERTY_TYPES, "propertyTypes")?,
            meal_plans: normalize_codes(raw.meal_plans, MEAL_PLANS, "mealPlans")?,
            payment_collections: normalize_codes(raw.payment_collections, PAYMENT_COLLECTIONS, "paymentCollections")?,
            accessibility: normalize_codes(raw.accessibility, ACCESSIBILITY_CODES, "accessibility")?,
            amenities: normalize_code_format(raw.amenities, "amenities")?,
            categories: normalize_code_format(raw.categories, "categories")?,
            price_min_minor,
            price_max_minor,
            min_free_cancellation_hours: raw.min_free_cancellation_hours,
            max_penalty_percent: raw.max_penalty_percent,
            origin,
            max_distance_km,
            sort,
            page,
            page_size,
        })
    }

    pub fn window(&self) -> Option<Window> {
        window_from(self.check_in, self.check_out).ok().flatten()
    }

    pub fn canonical_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    pub fn fingerprint(&self) -> String {
        let digest = Sha256::digest(self.canonical_json().as_bytes());
        hex::encode(&digest[..8])
    }
}
