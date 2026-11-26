use kernel::auth::Actor;
use kernel::error::{Error, Result};
use kernel::paging::{Page, PageRequest};
use sqlx::PgPool;

use super::types::{AdminRevisionView, LifecycleInput, QueueItem};
use super::{admin_queue, lifecycle, queries, revision_view};
use crate::properties::messages::REVISION_NOT_FOUND;
use crate::properties::property::types::PropertyView;
use crate::properties::revisions::content::ListingContent;
use crate::properties::scope::{admin_property, property_for};
use kernel::permissions::names::PROPERTIES_WRITE;

pub async fn host_lifecycle(pool: &PgPool, actor: &Actor, property_id: &str, input: LifecycleInput) -> Result<PropertyView> {
    property_for(pool, actor, property_id, PROPERTIES_WRITE).await?;
    lifecycle::apply(pool, actor, property_id, input, false).await
}

pub async fn admin_lifecycle(pool: &PgPool, actor: &Actor, property_id: &str, input: LifecycleInput) -> Result<PropertyView> {
    admin_property(pool, actor, property_id).await?;
    lifecycle::apply(pool, actor, property_id, input, true).await
}

pub async fn admin_queue(pool: &PgPool, actor: &Actor, status: Option<String>, page: PageRequest) -> Result<Page<QueueItem>> {
    crate::properties::scope::ensure_moderator(actor)?;
    let status = admin_queue::queue_status(status)?;
    admin_queue::queue(pool, actor.city, &status, page).await
}

pub async fn admin_revision(pool: &PgPool, actor: &Actor, revision_id: &str) -> Result<AdminRevisionView> {
    crate::properties::scope::ensure_moderator(actor)?;
    let row = queries::find_revision(pool, revision_id).await?.ok_or_else(|| Error::not_found(REVISION_NOT_FOUND))?;
    let property = admin_property(pool, actor, &row.summary.property_id).await?;
    let live = ListingContent::from_property(&property)?;
    Ok(AdminRevisionView { revision: revision_view::view(row)?, live })
}
