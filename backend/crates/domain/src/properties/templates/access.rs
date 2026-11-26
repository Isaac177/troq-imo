use kernel::auth::Actor;
use kernel::city::City;
use kernel::error::{Error, Result};
use sqlx::{PgConnection, PgPool};

use super::queries;
use super::types::TemplateView;
use crate::properties::messages::{HOST_NOT_FOUND, TEMPLATE_NOT_FOUND};
use crate::properties::scope::{ensure_host_scope, ensure_moderator};

pub fn ensure_member(actor: &Actor, host_id: &str, permission: &str, as_admin: bool) -> Result<()> {
    if as_admin {
        ensure_moderator(actor)
    } else {
        ensure_host_scope(actor, host_id, permission)
    }
}

pub async fn template_for(
    pool: &PgPool,
    actor: &Actor,
    template_id: &str,
    permission: &str,
    as_admin: bool,
) -> Result<TemplateView> {
    let template = queries::find(pool, template_id)
        .await?
        .ok_or_else(|| Error::not_found(TEMPLATE_NOT_FOUND))?;
    ensure_member(actor, &template.operator_id, permission, as_admin)?;
    actor.ensure_city(City::from_code(&template.city_id)?)?;
    Ok(template)
}

pub async fn ensure_host_city(conn: &mut PgConnection, host_id: &str, city: City) -> Result<()> {
    let host: Option<String> = sqlx::query_scalar("SELECT id FROM hosts WHERE id = $1 AND city_id = $2 FOR SHARE")
        .bind(host_id.trim())
        .bind(city.code())
        .fetch_optional(&mut *conn)
        .await?;
    host.map(|_| ()).ok_or_else(|| Error::not_found(HOST_NOT_FOUND))
}
