use kernel::city::City;
use kernel::error::{Error, Result};
use kernel::idempotency;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sqlx::{PgPool, Postgres, Transaction};

use crate::inventory::messages::IDEMPOTENCY_IN_PROGRESS;

pub const CREATE_SCOPE: &str = "unit_type.create";
pub const UPDATE_SCOPE: &str = "unit_type.update";

pub fn clean(key: Option<&str>) -> Result<Option<&str>> {
    key.map(idempotency::clean_key).transpose()
}

pub async fn replay<T: DeserializeOwned>(
    pool: &PgPool,
    city: City,
    scope: &str,
    key: Option<&str>,
) -> Result<Option<T>> {
    let Some(key) = key else {
        return Ok(None);
    };
    match idempotency::stored_response(pool, city, scope, key).await? {
        Some(stored) => serde_json::from_value(stored)
            .map(Some)
            .map_err(|error| Error::internal(format!("stored response unreadable: {error}"))),
        None => Ok(None),
    }
}

pub async fn replay_or_busy<T: DeserializeOwned>(
    pool: &PgPool,
    city: City,
    scope: &str,
    key: Option<&str>,
) -> Result<T> {
    replay(pool, city, scope, key)
        .await?
        .ok_or_else(|| Error::conflict(IDEMPOTENCY_IN_PROGRESS))
}

pub async fn claim(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    scope: &str,
    key: Option<&str>,
    resource_id: &str,
) -> Result<bool> {
    match key {
        Some(key) => idempotency::claim(tx, city, scope, key, resource_id).await,
        None => Ok(true),
    }
}

pub async fn remember<T: Serialize>(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    scope: &str,
    key: Option<&str>,
    value: &T,
) -> Result<()> {
    let Some(key) = key else {
        return Ok(());
    };
    let stored = serde_json::to_value(value).map_err(|error| Error::internal(error.to_string()))?;
    idempotency::store_response(tx, city, scope, key, &stored).await
}
