use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::error::{Error, Result};
use kernel::outbox;
use kernel::permissions::names::INVENTORY_WRITE;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::inventory::messages::INVENTORY_VERSION_CONFLICT;
use crate::inventory::scope::{actor_property, lock_property};

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InventoryDefaultsInput {
    pub property_id: String,
    pub unit_type_id: String,
    pub expected_version: i32,
    pub enabled: bool,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InventoryDefaultsResult {
    pub unit_type_id: String,
    pub enabled: bool,
    pub version: i32,
}

pub async fn update(
    pool: &PgPool,
    actor: &Actor,
    input: InventoryDefaultsInput,
    as_admin: bool,
) -> Result<InventoryDefaultsResult> {
    let property = actor_property(pool, actor, &input.property_id, INVENTORY_WRITE, as_admin).await?;
    let mut tx = pool.begin().await?;
    lock_property(&mut tx, &property.id).await?;
    let version = sqlx::query_scalar::<_, i32>(
        r#"UPDATE stay_inventory_pools SET "defaultNightlyCapacity" = $4, version = version + 1
           WHERE "propertyId" = $1 AND "unitTypeId" = $2 AND version = $3
           RETURNING version"#,
    )
    .bind(&property.id)
    .bind(&input.unit_type_id)
    .bind(input.expected_version)
    .bind(input.enabled)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| Error::conflict(INVENTORY_VERSION_CONFLICT))?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "inventory.default_capacity_changed",
            resource_type: "property",
            resource_id: &property.id,
            reason: None,
            change: json!({ "unitTypeId": input.unit_type_id, "enabled": input.enabled }),
        },
    )
    .await?;
    outbox::enqueue(
        &mut tx,
        property.city,
        "property",
        &property.id,
        "property.inventory_changed",
        json!({ "propertyId": property.id, "unitTypeId": input.unit_type_id }),
    )
    .await?;
    tx.commit().await?;
    Ok(InventoryDefaultsResult { unit_type_id: input.unit_type_id, enabled: input.enabled, version })
}
