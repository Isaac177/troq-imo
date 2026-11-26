use kernel::auth::Actor;
use kernel::city::City;
use kernel::error::Result;
use kernel::permissions::names::PROPERTIES_MODERATE;
use kernel::Error;
use sqlx::{PgPool, Postgres, Transaction};

use super::messages::PROPERTY_NOT_FOUND;

#[derive(Debug, Clone)]
pub struct PropertyRef {
    pub id: String,
    pub host_id: String,
    pub city: City,
    pub timezone: String,
}

pub async fn load(pool: &PgPool, property_id: &str) -> Result<PropertyRef> {
    let row: Option<(String, String, String, String)> = sqlx::query_as(
        r#"SELECT id, "operatorId", city_id, timezone FROM stay_properties WHERE id = $1"#,
    )
    .bind(property_id)
    .fetch_optional(pool)
    .await?;
    let (id, host_id, city, timezone) =
        row.ok_or_else(|| Error::not_found(PROPERTY_NOT_FOUND))?;
    Ok(PropertyRef { id, host_id, city: City::from_code(&city)?, timezone })
}

pub async fn host_property(
    pool: &PgPool,
    actor: &Actor,
    property_id: &str,
    permission: &str,
) -> Result<PropertyRef> {
    let property = load(pool, property_id).await?;
    actor.ensure_city(property.city)?;
    actor.ensure_host_scope(permission, &property.host_id, &property.id)?;
    Ok(property)
}

pub async fn admin_property(pool: &PgPool, actor: &Actor, property_id: &str) -> Result<PropertyRef> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    let property = load(pool, property_id).await?;
    actor.ensure_city(property.city)?;
    Ok(property)
}

pub async fn actor_property(
    pool: &PgPool,
    actor: &Actor,
    property_id: &str,
    permission: &str,
    as_admin: bool,
) -> Result<PropertyRef> {
    if as_admin {
        admin_property(pool, actor, property_id).await
    } else {
        host_property(pool, actor, property_id, permission).await
    }
}

pub async fn lock_property(tx: &mut Transaction<'_, Postgres>, property_id: &str) -> Result<()> {
    sqlx::query("SELECT id FROM stay_properties WHERE id = $1 FOR UPDATE")
        .bind(property_id)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

pub async fn unit_type_belongs(
    pool: &PgPool,
    property_id: &str,
    unit_type_id: &str,
) -> Result<bool> {
    let found: bool = sqlx::query_scalar(
        r#"SELECT EXISTS (SELECT 1 FROM stay_unit_types WHERE id = $1 AND "propertyId" = $2)"#,
    )
    .bind(unit_type_id)
    .bind(property_id)
    .fetch_one(pool)
    .await?;
    Ok(found)
}
