use chrono::NaiveDate;
use kernel::error::{Error, Result};
use serde::Serialize;
use serde_json::{json, Value};
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::inventory::messages::{violation as code, STAY_RULE_VIOLATED};

#[derive(Debug, sqlx::FromRow)]
pub(super) struct ActiveRule {
    pub(super) kind: String,
    pub(super) basis: String,
    pub(super) start_date: Option<NaiveDate>,
    pub(super) end_date: Option<NaiveDate>,
    pub(super) value: Option<i32>,
    pub(super) unit_scoped: bool,
}

#[derive(Debug, Clone)]
pub struct StayRequest {
    pub unit_type_id: String,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub booking_day: NaiveDate,
}

#[derive(Debug, Clone, PartialEq, Serialize, ToSchema)]
pub struct Violation {
    pub code: String,
    pub params: Value,
}

impl Violation {
    pub(super) fn new(code: &str, params: Value) -> Self {
        Self { code: code.to_string(), params }
    }

    pub fn into_error(self) -> Error {
        Error::bad_request(STAY_RULE_VIOLATED)
            .with_params(json!({ "violation": self.code, "details": self.params }))
    }
}

pub async fn enforce(pool: &PgPool, property_id: &str, request: &StayRequest) -> Result<()> {
    match violations(pool, property_id, request).await?.into_iter().next() {
        Some(violation) => Err(violation.into_error()),
        None => Ok(()),
    }
}

pub async fn violations(
    pool: &PgPool,
    property_id: &str,
    request: &StayRequest,
) -> Result<Vec<Violation>> {
    let rules = active_rules(pool, property_id, &request.unit_type_id).await?;
    with_turnover(pool, &rules, request, None).await
}

pub(super) async fn with_turnover(
    pool: &PgPool,
    rules: &[ActiveRule],
    request: &StayRequest,
    excluded_reservation: Option<&str>,
) -> Result<Vec<Violation>> {
    let mut found = evaluate(rules, request);
    let buffer_rules: Vec<&ActiveRule> = rules
        .iter()
        .filter(|rule| rule.kind == "TURNOVER_BUFFER_NIGHTS" && overlaps(rule, request))
        .collect();
    if let Some(buffer) = strictest(&buffer_rules, i32::max) {
        if turnover_conflict(pool, request, buffer, excluded_reservation).await? {
            found.push(Violation::new(code::TURNOVER_BUFFER, json!({ "nights": buffer })));
        }
    }
    Ok(found)
}

async fn turnover_conflict(
    pool: &PgPool,
    request: &StayRequest,
    buffer: i32,
    excluded_reservation: Option<&str>,
) -> Result<bool> {
    let single_unit: bool = sqlx::query_scalar(
        r#"SELECT COALESCE(SUM(p."sellableQuantity"), 0) = 1 FROM stay_inventory_pools p WHERE p."unitTypeId" = $1"#,
    )
    .bind(&request.unit_type_id)
    .fetch_one(pool)
    .await?;
    if !single_unit || buffer <= 0 {
        return Ok(false);
    }
    let busy: bool = sqlx::query_scalar(
        r#"SELECT EXISTS (
               SELECT 1 FROM stay_inventory_allocations a
               JOIN stay_inventory_pools p ON p.id = a."poolId"
               WHERE p."unitTypeId" = $1 AND a.state = 'ACTIVE'
                 AND ($5::text IS NULL OR a."referenceId" <> $5)
                 AND ((a."startDate" < $2 AND a."endDate" > $2 - make_interval(days => $4::int))
                   OR (a."startDate" < $3 + make_interval(days => $4::int) AND a."endDate" > $3)))"#,
    )
    .bind(&request.unit_type_id)
    .bind(request.check_in)
    .bind(request.check_out)
    .bind(buffer)
    .bind(excluded_reservation)
    .fetch_one(pool)
    .await?;
    Ok(busy)
}

