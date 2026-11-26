use chrono::NaiveDate;
use serde_json::json;

use super::check::{evaluate, ActiveRule, StayRequest, Violation};
use crate::inventory::messages::violation as code;

fn day(d: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 12, d).unwrap()
}

fn rule(kind: &str, value: Option<i32>, unit_scoped: bool) -> ActiveRule {
    ActiveRule {
        kind: kind.into(),
        basis: "STAY_THROUGH".into(),
        start_date: None,
        end_date: None,
        value,
        unit_scoped,
    }
}

fn request(check_in: u32, check_out: u32, booking: u32) -> StayRequest {
    StayRequest {
        unit_type_id: "u".into(),
        check_in: day(check_in),
        check_out: day(check_out),
        booking_day: day(booking),
    }
}

fn codes(found: Vec<Violation>) -> Vec<(String, serde_json::Value)> {
    found.into_iter().map(|violation| (violation.code, violation.params)).collect()
}

#[test]
fn minimum_stay_is_enforced() {
    let rules = vec![rule("MIN_NIGHTS", Some(3), false)];
    assert_eq!(
        codes(evaluate(&rules, &request(10, 12, 1))),
        vec![(code::MIN_NIGHTS.to_string(), json!({ "nights": 3 }))]
    );
    assert!(evaluate(&rules, &request(10, 13, 1)).is_empty());
}

#[test]
fn unit_rule_overrides_property_rule_even_when_looser() {
    let rules = vec![rule("MIN_NIGHTS", Some(5), false), rule("MIN_NIGHTS", Some(2), true)];
    assert!(evaluate(&rules, &request(10, 12, 1)).is_empty());
}

#[test]
fn strictest_value_wins_within_a_level() {
    let rules = vec![rule("MIN_NIGHTS", Some(2), false), rule("MIN_NIGHTS", Some(4), false)];
    assert_eq!(
        codes(evaluate(&rules, &request(10, 13, 1))),
        vec![(code::MIN_NIGHTS.to_string(), json!({ "nights": 4 }))]
    );
}

#[test]
fn closures_apply_to_the_exact_arrival_or_departure_day() {
    let closed = ActiveRule {
        kind: "CLOSED_TO_ARRIVAL".into(),
        basis: "STAY_THROUGH".into(),
        start_date: Some(day(10)),
        end_date: Some(day(11)),
        value: None,
        unit_scoped: false,
    };
    assert_eq!(
        codes(evaluate(std::slice::from_ref(&closed), &request(10, 12, 1))),
        vec![(code::ARRIVAL_CLOSED.to_string(), json!({ "date": "2026-12-10" }))]
    );
    assert!(evaluate(std::slice::from_ref(&closed), &request(11, 12, 1)).is_empty());
}

#[test]
fn notice_and_horizon_are_checked_against_booking_day() {
    let rules = vec![rule("ADVANCE_NOTICE_DAYS", Some(3), false), rule("MAX_HORIZON_DAYS", Some(30), false)];
    assert_eq!(
        codes(evaluate(&rules, &request(10, 12, 9))),
        vec![(code::ADVANCE_NOTICE.to_string(), json!({ "days": 3 }))]
    );
    assert!(evaluate(&rules, &request(10, 12, 7)).is_empty());
}

#[test]
fn arrival_basis_minimum_only_looks_at_arrival_day() {
    let arrival_rule = ActiveRule {
        kind: "MIN_NIGHTS".into(),
        basis: "ARRIVAL".into(),
        start_date: Some(day(20)),
        end_date: Some(day(31)),
        value: Some(4),
        unit_scoped: false,
    };
    assert!(evaluate(std::slice::from_ref(&arrival_rule), &request(10, 11, 1)).is_empty());
    assert_eq!(
        codes(evaluate(std::slice::from_ref(&arrival_rule), &request(20, 22, 1))),
        vec![(code::MIN_NIGHTS.to_string(), json!({ "nights": 4 }))]
    );
}
