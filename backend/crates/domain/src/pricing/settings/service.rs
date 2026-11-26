use chrono::{DateTime, Utc};
use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::permissions::names::PROPERTIES_MODERATE;
use kernel::{Error, Result};
use serde_json::json;
use sqlx::PgPool;

use super::keys::{SettingKey, ALL_KEYS, UPDATE_REASONS};
use super::store;
use super::types::{SettingView, SettingsView, UpdateSettingInput, UpdatedSetting};
use crate::pricing::messages::{SETTING_REASON_UNKNOWN, SETTING_UNCHANGED};

type StoredRow = (String, i32, Option<String>, DateTime<Utc>);

pub async fn list(pool: &PgPool, actor: &Actor) -> Result<SettingsView> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    let stored: Vec<StoredRow> = sqlx::query_as(
        r#"SELECT key, "valueInt", "updatedByUserId", "updatedAt"
           FROM stay_platform_settings WHERE city_id = $1"#,
    )
    .bind(actor.city.code())
    .fetch_all(pool)
    .await?;
    let items = ALL_KEYS
        .into_iter()
        .map(|key| {
            let row = stored.iter().find(|row| row.0 == key.code());
            let value = row.map_or_else(|| key.default_value(), |row| row.1);
            let (min, max) = key.range();
            SettingView {
                key: key.code(),
                value,
                default_value: key.default_value(),
                min,
                max,
                unit: key.unit(),
                changed: value != key.default_value(),
                updated_by_user_id: row.and_then(|row| row.2.clone()),
                updated_at: row.filter(|row| row.2.is_some()).map(|row| row.3),
            }
        })
        .collect();
    Ok(SettingsView { city: actor.city.code().to_string(), items })
}

pub async fn update(
    pool: &PgPool,
    actor: &Actor,
    key_code: &str,
    input: UpdateSettingInput,
) -> Result<UpdatedSetting> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    let key = SettingKey::parse(key_code)?;
    key.ensure_in_range(input.value)?;
    if !UPDATE_REASONS.contains(&input.reason_code.as_str()) {
        return Err(Error::bad_request(SETTING_REASON_UNKNOWN).with_params(json!({ "allowed": UPDATE_REASONS })));
    }
    let previous = store::get(pool, actor.city, key).await?;
    if previous == input.value {
        return Err(Error::conflict(SETTING_UNCHANGED));
    }
    let mut tx = pool.begin().await?;
    sqlx::query(
        r#"INSERT INTO stay_platform_settings (city_id, key, "valueInt", "updatedByUserId", "updatedAt")
           VALUES ($1, $2, $3, $4, now())
           ON CONFLICT (city_id, key) DO UPDATE SET "valueInt" = EXCLUDED."valueInt",
               "updatedByUserId" = EXCLUDED."updatedByUserId", "updatedAt" = now()"#,
    )
    .bind(actor.city.code())
    .bind(key.code())
    .bind(input.value)
    .bind(actor.user_id().as_str())
    .execute(&mut *tx)
    .await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "settings.updated",
            resource_type: "settings",
            resource_id: key.code(),
            reason: Some(input.reason_code.as_str()),
            change: json!({ "from": previous, "to": input.value }),
        },
    )
    .await?;
    tx.commit().await?;
    Ok(UpdatedSetting { key: key.code(), value: input.value })
}
