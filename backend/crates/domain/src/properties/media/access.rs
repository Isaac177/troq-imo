use kernel::auth::Actor;
use kernel::city::City;
use kernel::error::{Error, Result};
use kernel::permissions::names::{PROPERTIES_READ, PROPERTIES_WRITE};
use sqlx::PgPool;

use super::kind::MediaKind;
use super::service::{self, Owner};
use super::types::PropertyMediaView;
use crate::properties::messages::MEDIA_NOT_FOUND;
use crate::properties::scope::property_for_actor;

pub struct Scoped {
    pub owner_id: String,
    pub city: City,
}

impl Scoped {
    pub fn owner(&self) -> Owner<'_> {
        Owner { kind: MediaKind::Property, id: &self.owner_id, city: self.city }
    }
}

pub async fn scoped(pool: &PgPool, actor: &Actor, property_id: &str, write: bool, as_admin: bool) -> Result<Scoped> {
    let permission = if write { PROPERTIES_WRITE } else { PROPERTIES_READ };
    let property = property_for_actor(pool, actor, property_id, permission, as_admin).await?;
    Ok(Scoped { owner_id: property.id, city: City::from_code(&property.city)? })
}

pub async fn scoped_media(
    pool: &PgPool,
    actor: &Actor,
    property_id: &str,
    media_id: &str,
    write: bool,
    as_admin: bool,
) -> Result<(Scoped, PropertyMediaView)> {
    let scope = scoped(pool, actor, property_id, write, as_admin).await?;
    let media = service::find(pool, MediaKind::Property, media_id).await?;
    if media.owner_id != scope.owner_id {
        return Err(Error::not_found(MEDIA_NOT_FOUND));
    }
    Ok((scope, media))
}

pub async fn scoped_media_by_id(
    pool: &PgPool,
    actor: &Actor,
    media_id: &str,
    as_admin: bool,
) -> Result<PropertyMediaView> {
    let media = service::find(pool, MediaKind::Property, media_id).await?;
    scoped(pool, actor, &media.owner_id, false, as_admin).await?;
    Ok(media)
}
