use chrono::{DateTime, Utc};
use kernel::city::City;
use kernel::error::Result;
use serde_json::Value;
use sqlx::{PgPool, Postgres, Transaction};

use super::types::SavedSearchView;

pub async fn count(pool: &PgPool, city: City, guest: &str) -> Result<i64> {
    Ok(sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM stay_saved_searches WHERE city_id = $1 AND "guestUserId" = $2"#,
    )
    .bind(city.code())
    .bind(guest)
    .fetch_one(pool)
    .await?)
}

pub async fn insert(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    id: &str,
    guest: &str,
    parameters: &Value,
    alert_consent: bool,
) -> Result<DateTime<Utc>> {
    Ok(sqlx::query_scalar(
        r#"INSERT INTO stay_saved_searches (id, city_id, "guestUserId", parameters, "alertConsent")
           VALUES ($1, $2, $3, $4, $5) RETURNING "createdAt""#,
    )
    .bind(id)
    .bind(city.code())
    .bind(guest)
    .bind(parameters)
    .bind(alert_consent)
    .fetch_one(&mut **tx)
    .await?)
}

pub async fn list(
    pool: &PgPool,
    city: City,
    guest: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<SavedSearchView>> {
    Ok(sqlx::query_as::<_, SavedSearchView>(
        r#"SELECT id, parameters, "alertConsent" AS alert_consent, "createdAt" AS created_at
           FROM stay_saved_searches WHERE city_id = $1 AND "guestUserId" = $2
           ORDER BY "createdAt", id LIMIT $3 OFFSET $4"#,
    )
    .bind(city.code())
    .bind(guest)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?)
}

pub async fn delete(pool: &PgPool, city: City, guest: &str, saved_search_id: &str) -> Result<bool> {
    let result = sqlx::query(
        r#"DELETE FROM stay_saved_searches WHERE id = $1 AND city_id = $2 AND "guestUserId" = $3"#,
    )
    .bind(saved_search_id)
    .bind(city.code())
    .bind(guest)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() == 1)
}