pub(super) async fn active_rules(
    pool: &PgPool,
    property_id: &str,
    unit_type_id: &str,
) -> Result<Vec<ActiveRule>> {
    let rows = sqlx::query_as::<_, ActiveRule>(
        r#"SELECT kind, basis, "startDate" AS start_date, "endDate" AS end_date, value,
                  ("unitTypeId" IS NOT NULL) AS unit_scoped
           FROM stay_restrictions
           WHERE "propertyId" = $1 AND "removedAt" IS NULL
             AND ("unitTypeId" IS NULL OR "unitTypeId" = $2)
           ORDER BY kind"#,
    )
    .bind(property_id)
    .bind(unit_type_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub(super) fn evaluate(rules: &[ActiveRule], request: &StayRequest) -> Vec<Violation> {
    let mut found = Vec::new();
    let nights = (request.check_out - request.check_in).num_days();
    let notice = (request.check_in - request.booking_day).num_days();

    if rules
        .iter()
        .any(|rule| rule.kind == "CLOSED_TO_ARRIVAL" && contains(rule, request.check_in))
    {
        found.push(Violation::new(code::ARRIVAL_CLOSED, json!({ "date": request.check_in.to_string() })));
    }
    if rules
        .iter()
        .any(|rule| rule.kind == "CLOSED_TO_DEPARTURE" && contains(rule, request.check_out))
    {
        found.push(Violation::new(code::DEPARTURE_CLOSED, json!({ "date": request.check_out.to_string() })));
    }

    let minimum_rules: Vec<&ActiveRule> = rules
        .iter()
        .filter(|rule| rule.kind == "MIN_NIGHTS")
        .filter(|rule| match rule.basis.as_str() {
            "ARRIVAL" => contains(rule, request.check_in),
            _ => overlaps(rule, request),
        })
        .collect();
    if let Some(required) = strictest(&minimum_rules, i32::max) {
        if nights < i64::from(required) {
            found.push(Violation::new(code::MIN_NIGHTS, json!({ "nights": required })));
        }
    }

    let maximum_rules: Vec<&ActiveRule> = rules
        .iter()
        .filter(|rule| rule.kind == "MAX_NIGHTS" && overlaps(rule, request))
        .collect();
    if let Some(allowed) = strictest(&maximum_rules, i32::min) {
        if nights > i64::from(allowed) {
            found.push(Violation::new(code::MAX_NIGHTS, json!({ "nights": allowed })));
        }
    }

    let notice_rules: Vec<&ActiveRule> = rules
        .iter()
        .filter(|rule| rule.kind == "ADVANCE_NOTICE_DAYS" && contains(rule, request.check_in))
        .collect();
    if let Some(days) = strictest(&notice_rules, i32::max) {
        if notice < i64::from(days) {
            found.push(Violation::new(code::ADVANCE_NOTICE, json!({ "days": days })));
        }
    }

    let horizon_rules: Vec<&ActiveRule> = rules
        .iter()
        .filter(|rule| rule.kind == "MAX_HORIZON_DAYS" && contains(rule, request.check_in))
        .collect();
    if let Some(days) = strictest(&horizon_rules, i32::min) {
        if notice > i64::from(days) {
            found.push(Violation::new(code::MAX_HORIZON, json!({ "days": days })));
        }
    }

    found
}

pub(super) fn strictest(rules: &[&ActiveRule], pick: fn(i32, i32) -> i32) -> Option<i32> {
    let specific: Vec<i32> = rules.iter().filter(|rule| rule.unit_scoped).filter_map(|rule| rule.value).collect();
    let general: Vec<i32> = rules.iter().filter(|rule| !rule.unit_scoped).filter_map(|rule| rule.value).collect();
    let chosen = if specific.is_empty() { general } else { specific };
    chosen.into_iter().reduce(pick)
}

fn contains(rule: &ActiveRule, date: NaiveDate) -> bool {
    rule.start_date.map_or(true, |start| date >= start)
        && rule.end_date.map_or(true, |end| date < end)
}

pub(super) fn overlaps(rule: &ActiveRule, request: &StayRequest) -> bool {
    rule.start_date.map_or(true, |start| start < request.check_out)
        && rule.end_date.map_or(true, |end| end > request.check_in)
}
