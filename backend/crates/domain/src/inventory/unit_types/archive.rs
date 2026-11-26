use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::error::{Error, Result};
use kernel::permissions::names::PROPERTIES_CONFIGURE;
use serde_json::json;
use sqlx::PgPool;

use super::queries::{has_reservations, load};
use super::types::UnitTypeView;
use crate::inventory::messages::{UNIT_TYPE_HAS_RESERVATIONS, UNIT_TYPE_VERSION_CONFLICT};
use crate::inventory::pools::retire_units;
use crate::inventory::scope::actor_property;

pub async fn archive(
    pool: &PgPool,
    actor: &Actor,
    property_id: &str,
    unit_type_id: &str,
    expected_version: i32,
    as_admin: bool,
) -> Result<UnitTypeView> {
    let property = actor_property(pool, actor, property_id, PROPERTIES_CONFIGURE, as_admin).await?;
    let mut tx = pool.begin().await?;
    let current = load(&mut tx, &property.id, unit_type_id).await?;
    if current.version != expected_version {
        return Err(Error::conflict(UNIT_TYPE_VERSION_CONFLICT));
    }
    if has_reservations(&mut tx, &current.id).await? {
        return Err(Error::conflict(UNIT_TYPE_HAS_RESERVATIONS));
    }
    sqlx::query(r#"UPDATE stay_rate_plans SET status = 'ARCHIVED' WHERE "unitTypeId" = $1"#)
        .bind(&current.id)
        .execute(&mut *tx)
        .await?;
    retire_units(&mut tx, &current.id).await?;
    sqlx::query(
        r#"UPDATE stay_unit_types SET status = 'ARCHIVED', version = version + 1, "updatedAt" = now()
           WHERE id = $1 AND "propertyId" = $2"#,
    )
    .bind(&current.id)
    .bind(&property.id)
    .execute(&mut *tx)
    .await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "unit_type.archive",
            resource_type: "unit_type",
            resource_id: &current.id,
            reason: None,
            change: json!({ "sellableQuantity": current.sellable_quantity }),
        },
    )
    .await?;
    let view = load(&mut tx, &property.id, &current.id).await?;
    tx.commit().await?;
    Ok(view)
}
