use kernel::auth::Actor;
use kernel::error::{Error, Result};
use kernel::permissions::names::PROPERTIES_MODERATE;
use kernel::City;
use sqlx::{FromRow, PgPool};

use crate::pricing::messages::PROPERTY_NOT_FOUND;

#[derive(Debug, Clone, FromRow)]
pub struct PropertyScope {
    pub id: String,
    pub host_id: String,
    pub city_id: String,
}

pub async fn find(pool: &PgPool, property_id: &str) -> Result<Option<PropertyScope>> {
    let row = sqlx::query_as::<_, PropertyScope>(
        r#"SELECT id, "operatorId" AS host_id, city_id
           FROM stay_properties WHERE id = $1"#,
    )
    .bind(property_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn for_host(
    pool: &PgPool,
    actor: &Actor,
    property_id: &str,
    permission: &str,
) -> Result<PropertyScope> {
    let property = existing(pool, actor, property_id).await?;
    actor.ensure_host_scope(permission, &property.host_id, &property.id)?;
    Ok(property)
}

pub async fn for_admin(pool: &PgPool, actor: &Actor, property_id: &str) -> Result<PropertyScope> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    existing(pool, actor, property_id).await
}

pub async fn resolve(
    pool: &PgPool,
    actor: &Actor,
    property_id: &str,
    permission: &str,
    as_admin: bool,
) -> Result<PropertyScope> {
    if as_admin {
        for_admin(pool, actor, property_id).await
    } else {
        for_host(pool, actor, property_id, permission).await
    }
}

async fn existing(pool: &PgPool, actor: &Actor, property_id: &str) -> Result<PropertyScope> {
    let property = find(pool, property_id)
        .await?
        .ok_or_else(|| Error::not_found(PROPERTY_NOT_FOUND))?;
    actor.ensure_city(City::from_code(&property.city_id)?)?;
    Ok(property)
}
