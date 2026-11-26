use kernel::auth::Actor;
use kernel::city::City;
use kernel::error::codes::NOT_FOUND;
use kernel::error::{Error, Result};
use kernel::permissions::names::PROPERTIES_MODERATE;
use sqlx::PgPool;

use super::messages::PROPERTY_NOT_FOUND;
use super::property::{queries, types::PropertyView};

pub async fn property_for(pool: &PgPool, actor: &Actor, id: &str, permission: &str) -> Result<PropertyView> {
    let property = queries::find(pool, id).await?.ok_or_else(|| Error::not_found(PROPERTY_NOT_FOUND))?;
    actor.ensure_city(City::from_code(&property.city)?)?;
    actor.ensure_host_scope(permission, &property.operator_id, &property.id)?;
    Ok(property)
}

pub fn ensure_moderator(actor: &Actor) -> Result<()> {
    actor.require_platform(PROPERTIES_MODERATE)
}

pub async fn admin_property(pool: &PgPool, actor: &Actor, id: &str) -> Result<PropertyView> {
    ensure_moderator(actor)?;
    let property = queries::find(pool, id).await?.ok_or_else(|| Error::not_found(PROPERTY_NOT_FOUND))?;
    actor.ensure_city(City::from_code(&property.city)?)?;
    Ok(property)
}

pub async fn property_for_actor(
    pool: &PgPool,
    actor: &Actor,
    id: &str,
    permission: &str,
    as_admin: bool,
) -> Result<PropertyView> {
    if as_admin {
        admin_property(pool, actor, id).await
    } else {
        property_for(pool, actor, id, permission).await
    }
}

pub fn ensure_host_scope(actor: &Actor, host_id: &str, permission: &str) -> Result<()> {
    if !actor.access.is_member_of(host_id) {
        return Err(Error::not_found(NOT_FOUND));
    }
    actor.require_scoped(permission, Some(host_id), None)
}
