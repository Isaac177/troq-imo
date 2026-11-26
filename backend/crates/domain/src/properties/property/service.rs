use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::city::City;
use kernel::error::{codes::NOT_FOUND, Error, Result};
use kernel::idempotency;
use kernel::ids::new_id;
use kernel::outbox;
use kernel::paging::{Page, PageRequest};
use kernel::permissions::names::{PROPERTIES_READ, PROPERTIES_WRITE};
use serde_json::json;
use sqlx::PgPool;

use super::queries::{self, ListScope};
use super::types::{CreatePropertyInput, PropertyListQuery, PropertyView, PROPERTY_STATUSES};
use super::validation::{validate_create, ValidatedProperty};
use crate::properties::messages::{
    HOST_NOT_ACTIVE, HOST_NOT_FOUND, HOST_NOT_MEMBER, PROPERTY_NOT_FOUND, PROPERTY_STATUS_UNKNOWN,
    REQUEST_IN_PROGRESS,
};

const IDEMPOTENCY_SCOPE: &str = "property.create";

pub async fn list(pool: &PgPool, actor: &Actor, query: &PropertyListQuery, page: PageRequest) -> Result<Page<PropertyView>> {
    actor.require(PROPERTIES_READ)?;
    if let Some(status) = query.status.as_deref() {
        if !PROPERTY_STATUSES.contains(&status) {
            return Err(Error::bad_request(PROPERTY_STATUS_UNKNOWN));
        }
    }
    let (full_hosts, property_ids) = actor.scoped_access(PROPERTIES_READ);
    let scope = ListScope {
        member_hosts: actor.host_ids().iter().map(|id| id.as_str().to_string()).collect(),
        full_hosts: full_hosts.iter().map(|id| id.as_str().to_string()).collect(),
        property_ids,
    };
    queries::list(pool, actor.city, query, &scope, page).await
}

pub async fn get(pool: &PgPool, actor: &Actor, property_id: &str) -> Result<PropertyView> {
    crate::properties::scope::property_for(pool, actor, property_id, PROPERTIES_READ).await
}

pub async fn create(pool: &PgPool, actor: &Actor, input: CreatePropertyInput) -> Result<PropertyView> {
    actor.ensure_writable()?;
    actor.require(PROPERTIES_WRITE)?;
    let validated = validate_create(input)?;
    if !actor.access.is_member_of(&validated.host_id) {
        return Err(Error::forbidden(HOST_NOT_MEMBER));
    }
    actor.require_scoped(PROPERTIES_WRITE, Some(&validated.host_id), None)?;
    create_validated(pool, actor, actor.city, validated).await
}

pub async fn create_validated(
    pool: &PgPool,
    actor: &Actor,
    city: City,
    validated: ValidatedProperty,
) -> Result<PropertyView> {
    match queries::host_status(pool, &validated.host_id, city).await?.as_deref() {
        Some("ACTIVE") => {}
        Some(status) => return Err(Error::conflict(HOST_NOT_ACTIVE).with_params(json!({ "status": status }))),
        None => return Err(Error::not_found(HOST_NOT_FOUND)),
    }
    let key = validated.idempotency_key.as_deref();
    if let Some(key) = key {
        if let Some(existing) = idempotency::resource_for(pool, city, IDEMPOTENCY_SCOPE, key).await? {
            return queries::find(pool, &existing)
                .await?
                .ok_or_else(|| Error::internal("idempotent property not readable"));
        }
    }
    let property_id = new_id("prop");
    let mut tx = pool.begin().await?;
    if let Some(key) = key {
        if !idempotency::claim(&mut tx, city, IDEMPOTENCY_SCOPE, key, &property_id).await? {
            return Err(Error::conflict(REQUEST_IN_PROGRESS));
        }
    }
    queries::insert(&mut tx, &property_id, city, &validated).await?;
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "property.create",
        resource_type: "property",
        resource_id: &property_id,
        reason: None,
        change: json!({ "hostId": validated.host_id, "city": city.code() }),
    })
    .await?;
    let owner = crate::properties::events::owner_user(&mut tx, &validated.host_id).await?;
    outbox::enqueue(&mut tx, city, "property", &property_id, "property.created", json!({
        "propertyId": property_id,
        "hostId": validated.host_id,
        "ownerUserId": owner,
        "propertyName": { "en": validated.name_en, "fr": validated.name_fr },
        "city": city.code(),
    }))
    .await?;
    tx.commit().await?;
    queries::find(pool, &property_id)
        .await?
        .ok_or_else(|| Error::not_found(PROPERTY_NOT_FOUND))
}

pub fn not_found() -> Error {
    Error::not_found(NOT_FOUND)
}
