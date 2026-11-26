use kernel::auth::Actor;
use kernel::error::Result;
use kernel::paging::{Page, PageRequest};
use kernel::permissions::names::PROPERTIES_READ;
use kernel::storage::Storage;
use sqlx::PgPool;

use super::queries::{attach_images, count_active, list_active};
use super::types::UnitTypeView;
use crate::inventory::scope::actor_property;

pub async fn list(
    pool: &PgPool,
    storage: &Storage,
    actor: &Actor,
    property_id: &str,
    page: PageRequest,
    as_admin: bool,
) -> Result<Page<UnitTypeView>> {
    let property = actor_property(pool, actor, property_id, PROPERTIES_READ, as_admin).await?;
    let total = count_active(pool, &property.id).await?;
    let mut items = list_active(pool, &property.id, page.limit(), page.offset()).await?;
    attach_images(pool, storage, &mut items).await?;
    Ok(Page::new(items, total, page))
}
