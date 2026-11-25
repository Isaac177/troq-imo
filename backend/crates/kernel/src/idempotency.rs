use serde_json::Value;
use sqlx::{PgPool, Postgres, Transaction};

use crate::city::City;
use crate::error::codes::IDEMPOTENCY_KEY_INVALID;
use crate::error::{Error, Result};

const MAX_KEY_LENGTH: usize = 128;

pub fn clean_key(value: &str) -> Result<&str> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.len() > MAX_KEY_LENGTH {
        return Err(Error::bad_request(IDEMPOTENCY_KEY_INVALID)
            .with_params(serde_json::json!({ "maxLength": MAX_KEY_LENGTH })));
    }
    Ok(trimmed)
}

pub async fn claim(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    scope: &str,
    key: &str,
    resource_id: &str,
) -> Result<bool> {
    let result = sqlx::query(
        r#"INSERT INTO kernel_idempotency_keys (city_id, scope, idempotency_key, resource_id)
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (city_id, scope, idempotency_key) DO NOTHING"#,
    )
    .bind(city.code())
    .bind(scope)
    .bind(key)
    .bind(resource_id)
    .execute(&mut **tx)
    .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn stored_response(
    pool: &PgPool,
    city: City,
    scope: &str,
    key: &str,
) -> Result<Option<Value>> {
    let response = sqlx::query_scalar::<_, Option<Value>>(
        r#"SELECT response FROM kernel_idempotency_keys
           WHERE city_id = $1 AND scope = $2 AND idempotency_key = $3"#,
    )
    .bind(city.code())
    .bind(scope)
    .bind(key)
    .fetch_optional(pool)
    .await?;
    Ok(response.flatten())
}

pub async fn store_response(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    scope: &str,
    key: &str,
    response: &Value,
) -> Result<()> {
    sqlx::query(
        r#"UPDATE kernel_idempotency_keys SET response = $4
           WHERE city_id = $1 AND scope = $2 AND idempotency_key = $3"#,
    )
    .bind(city.code())
    .bind(scope)
    .bind(key)
    .bind(response)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn resource_for(
    pool: &PgPool,
    city: City,
    scope: &str,
    key: &str,
) -> Result<Option<String>> {
    let resource_id = sqlx::query_scalar::<_, String>(
        r#"SELECT resource_id FROM kernel_idempotency_keys
           WHERE city_id = $1 AND scope = $2 AND idempotency_key = $3"#,
    )
    .bind(city.code())
    .bind(scope)
    .bind(key)
    .fetch_optional(pool)
    .await?;
    Ok(resource_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_are_trimmed_and_bounded() {
        assert_eq!(clean_key("  abc ").unwrap(), "abc");
        assert!(clean_key("").is_err());
        assert!(clean_key(&"k".repeat(129)).is_err());
    }
}
