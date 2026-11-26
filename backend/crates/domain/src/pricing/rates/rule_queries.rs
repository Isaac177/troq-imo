use std::collections::HashMap;

use chrono::NaiveDate;
use kernel::{City, Money, Result};
use sqlx::{FromRow, PgConnection, PgPool, Postgres, Transaction};

use super::engine::{Adjustment, RateRule};
use super::types::{weekdays_from_mask, RuleView};
use crate::pricing::amounts::parse_stored;

#[derive(Debug, FromRow)]
struct RuleRow {
    id: String,
    rate_plan_id: String,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    weekday_mask: i16,
    adjustment_type: String,
    amount: String,
    priority: i32,
}

pub struct NewRule<'a> {
    pub id: &'a str,
    pub city: City,
    pub rate_plan_id: &'a str,
    pub start: Option<NaiveDate>,
    pub end: Option<NaiveDate>,
    pub weekday_mask: u8,
    pub adjustment_type: &'a str,
    pub amount: Money,
    pub priority: i32,
}

pub async fn list_rules(
    pool: &PgPool,
    city: City,
    plan_id: &str,
    limit: i64,
    offset: i64,
) -> Result<(Vec<RuleView>, i64)> {
    let total: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM stay_rate_rules WHERE city_id = $1 AND "ratePlanId" = $2"#,
    )
    .bind(city.code())
    .bind(plan_id)
    .fetch_one(pool)
    .await?;
    let rows = sqlx::query_as::<_, RuleRow>(
        r#"SELECT id, "ratePlanId" AS rate_plan_id, "startDate" AS start_date, "endDate" AS end_date,
                  "weekdayMask" AS weekday_mask, "adjustmentType" AS adjustment_type,
                  amount::text AS amount, priority
           FROM stay_rate_rules WHERE city_id = $1 AND "ratePlanId" = $2
           ORDER BY priority DESC, id LIMIT $3 OFFSET $4"#,
    )
    .bind(city.code())
    .bind(plan_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    let items = rows.into_iter().map(rule_view).collect::<Result<_>>()?;
    Ok((items, total))
}

pub async fn insert_rule(tx: &mut Transaction<'_, Postgres>, rule: &NewRule<'_>) -> Result<()> {
    sqlx::query(
        r#"INSERT INTO stay_rate_rules (id, city_id, "ratePlanId", "startDate", "endDate", "weekdayMask",
               "adjustmentType", amount, priority)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8::text::numeric, $9)"#,
    )
    .bind(rule.id)
    .bind(rule.city.code())
    .bind(rule.rate_plan_id)
    .bind(rule.start)
    .bind(rule.end)
    .bind(i16::from(rule.weekday_mask))
    .bind(rule.adjustment_type)
    .bind(rule.amount.to_wire())
    .bind(rule.priority)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn delete_rule(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    plan_id: &str,
    rule_id: &str,
) -> Result<bool> {
    let result = sqlx::query(
        r#"DELETE FROM stay_rate_rules WHERE city_id = $1 AND id = $2 AND "ratePlanId" = $3"#,
    )
    .bind(city.code())
    .bind(rule_id)
    .bind(plan_id)
    .execute(&mut **tx)
    .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn load_rules(pool: &PgPool, city: City, rate_plan_id: &str) -> Result<Vec<RateRule>> {
    let rows = sqlx::query_as::<_, RuleRow>(
        r#"SELECT id, "ratePlanId" AS rate_plan_id, "startDate" AS start_date, "endDate" AS end_date,
                  "weekdayMask" AS weekday_mask, "adjustmentType" AS adjustment_type,
                  amount::text AS amount, priority
           FROM stay_rate_rules WHERE city_id = $1 AND "ratePlanId" = $2
           ORDER BY priority DESC, id"#,
    )
    .bind(city.code())
    .bind(rate_plan_id)
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(|row| rate_rule(row).map(|(_, rule)| rule)).collect()
}

pub async fn rules_for_plans(
    connection: &mut PgConnection,
    city: City,
    plan_ids: &[String],
) -> Result<HashMap<String, Vec<RateRule>>> {
    let rows = sqlx::query_as::<_, RuleRow>(
        r#"SELECT id, "ratePlanId" AS rate_plan_id, "startDate" AS start_date, "endDate" AS end_date,
                  "weekdayMask" AS weekday_mask, "adjustmentType" AS adjustment_type,
                  amount::text AS amount, priority
           FROM stay_rate_rules WHERE city_id = $1 AND "ratePlanId" = ANY($2)
           ORDER BY priority DESC, id"#,
    )
    .bind(city.code())
    .bind(plan_ids)
    .fetch_all(connection)
    .await?;
    let mut grouped: HashMap<String, Vec<RateRule>> = HashMap::new();
    for row in rows {
        let (plan_id, rule) = rate_rule(row)?;
        grouped.entry(plan_id).or_default().push(rule);
    }
    Ok(grouped)
}

fn rule_mask(value: i16) -> Result<u8> {
    u8::try_from(value).map_err(|_| kernel::Error::internal("weekday mask out of range"))
}

fn rule_view(row: RuleRow) -> Result<RuleView> {
    Ok(RuleView {
        id: row.id,
        rate_plan_id: row.rate_plan_id,
        start_date: row.start_date,
        end_date: row.end_date,
        weekdays: weekdays_from_mask(rule_mask(row.weekday_mask)?),
        adjustment_type: row.adjustment_type,
        amount: Money::parse(&row.amount)?,
        priority: row.priority,
    })
}

fn rate_rule(row: RuleRow) -> Result<(String, RateRule)> {
    let rule = RateRule {
        id: row.id,
        start: row.start_date,
        end: row.end_date,
        weekday_mask: rule_mask(row.weekday_mask)?,
        adjustment: Adjustment::parse(&row.adjustment_type)?,
        amount_minor: parse_stored(&row.amount)?,
        priority: row.priority,
    };
    Ok((row.rate_plan_id, rule))
}
