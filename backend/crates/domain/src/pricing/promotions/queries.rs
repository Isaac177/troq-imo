use chrono::NaiveDate;
use kernel::{City, Error, Money, Result};
use sqlx::{FromRow, PgConnection, PgPool, Postgres, Transaction};

use super::engine::Promotion;
use super::types::PromotionView;
use crate::pricing::amounts::parse_stored;
use crate::pricing::messages::{PROMOTION_BUDGET_USED, PROMOTION_GONE};

const REDEEMED_STATES: &str = "('REQUESTED', 'CONFIRMED', 'CHECKED_IN', 'CHECKED_OUT')";

#[derive(Debug, FromRow)]
pub struct PromotionRow {
    pub id: String,
    pub rate_plan_id: Option<String>,
    pub min_nights: i32,
    pub discount_percent: String,
    pub cap_amount: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub priority: i32,
    pub redemption_limit: Option<i32>,
    pub redeemed: i64,
    pub status: String,
}

#[derive(Debug, FromRow)]
struct ActiveRow {
    id: String,
    min_nights: i32,
    percent: String,
    cap: Option<String>,
    priority: i32,
    redemption_limit: Option<i32>,
    redeemed: i64,
}

#[derive(Debug, FromRow)]
struct ScopedRow {
    property_id: String,
    rate_plan_id: Option<String>,
    #[sqlx(flatten)]
    active: ActiveRow,
}

pub struct NewPromotion<'a> {
    pub id: &'a str,
    pub city: City,
    pub property_id: &'a str,
    pub rate_plan_id: Option<&'a str>,
    pub min_nights: i32,
    pub discount_percent: Money,
    pub cap_amount: Option<Money>,
    pub start: Option<NaiveDate>,
    pub end: Option<NaiveDate>,
    pub priority: i32,
    pub redemption_limit: Option<i32>,
}

fn select_sql(where_clause: &str) -> String {
    format!(
        r#"SELECT p.id, p."ratePlanId" AS rate_plan_id, p."minNights" AS min_nights,
                  p."discountPercent"::text AS discount_percent, p."capAmount"::text AS cap_amount,
                  p."startDate" AS start_date, p."endDate" AS end_date, p.priority,
                  p."redemptionLimit" AS redemption_limit, p.status,
                  (SELECT COUNT(*) FROM stay_reservations r
                    WHERE r."contractSnapshot" -> 'discount' ->> 'promotionId' = p.id
                      AND r.state IN {REDEEMED_STATES}) AS redeemed
           FROM stay_promotions p {where_clause}"#
    )
}

