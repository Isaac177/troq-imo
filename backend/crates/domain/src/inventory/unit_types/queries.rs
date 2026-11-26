use kernel::error::{Error, Result};
use kernel::storage::Storage;
use sqlx::{PgPool, Postgres, Transaction};

use super::types::{ImageView, UnitTypeView};
use crate::inventory::messages::UNIT_TYPE_NOT_FOUND;
use crate::properties::media::image_url;

const VIEW_COLUMNS: &str = r#"u.id, u."inventoryModel" AS inventory_model, u."categoryCode" AS category_code,
    u."templateId" AS template_id, u."nameEn" AS name_en, u."nameFr" AS name_fr,
    u."descriptionEn" AS description_en, u."descriptionFr" AS description_fr,
    u."maxAdults" AS max_adults, u."maxChildren" AS max_children,
    u."bedroomCount" AS bedroom_count, u."bedCount" AS bed_count,
    u."bathroomCount" AS bathroom_count, u.accessibility,
    COALESCE(p."sellableQuantity", 0) AS sellable_quantity,
    (SELECT MIN(rr.amount)::text FROM stay_rate_plans rp JOIN stay_rate_rules rr ON rr."ratePlanId" = rp.id
     WHERE rp."unitTypeId" = u.id AND rp.status = 'ACTIVE' AND rr."adjustmentType" = 'OVERRIDE'
       AND rr."startDate" IS NULL) AS base_nightly_amount,
    u.status, u.version"#;

pub async fn load(
    tx: &mut Transaction<'_, Postgres>,
    property_id: &str,
    id: &str,
) -> Result<UnitTypeView> {
    sqlx::query_as::<_, UnitTypeView>(&format!(
        r#"SELECT {VIEW_COLUMNS}
           FROM stay_unit_types u JOIN stay_inventory_pools p ON p."unitTypeId" = u.id
           WHERE u."propertyId" = $1 AND u.id = $2"#
    ))
    .bind(property_id)
    .bind(id)
    .fetch_optional(&mut **tx)
    .await?
    .ok_or_else(|| Error::not_found(UNIT_TYPE_NOT_FOUND))
}

pub async fn count_active(pool: &PgPool, property_id: &str) -> Result<i64> {
    let total = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM stay_unit_types WHERE "propertyId" = $1 AND status <> 'ARCHIVED'"#,
    )
    .bind(property_id)
    .fetch_one(pool)
    .await?;
    Ok(total)
}

pub async fn list_active(
    pool: &PgPool,
    property_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<UnitTypeView>> {
    let rows = sqlx::query_as::<_, UnitTypeView>(&format!(
        r#"SELECT {VIEW_COLUMNS}
           FROM stay_unit_types u LEFT JOIN stay_inventory_pools p ON p."unitTypeId" = u.id
           WHERE u."propertyId" = $1 AND u.status <> 'ARCHIVED'
           ORDER BY u."createdAt", u.id LIMIT $2 OFFSET $3"#
    ))
    .bind(property_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn attach_images(pool: &PgPool, storage: &Storage, items: &mut [UnitTypeView]) -> Result<()> {
    let ids: Vec<String> = items
        .iter()
        .filter(|item| item.template_id.is_some())
        .map(|item| item.id.clone())
        .collect();
    if ids.is_empty() {
        return Ok(());
    }
    let rows: Vec<(String, String, String, String, bool, String)> = sqlx::query_as(
        r#"SELECT u.id, m.id, m."altEn", m."altFr", m."isCover", m."storageKey"
           FROM stay_unit_types u
           JOIN stay_template_media m ON m."templateId" = u."templateId"
             AND m.status = 'READY' AND m."storageKey" IS NOT NULL
           WHERE u.id = ANY($1)
           ORDER BY u.id, m."isCover" DESC, m."sortOrder", m."createdAt", m.id"#,
    )
    .bind(&ids)
    .fetch_all(pool)
    .await?;
    for (unit_type_id, media_id, alt_en, alt_fr, is_cover, key) in rows {
        let Some(item) = items.iter_mut().find(|item| item.id == unit_type_id) else {
            continue;
        };
        match image_url(storage, &key).await {
            Ok(url) => item.images.push(ImageView { id: media_id, url, alt_en, alt_fr, is_cover }),
            Err(error) => tracing::warn!(media_id = %media_id, error = %error, "template media unavailable"),
        }
    }
    for item in items.iter_mut() {
        item.cover_url = item.images.first().map(|image| image.url.clone());
    }
    Ok(())
}

pub async fn category_selectable(pool: &PgPool, code: &str, current: Option<&str>) -> Result<bool> {
    let sql = if current == Some(code) {
        "SELECT EXISTS (SELECT 1 FROM stay_accommodation_categories WHERE code = $1)"
    } else {
        "SELECT EXISTS (SELECT 1 FROM stay_accommodation_categories WHERE code = $1 AND active)"
    };
    let found = sqlx::query_scalar(sql).bind(code).fetch_one(pool).await?;
    Ok(found)
}

pub async fn template_host(pool: &PgPool, template_id: &str) -> Result<Option<String>> {
    let host = sqlx::query_scalar::<_, String>(
        r#"SELECT "operatorId" FROM stay_accommodation_templates WHERE id = $1"#,
    )
    .bind(template_id)
    .fetch_optional(pool)
    .await?;
    Ok(host)
}

pub async fn quantity_is_locked(tx: &mut Transaction<'_, Postgres>, unit_type_id: &str) -> Result<bool> {
    let used = sqlx::query_scalar(
        r#"SELECT EXISTS (
               SELECT 1 FROM stay_night_inventory n JOIN stay_inventory_pools p ON p.id = n."poolId"
               WHERE p."unitTypeId" = $1
           ) OR EXISTS (
               SELECT 1 FROM stay_inventory_allocations a JOIN stay_inventory_pools p ON p.id = a."poolId"
               WHERE p."unitTypeId" = $1
           ) OR EXISTS (
               SELECT 1 FROM stay_reservation_items WHERE "unitTypeId" = $1
           )"#,
    )
    .bind(unit_type_id)
    .fetch_one(&mut **tx)
    .await?;
    Ok(used)
}

pub async fn has_reservations(tx: &mut Transaction<'_, Postgres>, unit_type_id: &str) -> Result<bool> {
    let reserved = sqlx::query_scalar(
        r#"SELECT EXISTS (SELECT 1 FROM stay_reservation_items WHERE "unitTypeId" = $1)"#,
    )
    .bind(unit_type_id)
    .fetch_one(&mut **tx)
    .await?;
    Ok(reserved)
}
