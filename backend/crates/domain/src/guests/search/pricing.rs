use std::collections::{HashMap, HashSet};

use chrono::{NaiveDate, Utc};
use kernel::city::City;
use kernel::error::Result;
use kernel::timezone::offset_for_timezone;
use kernel::Money;
use sqlx::PgConnection;

use super::candidates::CandidateRow;
use crate::guests::collaborators::{
    active_promotions_for_properties, guest_factor, nightly_amount_minor, rules_for_plans,
    stay_total_minor, violations_batch, Promotion, RateRule, StayRequest, StayTerms,
};
use super::query::{SearchQuery, Window};

#[derive(Debug, Clone)]
pub struct Offer {
    pub row: CandidateRow,
    pub total_minor: Option<i64>,
    pub nightly_minor: Option<i64>,
}

pub struct Stay<'a> {
    pub window: Window,
    pub nights: &'a [NaiveDate],
}

pub async fn price(
    connection: &mut PgConnection,
    city: City,
    rows: Vec<CandidateRow>,
    query: &SearchQuery,
    stay: Option<&Stay<'_>>,
) -> Result<Vec<Offer>> {
    let Some(stay) = stay else {
        return nightly_offers(connection, city, rows, query).await;
    };
    let blocked = restricted_units(&mut *connection, &rows, stay).await?;
    let plan_ids = unique(rows.iter().map(|row| row.plan_id.clone()));
    let property_ids = unique(rows.iter().map(|row| row.property_id.clone()));
    let rules = rules_for_plans(&mut *connection, city, &plan_ids).await?;
    let promotions = active_promotions_for_properties(
        &mut *connection,
        city,
        &property_ids,
        stay.window.check_in,
        stay.window.check_out,
    )
    .await?;
    let mut offers = Vec::new();
    for row in rows {
        if blocked.contains(&(row.property_id.clone(), row.unit_type_id.clone())) {
            continue;
        }
        let Some(total_minor) = total_for(&row, query, stay, &rules, &promotions) else {
            continue;
        };
        let nightly_minor = i64::try_from(stay.nights.len())
            .ok()
            .filter(|count| *count > 0)
            .map(|count| total_minor / count);
        offers.push(Offer { row, total_minor: Some(total_minor), nightly_minor });
    }
    Ok(offers)
}

async fn nightly_offers(
    connection: &mut PgConnection,
    city: City,
    rows: Vec<CandidateRow>,
    query: &SearchQuery,
) -> Result<Vec<Offer>> {
    let plan_ids = unique(rows.iter().map(|row| row.plan_id.clone()));
    let rules = rules_for_plans(&mut *connection, city, &plan_ids).await?;
    let today = Utc::now().date_naive();
    Ok(rows
        .into_iter()
        .map(|row| {
            let plan_rules: &[RateRule] = rules.get(&row.plan_id).map(Vec::as_slice).unwrap_or(&[]);
            let nightly_minor = nightly_amount_minor(today, plan_rules)
                .and_then(|amount| amount.checked_mul(guest_factor(&row.occupancy_basis, query.adults)));
            Offer { row, total_minor: None, nightly_minor }
        })
        .collect())
}

async fn restricted_units(
    connection: &mut PgConnection,
    rows: &[CandidateRow],
    stay: &Stay<'_>,
) -> Result<HashSet<(String, String)>> {
    let mut seen: HashSet<(String, String)> = HashSet::new();
    let mut requests: Vec<(String, StayRequest)> = Vec::new();
    for row in rows {
        let key = (row.property_id.clone(), row.unit_type_id.clone());
        if !seen.insert(key.clone()) {
            continue;
        }
        let Some(offset) = offset_for_timezone(&row.timezone) else {
            continue;
        };
        requests.push((
            key.0.clone(),
            StayRequest {
                unit_type_id: key.1.clone(),
                check_in: stay.window.check_in,
                check_out: stay.window.check_out,
                booking_day: Utc::now().with_timezone(&offset).date_naive(),
            },
        ));
    }
    let verdicts = violations_batch(&mut *connection, &requests).await?;
    Ok(requests
        .into_iter()
        .zip(verdicts)
        .filter(|(_, found)| !found.is_empty())
        .map(|((property_id, request), _)| (property_id, request.unit_type_id))
        .collect())
}

fn total_for(
    row: &CandidateRow,
    query: &SearchQuery,
    stay: &Stay<'_>,
    rules: &HashMap<String, Vec<RateRule>>,
    promotions: &[(String, Option<String>, Promotion)],
) -> Option<i64> {
    let plan_rules: &[RateRule] = rules.get(&row.plan_id).map(Vec::as_slice).unwrap_or(&[]);
    let plan_promotions: Vec<Promotion> = promotions
        .iter()
        .filter(|(property, plan, _)| {
            property == &row.property_id && plan.as_deref().is_none_or(|id| id == row.plan_id)
        })
        .map(|(_, _, promotion)| promotion.clone())
        .collect();
    let terms = StayTerms {
        rules: plan_rules,
        guest_factor: guest_factor(&row.occupancy_basis, query.adults),
        child_nightly_minor: Money::parse(&row.child_nightly_amount).ok()?.minor(),
        child_count: i64::from(query.children),
        promotions: &plan_promotions,
    };
    stay_total_minor(stay.nights, &terms)
}

fn unique(values: impl Iterator<Item = String>) -> Vec<String> {
    let mut values: Vec<String> = values.collect();
    values.sort();
    values.dedup();
    values
}
