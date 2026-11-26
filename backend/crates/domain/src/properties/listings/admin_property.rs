use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::city::City;
use kernel::error::{Error, Result};
use kernel::outbox;
use kernel::paging::{Page, PageRequest};
use serde_json::json;
use sqlx::PgPool;

use super::apply::apply_content;
use super::events::payload;
use super::guards::{ensure_not_archived, ensure_version};
use super::types::{AdminPropertyDetail, AdminUpdateInput};
use crate::properties::amenities;
use crate::properties::messages::PROPERTY_NOT_FOUND;
use crate::properties::property::queries as property_queries;
use crate::properties::property::service::create_validated;
use crate::properties::property::types::{CreatePropertyInput, PropertyView};
use crate::properties::property::validation::validate_create;
use crate::properties::revisions::queries as revision_queries;
use crate::properties::revisions::types::RevisionSummary;
use crate::properties::scope::{admin_property, ensure_moderator};

pub async fn detail(pool: &PgPool, actor: &Actor, property_id: &str) -> Result<AdminPropertyDetail> {
    let property = admin_property(pool, actor, property_id).await?;
    let (host_name, host_status): (String, String) =
        sqlx::query_as("SELECT display_name, status FROM hosts WHERE id = $1")
            .bind(&property.operator_id)
            .fetch_one(pool)
            .await?;
    Ok(AdminPropertyDetail { property, host_name, host_status })
}

pub async fn revisions(pool: &PgPool, actor: &Actor, property_id: &str, page: PageRequest) -> Result<Page<RevisionSummary>> {
    admin_property(pool, actor, property_id).await?;
    revision_queries::list(pool, property_id, page).await
}

pub async fn create(pool: &PgPool, actor: &Actor, input: CreatePropertyInput) -> Result<PropertyView> {
    actor.ensure_writable()?;
    ensure_moderator(actor)?;
    let validated = validate_create(input)?;
    create_validated(pool, actor, actor.city, validated).await
}

pub async fn update(pool: &PgPool, actor: &Actor, property_id: &str, input: AdminUpdateInput) -> Result<PropertyView> {
    actor.ensure_writable()?;
    ensure_moderator(actor)?;
    let content = input.content.validated()?;
    amenities::ensure_known(pool, &content.amenities).await?;
    let mut tx = pool.begin().await?;
    let property = property_queries::lock(&mut tx, property_id)
        .await?
        .ok_or_else(|| Error::not_found(PROPERTY_NOT_FOUND))?;
    actor.ensure_city(City::from_code(&property.city)?)?;
    ensure_version(property.version, input.expected_version)?;
    ensure_not_archived(&property)?;
    apply_content(&mut tx, &property.id, &content, None).await?;
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "property.admin_update",
        resource_type: "property",
        resource_id: &property.id,
        reason: None,
        change: json!({ "propertyId": property.id, "fromVersion": property.version }),
    })
    .await?;
    let data = payload(&mut tx, &property.id, None, json!({})).await?;
    outbox::enqueue(&mut tx, actor.city, "property", &property.id, "property.admin_updated", data).await?;
    tx.commit().await?;
    property_queries::find(pool, property_id).await?.ok_or_else(|| Error::not_found(PROPERTY_NOT_FOUND))
}
