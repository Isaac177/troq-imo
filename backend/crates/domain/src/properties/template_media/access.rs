use kernel::auth::Actor;
use kernel::city::City;
use kernel::error::{Error, Result};
use kernel::permissions::names::{PROPERTIES_CONFIGURE, PROPERTIES_READ};
use sqlx::PgPool;

use crate::properties::media::kind::MediaKind;
use crate::properties::media::service::{self, Owner};
use crate::properties::media::types::PropertyMediaView;
use crate::properties::messages::MEDIA_NOT_FOUND;
use crate::properties::templates::access::template_for;

pub struct Scoped {
    pub owner_id: String,
    pub city: City,
}

impl Scoped {
    pub fn owner(&self) -> Owner<'_> {
        Owner { kind: MediaKind::Template, id: &self.owner_id, city: self.city }
    }
}

pub async fn scoped(pool: &PgPool, actor: &Actor, template_id: &str, write: bool, as_admin: bool) -> Result<Scoped> {
    let permission = if write { PROPERTIES_CONFIGURE } else { PROPERTIES_READ };
    let template = template_for(pool, actor, template_id, permission, as_admin).await?;
    Ok(Scoped { owner_id: template.id, city: City::from_code(&template.city_id)? })
}

pub async fn scoped_media(
    pool: &PgPool,
    actor: &Actor,
    template_id: &str,
    media_id: &str,
    write: bool,
    as_admin: bool,
) -> Result<(Scoped, PropertyMediaView)> {
    let scope = scoped(pool, actor, template_id, write, as_admin).await?;
    let media = service::find(pool, MediaKind::Template, media_id).await?;
    if media.owner_id != scope.owner_id {
        return Err(Error::not_found(MEDIA_NOT_FOUND));
    }
    Ok((scope, media))
}
