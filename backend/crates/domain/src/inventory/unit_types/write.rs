use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::error::{Error, Result};
use kernel::ids::new_id;
use kernel::permissions::names::PROPERTIES_CONFIGURE;
use serde_json::json;
use sqlx::PgPool;

use super::idempotency::{claim, clean, remember, replay, replay_or_busy, CREATE_SCOPE, UPDATE_SCOPE};
use super::queries::{category_selectable, load, quantity_is_locked, template_host};
use super::types::{CreateUnitTypeInput, UnitTypeView, UpdateUnitTypeInput};
use super::validation::{validate_inventory_model, validate_unit_type, UnitTypeFields};
use crate::inventory::messages::{
    UNIT_TYPE_CATEGORY_UNKNOWN, UNIT_TYPE_QUANTITY_LOCKED, UNIT_TYPE_TEMPLATE_UNKNOWN,
    UNIT_TYPE_VERSION_CONFLICT,
};
use crate::inventory::pools::{create_pool, resize_pool};
use crate::inventory::scope::actor_property;

async fn ensure_category(pool: &PgPool, code: &str, current: Option<&str>) -> Result<()> {
    if category_selectable(pool, code, current).await? {
        Ok(())
    } else {
        Err(Error::bad_request(UNIT_TYPE_CATEGORY_UNKNOWN))
    }
}

async fn ensure_template(pool: &PgPool, template_id: Option<&str>, host_id: &str) -> Result<()> {
    let Some(template_id) = template_id else {
        return Ok(());
    };
    match template_host(pool, template_id).await? {
        Some(owner) if owner == host_id => Ok(()),
        _ => Err(Error::bad_request(UNIT_TYPE_TEMPLATE_UNKNOWN)),
    }
}

pub async fn create(
    pool: &PgPool,
    actor: &Actor,
    property_id: &str,
    input: CreateUnitTypeInput,
    as_admin: bool,
) -> Result<UnitTypeView> {
    let property = actor_property(pool, actor, property_id, PROPERTIES_CONFIGURE, as_admin).await?;
    let key = clean(input.idempotency_key.as_deref())?;
    if let Some(view) = replay::<UnitTypeView>(pool, property.city, CREATE_SCOPE, key).await? {
        return Ok(view);
    }
    validate_inventory_model(&input.inventory_model)?;
    ensure_category(pool, input.category_code.trim(), None).await?;
    ensure_template(pool, input.template_id.as_deref(), &property.host_id).await?;
    validate_unit_type(&UnitTypeFields {
        name_en: &input.name_en,
        name_fr: &input.name_fr,
        max_adults: input.max_adults,
        max_children: input.max_children,
        bedroom_count: input.bedroom_count,
        bed_count: input.bed_count,
        bathroom_count: input.bathroom_count,
        sellable_quantity: input.sellable_quantity,
        accessibility: &input.accessibility,
    })?;
    let unit_type_id = new_id("unit-type");
    let mut tx = pool.begin().await?;
    if !claim(&mut tx, property.city, CREATE_SCOPE, key, &unit_type_id).await? {
        tx.rollback().await?;
        return replay_or_busy(pool, property.city, CREATE_SCOPE, key).await;
    }
    sqlx::query(
        r#"INSERT INTO stay_unit_types
           (id, city_id, "propertyId", "inventoryModel", "categoryCode", "nameEn", "nameFr", "descriptionEn", "descriptionFr",
            "maxAdults", "maxChildren", "bedroomCount", "bedCount", "bathroomCount", accessibility, "templateId")
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)"#,
    )
    .bind(&unit_type_id)
    .bind(property.city.code())
    .bind(&property.id)
    .bind(&input.inventory_model)
    .bind(input.category_code.trim())
    .bind(input.name_en.trim())
    .bind(input.name_fr.trim())
    .bind(input.description_en.trim())
    .bind(input.description_fr.trim())
    .bind(input.max_adults)
    .bind(input.max_children)
    .bind(input.bedroom_count)
    .bind(input.bed_count)
    .bind(input.bathroom_count)
    .bind(json!(input.accessibility))
    .bind(input.template_id.as_deref())
    .execute(&mut *tx)
    .await?;
    create_pool(&mut tx, property.city, &property.id, &unit_type_id, input.sellable_quantity).await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "unit_type.create",
            resource_type: "unit_type",
            resource_id: &unit_type_id,
            reason: None,
            change: json!({ "inventoryModel": input.inventory_model, "sellableQuantity": input.sellable_quantity }),
        },
    )
    .await?;
    let view = load(&mut tx, &property.id, &unit_type_id).await?;
    remember(&mut tx, property.city, CREATE_SCOPE, key, &view).await?;
    tx.commit().await?;
    Ok(view)
}

