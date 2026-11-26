use kernel::{City, Result};
use sqlx::PgPool;

use super::keys::SettingKey;

pub async fn get(pool: &PgPool, city: City, key: SettingKey) -> Result<i32> {
    let value: Option<i32> = sqlx::query_scalar(
        r#"SELECT "valueInt" FROM stay_platform_settings WHERE city_id = $1 AND key = $2"#,
    )
    .bind(city.code())
    .bind(key.code())
    .fetch_optional(pool)
    .await?;
    Ok(value.unwrap_or_else(|| key.default_value()))
}

pub async fn get_i64(pool: &PgPool, city: City, key: SettingKey) -> Result<i64> {
    Ok(i64::from(get(pool, city, key).await?))
}
