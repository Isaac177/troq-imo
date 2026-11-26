use kernel::audit::{self, AuditActor, AuditEntry};
use kernel::error::{Error, Result};
use kernel::idempotency;
use kernel::ids::new_id;
use kernel::outbox;
use serde_json::json;
use sqlx::{PgPool, Postgres, Transaction};

use super::queries::{self, NewRestriction};
use super::types::{BulkRestrictionInput, BulkRestrictionResult, RestrictionOperation, RestrictionView};
use super::validation::{
    validate_basis, validate_kind, validate_value, validate_window, MAX_BULK_OPERATIONS,
};
use crate::inventory::messages::{
    BULK_ACTION_UNKNOWN, BULK_SIZE_INVALID, IDEMPOTENCY_IN_PROGRESS, RESTRICTION_ID_REQUIRED,
    RESTRICTION_KIND_REQUIRED, RESTRICTION_NOT_FOUND, UNIT_TYPE_NOT_FOUND,
};
use crate::inventory::scope::{lock_property, PropertyRef};

const BULK_SCOPE: &str = "restriction.bulk";

pub async fn apply_bulk_edit(
    pool: &PgPool,
    property: &PropertyRef,
    actor: &AuditActor,
    input: BulkRestrictionInput,
) -> Result<BulkRestrictionResult> {
    if input.operations.is_empty() || input.operations.len() > MAX_BULK_OPERATIONS {
        return Err(Error::bad_request(BULK_SIZE_INVALID)
            .with_params(json!({ "min": 1, "max": MAX_BULK_OPERATIONS })));
    }
    let key = input.idempotency_key.as_deref().map(idempotency::clean_key).transpose()?;
    if let Some(key) = key {
        if let Some(stored) = idempotency::stored_response(pool, property.city, BULK_SCOPE, key).await? {
            return serde_json::from_value(stored).map_err(|error| Error::internal(error.to_string()));
        }
    }
    let mut tx = pool.begin().await?;
    if let Some(key) = key {
        let marker = new_id("bulk");
        if !idempotency::claim(&mut tx, property.city, BULK_SCOPE, key, &marker).await? {
            return Err(Error::conflict(IDEMPOTENCY_IN_PROGRESS));
        }
    }
    lock_property(&mut tx, &property.id).await?;
    let mut result = BulkRestrictionResult { applied: Vec::new(), removed: Vec::new(), affected_nights: 0 };
    for operation in &input.operations {
        match operation.action.as_str() {
            "ADD" => {
                let added = add_one(&mut tx, property, actor, operation).await?;
                result.affected_nights += affected(operation);
                result.applied.push(added);
            }
            "REMOVE" => {
                let id = operation
                    .restriction_id
                    .as_deref()
                    .ok_or_else(|| Error::bad_request(RESTRICTION_ID_REQUIRED))?;
                if queries::find_active(&mut tx, &property.id, id).await?.is_none() {
                    return Err(Error::not_found(RESTRICTION_NOT_FOUND).with_params(json!({ "restrictionId": id })));
                }
                queries::remove(&mut tx, id).await?;
                result.removed.push(id.to_string());
            }
            other => {
                return Err(Error::bad_request(BULK_ACTION_UNKNOWN).with_params(json!({ "action": other })));
            }
        }
    }
    audit::record(
        &mut tx,
        actor,
        AuditEntry {
            action: "property.restrictions_bulk",
            resource_type: "property",
            resource_id: &property.id,
            reason: None,
            change: json!({ "added": result.applied.len(), "removed": result.removed.len(), "affectedNights": result.affected_nights }),
        },
    )
    .await?;
    outbox::enqueue(
        &mut tx,
        property.city,
        "property",
        &property.id,
        "property.restrictions_changed",
        json!({ "propertyId": property.id, "added": result.applied.len(), "removed": result.removed.len() }),
    )
    .await?;
    if let Some(key) = key {
        let stored = serde_json::to_value(&result).map_err(|error| Error::internal(error.to_string()))?;
        idempotency::store_response(&mut tx, property.city, BULK_SCOPE, key, &stored).await?;
    }
    tx.commit().await?;
    Ok(result)
}

async fn add_one(
    tx: &mut Transaction<'_, Postgres>,
    property: &PropertyRef,
    actor: &AuditActor,
    operation: &RestrictionOperation,
) -> Result<RestrictionView> {
    let kind = operation
        .kind
        .as_deref()
        .ok_or_else(|| Error::bad_request(RESTRICTION_KIND_REQUIRED))?;
    validate_kind(kind)?;
    let basis = operation.basis.as_deref().unwrap_or("STAY_THROUGH");
    validate_basis(kind, basis)?;
    validate_value(kind, operation.value)?;
    validate_window(operation.start_date, operation.end_date)?;
    if let Some(unit_type_id) = operation.unit_type_id.as_deref() {
        if !queries::unit_type_in_property(tx, &property.id, unit_type_id).await? {
            return Err(Error::not_found(UNIT_TYPE_NOT_FOUND));
        }
    }
    let id = new_id("restr");
    queries::insert(
        tx,
        NewRestriction {
            id: &id,
            city: property.city,
            property_id: &property.id,
            unit_type_id: operation.unit_type_id.as_deref(),
            kind,
            basis,
            start_date: operation.start_date,
            end_date: operation.end_date,
            value: operation.value,
            actor: actor.user_id.as_str(),
        },
    )
    .await?;
    Ok(RestrictionView {
        id,
        property_id: property.id.clone(),
        unit_type_id: operation.unit_type_id.clone(),
        kind: kind.to_string(),
        basis: basis.to_string(),
        start_date: operation.start_date,
        end_date: operation.end_date,
        value: operation.value,
        version: 1,
        created_at: chrono::Utc::now(),
        removed_at: None,
    })
}

fn affected(operation: &RestrictionOperation) -> i64 {
    match (operation.start_date, operation.end_date) {
        (Some(start), Some(end)) => (end - start).num_days().max(0),
        _ => 0,
    }
}
