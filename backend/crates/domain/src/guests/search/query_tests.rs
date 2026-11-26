use chrono::NaiveDate;

use super::cancellation_class;
use super::query::SearchQuery;
use super::raw::RawSearch;

fn date(text: &str) -> NaiveDate {
    NaiveDate::parse_from_str(text, "%Y-%m-%d").unwrap()
}

fn dated() -> RawSearch {
    RawSearch {
        check_in: Some(date("2040-03-01")),
        check_out: Some(date("2040-03-03")),
        ..RawSearch::default()
    }
}

#[test]
fn search_text_is_trimmed_and_length_limited() {
    let query = SearchQuery::from_raw(RawSearch { search: Some("  Villa  ".into()), ..RawSearch::default() }).unwrap();
    assert_eq!(query.search.as_deref(), Some("Villa"));
    assert!(SearchQuery::from_raw(RawSearch { search: Some("x".repeat(121)), ..RawSearch::default() }).is_err());
}

#[test]
fn defaults_are_relevance_first_page_one_of_twenty() {
    let query = SearchQuery::from_raw(RawSearch::default()).unwrap();
    assert_eq!(query.sort, "relevance");
    assert_eq!((query.page, query.page_size), (1, 20));
    assert_eq!((query.adults, query.children), (1, 0));
    assert!(query.window().is_none());
}

#[test]
fn dates_come_as_a_pair_and_stay_within_the_limit() {
    assert!(SearchQuery::from_raw(RawSearch { check_in: Some(date("2040-03-01")), ..RawSearch::default() }).is_err());
    assert!(SearchQuery::from_raw(RawSearch {
        check_in: Some(date("2040-03-03")),
        check_out: Some(date("2040-03-03")),
        ..RawSearch::default()
    })
    .is_err());
    let long = RawSearch {
        check_in: Some(date("2040-01-01")),
        check_out: Some(date("2041-06-01")),
        ..RawSearch::default()
    };
    assert!(SearchQuery::from_raw(long).is_err());
    assert_eq!(SearchQuery::from_raw(dated()).unwrap().window().unwrap().nights, 2);
}

#[test]
fn price_sort_and_price_filters_need_dates() {
    assert!(SearchQuery::from_raw(RawSearch { sort: Some("price".into()), ..RawSearch::default() }).is_err());
    assert!(SearchQuery::from_raw(RawSearch { price_max: Some("100.00".into()), ..RawSearch::default() }).is_err());
    assert!(SearchQuery::from_raw(RawSearch { sort: Some("price".into()), ..dated() }).is_ok());
}

#[test]
fn price_range_must_be_ordered_and_exact() {
    let inverted = RawSearch { price_min: Some("10.00".into()), price_max: Some("5.00".into()), ..dated() };
    assert!(SearchQuery::from_raw(inverted).is_err());
    let exact = SearchQuery::from_raw(RawSearch { price_min: Some("1000.50".into()), ..dated() }).unwrap();
    assert_eq!(exact.price_min_minor, Some(100_050));
    assert!(SearchQuery::from_raw(RawSearch { price_min: Some("1.005".into()), ..dated() }).is_err());
    let malformed = SearchQuery::from_raw(RawSearch { price_max: Some("abc".into()), ..dated() }).unwrap_err();
    assert_eq!(malformed.code().as_str(), "AMOUNT_INVALID");
}

#[test]
fn distance_needs_a_point_and_a_sane_radius() {
    assert!(SearchQuery::from_raw(RawSearch { max_distance_km: Some(2.0), ..RawSearch::default() }).is_err());
    assert!(SearchQuery::from_raw(RawSearch { latitude: Some(-1.67), ..RawSearch::default() }).is_err());
    assert!(SearchQuery::from_raw(RawSearch { latitude: Some(91.0), longitude: Some(0.0), ..RawSearch::default() }).is_err());
    let sorted = SearchQuery::from_raw(RawSearch {
        sort: Some("distance".into()),
        latitude: Some(-1.6792),
        longitude: Some(29.2228),
        ..RawSearch::default()
    });
    assert!(sorted.is_ok());
    assert!(SearchQuery::from_raw(RawSearch { sort: Some("distance".into()), ..RawSearch::default() }).is_err());
    assert!(SearchQuery::from_raw(RawSearch {
        max_distance_km: Some(900.0),
        latitude: Some(0.0),
        longitude: Some(0.0),
        ..RawSearch::default()
    })
    .is_err());
}