pub async fn list(
    pool: &PgPool,
    city: City,
    property_id: &str,
    limit: i64,
    offset: i64,
) -> Result<(Vec<PromotionView>, i64)> {
    let total: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM stay_promotions WHERE city_id = $1 AND "propertyId" = $2"#,
    )
    .bind(city.code())
    .bind(property_id)
    .fetch_one(pool)
    .await?;
    let sql = format!(
        "{} ORDER BY p.id LIMIT $3 OFFSET $4",
        select_sql(r#"WHERE p.city_id = $1 AND p."propertyId" = $2"#)
    );
    let rows = sqlx::query_as::<_, PromotionRow>(&sql)
        .bind(city.code())
        .bind(property_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
    let items = rows.into_iter().map(promotion_view).collect::<Result<_>>()?;
    Ok((items, total))
}

pub async fn find<'c, E>(
    executor: E,
    city: City,
    property_id: &str,
    promotion_id: &str,
) -> Result<Option<PromotionRow>>
where
    E: sqlx::Executor<'c, Database = Postgres>,
{
    let sql = select_sql(r#"WHERE p.city_id = $1 AND p."propertyId" = $2 AND p.id = $3"#);
    let row = sqlx::query_as::<_, PromotionRow>(&sql)
        .bind(city.code())
        .bind(property_id)
        .bind(promotion_id)
        .fetch_optional(executor)
        .await?;
    Ok(row)
}

pub async fn insert(tx: &mut Transaction<'_, Postgres>, promotion: &NewPromotion<'_>) -> Result<()> {
    sqlx::query(
        r#"INSERT INTO stay_promotions (id, city_id, "propertyId", "ratePlanId", kind, "minNights",
               "discountPercent", "capAmount", "startDate", "endDate", priority, status, "redemptionLimit")
           VALUES ($1, $2, $3, $4, 'LENGTH_OF_STAY', $5, $6::text::numeric, $7::text::numeric, $8, $9, $10, 'ACTIVE', $11)"#,
    )
    .bind(promotion.id)
    .bind(promotion.city.code())
    .bind(promotion.property_id)
    .bind(promotion.rate_plan_id)
    .bind(promotion.min_nights)
    .bind(promotion.discount_percent.to_wire())
    .bind(promotion.cap_amount.map(Money::to_wire))
    .bind(promotion.start)
    .bind(promotion.end)
    .bind(promotion.priority)
    .bind(promotion.redemption_limit)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn set_status_and_limit(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    promotion_id: &str,
    status: Option<&str>,
    redemption_limit: Option<i32>,
) -> Result<()> {
    sqlx::query(
        r#"UPDATE stay_promotions SET status = COALESCE($3, status),
               "redemptionLimit" = COALESCE($4, "redemptionLimit")
           WHERE city_id = $1 AND id = $2"#,
    )
    .bind(city.code())
    .bind(promotion_id)
    .bind(status)
    .bind(redemption_limit)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn load_active(
    pool: &PgPool,
    city: City,
    property_id: &str,
    rate_plan_id: &str,
    check_in: NaiveDate,
    check_out: NaiveDate,
) -> Result<Vec<Promotion>> {
    let rows = sqlx::query_as::<_, ActiveRow>(
        r#"SELECT p.id, p."minNights" AS min_nights, p."discountPercent"::text AS percent,
                  p."capAmount"::text AS cap, p.priority, p."redemptionLimit" AS redemption_limit,
                  (SELECT COUNT(*) FROM stay_reservations r
                    WHERE r."contractSnapshot" -> 'discount' ->> 'promotionId' = p.id
                      AND r.state IN ('REQUESTED', 'CONFIRMED', 'CHECKED_IN', 'CHECKED_OUT')) AS redeemed
           FROM stay_promotions p
           WHERE p.city_id = $1 AND p."propertyId" = $2 AND p.status = 'ACTIVE'
             AND (p."ratePlanId" IS NULL OR p."ratePlanId" = $3)
             AND (p."startDate" IS NULL OR p."startDate" <= $4)
             AND (p."endDate" IS NULL OR p."endDate" >= $5)
           ORDER BY p.id"#,
    )
    .bind(city.code())
    .bind(property_id)
    .bind(rate_plan_id)
    .bind(check_in)
    .bind(check_out)
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(to_promotion).collect()
}

pub async fn active_for_properties(
    connection: &mut PgConnection,
    city: City,
    property_ids: &[String],
    check_in: NaiveDate,
    check_out: NaiveDate,
) -> Result<Vec<(String, Option<String>, Promotion)>> {
    let rows = sqlx::query_as::<_, ScopedRow>(
        r#"SELECT p."propertyId" AS property_id, p."ratePlanId" AS rate_plan_id, p.id,
                  p."minNights" AS min_nights, p."discountPercent"::text AS percent,
                  p."capAmount"::text AS cap, p.priority, p."redemptionLimit" AS redemption_limit,
                  (SELECT COUNT(*) FROM stay_reservations r
                    WHERE r."contractSnapshot" -> 'discount' ->> 'promotionId' = p.id
                      AND r.state IN ('REQUESTED', 'CONFIRMED', 'CHECKED_IN', 'CHECKED_OUT')) AS redeemed
           FROM stay_promotions p
           WHERE p.city_id = $1 AND p."propertyId" = ANY($2) AND p.status = 'ACTIVE'
             AND (p."startDate" IS NULL OR p."startDate" <= $3)
             AND (p."endDate" IS NULL OR p."endDate" >= $4)
           ORDER BY p.id"#,
    )
    .bind(city.code())
    .bind(property_ids)
    .bind(check_in)
    .bind(check_out)
    .fetch_all(connection)
    .await?;
    rows.into_iter()
        .map(|row| Ok((row.property_id, row.rate_plan_id, to_promotion(row.active)?)))
        .collect()
}

pub async fn lock_budget(tx: &mut Transaction<'_, Postgres>, promotion_id: &str) -> Result<()> {
    let limit: Option<Option<i32>> = sqlx::query_scalar(
        r#"SELECT "redemptionLimit" FROM stay_promotions WHERE id = $1 FOR UPDATE"#,
    )
    .bind(promotion_id)
    .fetch_optional(&mut **tx)
    .await?;
    let Some(limit) = limit else {
        return Err(Error::conflict(PROMOTION_GONE));
    };
    let Some(limit) = limit else {
        return Ok(());
    };
    let redeemed: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM stay_reservations
           WHERE "contractSnapshot" -> 'discount' ->> 'promotionId' = $1
             AND state IN ('REQUESTED', 'CONFIRMED', 'CHECKED_IN', 'CHECKED_OUT')"#,
    )
    .bind(promotion_id)
    .fetch_one(&mut **tx)
    .await?;
    if redeemed >= i64::from(limit) {
        return Err(Error::conflict(PROMOTION_BUDGET_USED));
    }
    Ok(())
}

fn to_promotion(row: ActiveRow) -> Result<Promotion> {
    let percent_minor = parse_stored(&row.percent)?;
    if percent_minor <= 0 || percent_minor > 10_000 {
        return Err(Error::internal(format!("promotion {} percent out of range", row.id)));
    }
    Ok(Promotion {
        id: row.id,
        min_nights: i64::from(row.min_nights),
        percent_minor,
        cap_minor: row.cap.as_deref().map(parse_stored).transpose()?,
        priority: row.priority,
        redemption_limit: row.redemption_limit.map(i64::from),
        redeemed: row.redeemed,
    })
}

pub fn promotion_view(row: PromotionRow) -> Result<PromotionView> {
    Ok(PromotionView {
        id: row.id,
        rate_plan_id: row.rate_plan_id,
        min_nights: i64::from(row.min_nights),
        discount_percent: Money::parse(&row.discount_percent)?,
        cap_amount: row.cap_amount.as_deref().map(Money::parse).transpose()?,
        start_date: row.start_date,
        end_date: row.end_date,
        priority: row.priority,
        redemption_limit: row.redemption_limit,
        redeemed: row.redeemed,
        status: row.status,
    })
}
