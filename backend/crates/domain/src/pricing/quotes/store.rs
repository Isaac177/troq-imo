use chrono::{DateTime, NaiveDate, Utc};
use kernel::{City, Result};
use serde_json::Value;
use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow)]
pub struct QuoteRow {
    pub id: String,
    pub session_ref: String,
    pub property_id: String,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub adult_count: i32,
    pub child_count: i32,
    pub total_amount: String,
    pub breakdown: Value,
    pub expires_at: DateTime<Utc>,
}

pub struct NewQuote<'a> {
    pub id: &'a str,
    pub city: City,
    pub session_ref: &'a str,
    pub property_id: &'a str,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub adult_count: i32,
    pub child_count: i32,
    pub total: &'a str,
    pub breakdown: &'a Value,
    pub policy_ids: &'a [String],
    pub expires_at: DateTime<Utc>,
}

pub async fn insert(pool: &PgPool, quote: &NewQuote<'_>) -> Result<()> {
    sqlx::query(
        r#"INSERT INTO stay_quotes (id, "sessionRef", "propertyId", city_id, "checkIn", "checkOut",
               "adultCount", "childCount", currency, "totalAmount", breakdown, "policyVersionIds", "expiresAt")
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'CDF', $9::text::numeric, $10, $11, $12)"#,
    )
    .bind(quote.id)
    .bind(quote.session_ref)
    .bind(quote.property_id)
    .bind(quote.city.code())
    .bind(quote.check_in)
    .bind(quote.check_out)
    .bind(quote.adult_count)
    .bind(quote.child_count)
    .bind(quote.total)
    .bind(quote.breakdown)
    .bind(serde_json::json!(quote.policy_ids))
    .bind(quote.expires_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find(pool: &PgPool, city: City, quote_id: &str) -> Result<Option<QuoteRow>> {
    let row = sqlx::query_as::<_, QuoteRow>(
        r#"SELECT id, "sessionRef" AS session_ref, "propertyId" AS property_id,
                  "checkIn" AS check_in, "checkOut" AS check_out, "adultCount" AS adult_count,
                  "childCount" AS child_count, "totalAmount"::text AS total_amount,
                  breakdown, "expiresAt" AS expires_at
           FROM stay_quotes WHERE city_id = $1 AND id = $2"#,
    )
    .bind(city.code())
    .bind(quote_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}
