use kernel::error::Result;
use kernel::{City, Error};
use sqlx::PgPool;

use super::messages::HOST_NOT_FOUND;

const DEFAULT_COMMISSION_BPS: &str = "DEFAULT_COMMISSION_BPS";
const MIN_CREDIT_CDF: &str = "MIN_CREDIT_CDF";
const LOW_CREDIT_CDF: &str = "LOW_CREDIT_CDF";
const MIN_CREDIT_FALLBACK: i32 = 0;
const LOW_CREDIT_FALLBACK: i32 = 50_000;
const DEFAULT_COMMISSION_FALLBACK: i32 = 0;

async fn value(pool: &PgPool, city: City, key: &str, fallback: i32) -> Result<i32> {
    let stored: Option<i32> = sqlx::query_scalar(
        r#"SELECT "valueInt" FROM stay_platform_settings WHERE city_id = $1 AND key = $2"#,
    )
    .bind(city.code())
    .bind(key)
    .fetch_optional(pool)
    .await?;
    Ok(stored.unwrap_or(fallback))
}

pub async fn default_commission_bps(pool: &PgPool, city: City) -> Result<i32> {
    value(pool, city, DEFAULT_COMMISSION_BPS, DEFAULT_COMMISSION_FALLBACK).await
}

pub async fn min_credit_cdf(pool: &PgPool, city: City) -> Result<i32> {
    value(pool, city, MIN_CREDIT_CDF, MIN_CREDIT_FALLBACK).await
}

pub async fn low_credit_cdf(pool: &PgPool, city: City) -> Result<i32> {
    value(pool, city, LOW_CREDIT_CDF, LOW_CREDIT_FALLBACK).await
}

pub async fn host_city(pool: &PgPool, host_id: &str) -> Result<City> {
    let code: Option<String> = sqlx::query_scalar("SELECT city_id FROM hosts WHERE id = $1")
        .bind(host_id)
        .fetch_optional(pool)
        .await?;
    match code {
        Some(code) => City::from_code(&code),
        None => Err(Error::not_found(HOST_NOT_FOUND)),
    }
}