pub async fn update(
    pool: &PgPool,
    actor: &Actor,
    property_id: &str,
    unit_type_id: &str,
    input: UpdateUnitTypeInput,
    as_admin: bool,
) -> Result<UnitTypeView> {
    let property = actor_property(pool, actor, property_id, PROPERTIES_CONFIGURE, as_admin).await?;
    let key = clean(input.idempotency_key.as_deref())?;
    if let Some(view) = replay::<UnitTypeView>(pool, property.city, UPDATE_SCOPE, key).await? {
        return Ok(view);
    }
    validate_unit_type(&UnitTypeFields {
        name_en: &input.name_en,
        name_fr: &input.name_fr,
        max_adults: input.max_adults,
        max_children: input.max_children,
        bedroom_count: input.bedroom_count,
        bed_count: input.bed_count,
        bathroom_count: input.bathroom_count,
        sellable_quantity: input.sellable_quantity,
        accessibility: &input.accessibility,
    })?;
    let mut tx = pool.begin().await?;
    let current = load(&mut tx, &property.id, unit_type_id).await?;
    if current.version != input.expected_version {
        return Err(Error::conflict(UNIT_TYPE_VERSION_CONFLICT));
    }
    if !claim(&mut tx, property.city, UPDATE_SCOPE, key, &current.id).await? {
        tx.rollback().await?;
        return replay_or_busy(pool, property.city, UPDATE_SCOPE, key).await;
    }
    ensure_category(pool, input.category_code.trim(), Some(current.category_code.as_str())).await?;
    if current.sellable_quantity != input.sellable_quantity {
        if quantity_is_locked(&mut tx, &current.id).await? {
            return Err(Error::conflict(UNIT_TYPE_QUANTITY_LOCKED));
        }
        resize_pool(
            &mut tx,
            property.city,
            &property.id,
            &current.id,
            current.sellable_quantity,
            input.sellable_quantity,
        )
        .await?;
    }
    let updated = sqlx::query(
        r#"UPDATE stay_unit_types SET "nameEn" = $1, "nameFr" = $2, "descriptionEn" = $3,
           "descriptionFr" = $4, "maxAdults" = $5, "maxChildren" = $6, "bedroomCount" = $7,
           "bedCount" = $8, "bathroomCount" = $9, accessibility = $10, "categoryCode" = $11,
           version = version + 1, "updatedAt" = now()
           WHERE id = $12 AND "propertyId" = $13 AND version = $14"#,
    )
    .bind(input.name_en.trim())
    .bind(input.name_fr.trim())
    .bind(input.description_en.trim())
    .bind(input.description_fr.trim())
    .bind(input.max_adults)
    .bind(input.max_children)
    .bind(input.bedroom_count)
    .bind(input.bed_count)
    .bind(input.bathroom_count)
    .bind(json!(input.accessibility))
    .bind(input.category_code.trim())
    .bind(&current.id)
    .bind(&property.id)
    .bind(input.expected_version)
    .execute(&mut *tx)
    .await?;
    if updated.rows_affected() != 1 {
        return Err(Error::conflict(UNIT_TYPE_VERSION_CONFLICT));
    }
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "unit_type.update",
            resource_type: "unit_type",
            resource_id: &current.id,
            reason: None,
            change: json!({ "sellableQuantity": input.sellable_quantity, "maxAdults": input.max_adults, "maxChildren": input.max_children }),
        },
    )
    .await?;
    let view = load(&mut tx, &property.id, &current.id).await?;
    remember(&mut tx, property.city, UPDATE_SCOPE, key, &view).await?;
    tx.commit().await?;
    Ok(view)
}
