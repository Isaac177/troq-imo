use std::collections::HashMap;

use chrono::NaiveDate;
use kernel::error::Result;
use serde_json::json;
use sqlx::PgConnection;

use super::check::{evaluate, overlaps, strictest, ActiveRule, StayRequest, Violation};
use crate::inventory::messages::violation as code;

#[derive(Debug, sqlx::FromRow)]
struct PropertyRuleRow {
    property_id: String,
    unit_type_id: Option<String>,
    kind: String,
    basis: String,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    value: Option<i32>,
}

pub async fn violations_batch(
    connection: &mut PgConnection,
    requests: &[(String, StayRequest)],
) -> Result<Vec<Vec<Violation>>> {
    if requests.is_empty() {
        return Ok(Vec::new());
    }
    let mut property_ids: Vec<String> =
        requests.iter().map(|(property_id, _)| property_id.clone()).collect();
    property_ids.sort();
    property_ids.dedup();
    let rows = sqlx::query_as::<_, PropertyRuleRow>(
        r#"SELECT "propertyId" AS property_id, "unitTypeId" AS unit_type_id, kind, basis,
                  "startDate" AS start_date, "endDate" AS end_date, value
           FROM stay_restrictions
           WHERE "propertyId" = ANY($1) AND "removedAt" IS NULL
           ORDER BY kind"#,
    )
    .bind(&property_ids)
    .fetch_all(&mut *connection)
    .await?;
    let mut by_property: HashMap<String, Vec<PropertyRuleRow>> = HashMap::new();
    for row in rows {
        by_property.entry(row.property_id.clone()).or_default().push(row);
    }
    let mut results = Vec::with_capacity(requests.len());
    for (property_id, request) in requests {
        let rules: Vec<ActiveRule> = by_property
            .get(property_id)
            .map(|rows| {
                rows.iter()
                    .filter(|row| row.unit_type_id.as_deref().map_or(true, |unit| unit == request.unit_type_id))
                    .map(|row| ActiveRule {
                        kind: row.kind.clone(),
                        basis: row.basis.clone(),
                        start_date: row.start_date,
                        end_date: row.end_date,
                        value: row.value,
                        unit_scoped: row.unit_type_id.is_some(),
                    })
                    .collect()
            })
            .unwrap_or_default();
        results.push(with_turnover_batch(&mut *connection, &rules, request).await?);
    }
    Ok(results)
}

async fn with_turnover_batch(
    connection: &mut PgConnection,
    rules: &[ActiveRule],
    request: &StayRequest,
) -> Result<Vec<Violation>> {
    let mut found = evaluate(rules, request);
    let buffers: Vec<&ActiveRule> = rules
        .iter()
        .filter(|rule| rule.kind == "TURNOVER_BUFFER_NIGHTS" && overlaps(rule, request))
        .collect();
    if let Some(buffer) = strictest(&buffers, i32::max) {
        if buffer > 0 {
            let busy: bool = sqlx::query_scalar(
                r#"SELECT
                (SELECT COALESCE(SUM(p."sellableQuantity"), 0) = 1 FROM stay_inventory_pools p WHERE p."unitTypeId" = $1)
                AND EXISTS (SELECT 1 FROM stay_inventory_allocations a JOIN stay_inventory_pools p ON p.id = a."poolId"
                    WHERE p."unitTypeId" = $1 AND a.state = 'ACTIVE'
                    AND ((a."startDate" < $2 AND a."endDate" > $2 - make_interval(days => $4::int))
                      OR (a."startDate" < $3 + make_interval(days => $4::int) AND a."endDate" > $3)))"#,
            )
            .bind(&request.unit_type_id)
            .bind(request.check_in)
            .bind(request.check_out)
            .bind(buffer)
            .fetch_one(connection)
            .await?;
            if busy {
                found.push(Violation::new(code::TURNOVER_BUFFER, json!({ "nights": buffer })));
            }
        }
    }
    Ok(found)
}