#[test]
fn origin_is_rounded_to_two_decimals_before_it_is_used_or_stored() {
    let query = SearchQuery::from_raw(RawSearch {
        latitude: Some(-1.679212345),
        longitude: Some(29.222849),
        ..RawSearch::default()
    })
    .unwrap();
    let origin = query.origin.unwrap();
    assert_eq!(origin.latitude, -1.68);
    assert_eq!(origin.longitude, 29.22);
}

#[test]
fn codes_are_normalised_and_malformed_codes_are_refused() {
    let query = SearchQuery::from_raw(RawSearch {
        amenities: vec![" wifi ".into(), "PARKING".into(), "wifi".into(), "".into()],
        ..RawSearch::default()
    })
    .unwrap();
    assert_eq!(query.amenities, vec!["PARKING".to_string(), "WIFI".to_string()]);
    assert!(SearchQuery::from_raw(RawSearch { amenities: vec!["NOT VALID".into()], ..RawSearch::default() }).is_err());
    assert!(SearchQuery::from_raw(RawSearch { meal_plans: vec!["BUFFET".into()], ..RawSearch::default() }).is_err());
}

#[test]
fn cancellation_limits_and_unknown_sorts_are_checked() {
    assert!(SearchQuery::from_raw(RawSearch { min_free_cancellation_hours: Some(-1), ..RawSearch::default() }).is_err());
    assert!(SearchQuery::from_raw(RawSearch { max_penalty_percent: Some(101), ..RawSearch::default() }).is_err());
    assert!(SearchQuery::from_raw(RawSearch { sort: Some("popularity".into()), ..RawSearch::default() }).is_err());
}

#[test]
fn guests_and_child_ages_are_validated() {
    assert!(SearchQuery::from_raw(RawSearch { adults: Some(0), ..RawSearch::default() }).is_err());
    assert!(SearchQuery::from_raw(RawSearch { children: Some(1), ..RawSearch::default() }).is_err());
    assert!(SearchQuery::from_raw(RawSearch { children: Some(1), child_ages: vec![18], ..RawSearch::default() }).is_err());
    let sorted = SearchQuery::from_raw(RawSearch { children: Some(2), child_ages: vec![9, 3], ..RawSearch::default() }).unwrap();
    assert_eq!(sorted.child_ages, vec![3, 9]);
}

#[test]
fn paging_is_validated_and_offsets_are_bounded() {
    assert!(SearchQuery::from_raw(RawSearch { page: Some(0), ..RawSearch::default() }).is_err());
    assert!(SearchQuery::from_raw(RawSearch { page_size: Some(101), ..RawSearch::default() }).is_err());
    assert!(SearchQuery::from_raw(RawSearch { page: Some(i64::from(u32::MAX) + 1), ..RawSearch::default() }).is_err());
}

#[test]
fn fingerprint_depends_only_on_the_normalised_query() {
    let left = SearchQuery::from_raw(RawSearch { amenities: vec!["WIFI".into(), "PARKING".into()], ..dated() }).unwrap();
    let right = SearchQuery::from_raw(RawSearch { amenities: vec!["parking".into(), "wifi".into()], ..dated() }).unwrap();
    assert_eq!(left.fingerprint(), right.fingerprint());
    let other = SearchQuery::from_raw(RawSearch { instant_booking: true, ..dated() }).unwrap();
    assert_ne!(left.fingerprint(), other.fingerprint());
    assert_eq!(left.fingerprint().len(), 16);
}

#[test]
fn cancellation_classes_follow_the_documented_rules() {
    assert_eq!(cancellation_class(Some(48), Some(0)), "FLEXIBLE");
    assert_eq!(cancellation_class(Some(168), Some(50)), "MODERATE");
    assert_eq!(cancellation_class(Some(0), Some(100)), "NON_REFUNDABLE");
    assert_eq!(cancellation_class(Some(24), Some(0)), "MODERATE");
}
