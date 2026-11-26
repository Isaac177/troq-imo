use kernel::{City, Money, Result};
use sqlx::{FromRow, PgPool, Postgres, Transaction};

use super::types::PlanView;

const PLAN_COLUMNS: &str = r#"id, "unitTypeId" AS unit_type_id, "occupancyBasis" AS occupancy_basis,
    "mealPlan" AS meal_plan, "bookingMode" AS booking_mode, status,
    "childNightlyAmount"::text AS child_nightly_amount,
    "cancellationPolicyVersionId" AS cancellation_policy_version_id,
    "paymentPolicyVersionId" AS payment_policy_version_id, version"#;

#[derive(Debug, FromRow)]
pub struct PlanRow {
    pub id: String,
    pub unit_type_id: String,
    pub occupancy_basis: String,
    pub meal_plan: String,
    pub booking_mode: String,
    pub status: String,
    pub child_nightly_amount: String,
    pub cancellation_policy_version_id: String,
    pub payment_policy_version_id: String,
    pub version: i32,
}

pub struct NewPlan<'a> {
    pub id: &'a str,
    pub city: City,
    pub property_id: &'a str,
    pub unit_type_id: &'a str,
    pub occupancy_basis: &'a str,
    pub meal_plan: &'a str,
    pub booking_mode: &'a str,
    pub cancellation_policy_version_id: &'a str,
    pub payment_policy_version_id: &'a str,
    pub child_nightly_amount: Money,
}

pub struct PlanChange<'a> {
    pub status: Option<&'a str>,
    pub booking_mode: Option<&'a str>,
    pub meal_plan: Option<&'a str>,
    pub child_nightly_amount: Option<Money>,
}

pub async fn list_plans(
    pool: &PgPool,
    city: City,
    property_id: &str,
    limit: i64,
    offset: i64,
) -> Result<(Vec<PlanView>, i64)> {
    let total: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM stay_rate_plans WHERE city_id = $1 AND "propertyId" = $2"#,
    )
    .bind(city.code())
    .bind(property_id)
    .fetch_one(pool)
    .await?;
    let rows = sqlx::query_as::<_, PlanRow>(&format!(
        r#"SELECT {PLAN_COLUMNS} FROM stay_rate_plans
           WHERE city_id = $1 AND "propertyId" = $2 ORDER BY id LIMIT $3 OFFSET $4"#
    ))
    .bind(city.code())
    .bind(property_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    let items = rows.into_iter().map(plan_view).collect::<Result<_>>()?;
    Ok((items, total))
}

pub async fn find_plan<'c, E>(
    executor: E,
    city: City,
    property_id: &str,
    plan_id: &str,
) -> Result<Option<PlanRow>>
where
    E: sqlx::Executor<'c, Database = Postgres>,
{
    let row = sqlx::query_as::<_, PlanRow>(&format!(
        r#"SELECT {PLAN_COLUMNS} FROM stay_rate_plans
           WHERE city_id = $1 AND "propertyId" = $2 AND id = $3"#
    ))
    .bind(city.code())
    .bind(property_id)
    .bind(plan_id)
    .fetch_optional(executor)
    .await?;
    Ok(row)
}

pub async fn unit_type_in_property(
    pool: &PgPool,
    city: City,
    unit_type_id: &str,
    property_id: &str,
) -> Result<bool> {
    let found: Option<i32> = sqlx::query_scalar(
        r#"SELECT 1 FROM stay_unit_types WHERE city_id = $1 AND id = $2 AND "propertyId" = $3"#,
    )
    .bind(city.code())
    .bind(unit_type_id)
    .bind(property_id)
    .fetch_optional(pool)
    .await?;
    Ok(found.is_some())
}

pub async fn insert_plan(tx: &mut Transaction<'_, Postgres>, plan: &NewPlan<'_>) -> Result<()> {
    sqlx::query(
        r#"INSERT INTO stay_rate_plans (id, city_id, "propertyId", "unitTypeId", "occupancyBasis", "mealPlan",
               "cancellationPolicyVersionId", "paymentPolicyVersionId", "bookingMode", status, "childNightlyAmount")
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'DRAFT', $10::text::numeric)"#,
    )
    .bind(plan.id)
    .bind(plan.city.code())
    .bind(plan.property_id)
    .bind(plan.unit_type_id)
    .bind(plan.occupancy_basis)
    .bind(plan.meal_plan)
    .bind(plan.cancellation_policy_version_id)
    .bind(plan.payment_policy_version_id)
    .bind(plan.booking_mode)
    .bind(plan.child_nightly_amount.to_wire())
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn update_plan(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    plan_id: &str,
    expected_version: i32,
    change: &PlanChange<'_>,
) -> Result<bool> {
    let result = sqlx::query(
        r#"UPDATE stay_rate_plans SET
               status = COALESCE($4, status),
               "bookingMode" = COALESCE($5, "bookingMode"),
               "mealPlan" = COALESCE($6, "mealPlan"),
               "childNightlyAmount" = COALESCE($7::text::numeric, "childNightlyAmount"),
               version = version + 1,
               "updatedAt" = now()
           WHERE city_id = $1 AND id = $2 AND version = $3"#,
    )
    .bind(city.code())
    .bind(plan_id)
    .bind(expected_version)
    .bind(change.status)
    .bind(change.booking_mode)
    .bind(change.meal_plan)
    .bind(change.child_nightly_amount.map(Money::to_wire))
    .execute(&mut **tx)
    .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn has_base_rate(pool: &PgPool, city: City, plan_id: &str) -> Result<bool> {
    let found: Option<i32> = sqlx::query_scalar(
        r#"SELECT 1 FROM stay_rate_rules WHERE city_id = $1 AND "ratePlanId" = $2
             AND "adjustmentType" = 'OVERRIDE' AND "startDate" IS NULL AND "endDate" IS NULL
             AND "weekdayMask" = 127"#,
    )
    .bind(city.code())
    .bind(plan_id)
    .fetch_optional(pool)
    .await?;
    Ok(found.is_some())
}

pub fn plan_view(row: PlanRow) -> Result<PlanView> {
    Ok(PlanView {
        id: row.id,
        unit_type_id: row.unit_type_id,
        occupancy_basis: row.occupancy_basis,
        meal_plan: row.meal_plan,
        booking_mode: row.booking_mode,
        status: row.status,
        child_nightly_amount: Money::parse(&row.child_nightly_amount)?,
        cancellation_policy_version_id: row.cancellation_policy_version_id,
        payment_policy_version_id: row.payment_policy_version_id,
        version: row.version,
    })
}

