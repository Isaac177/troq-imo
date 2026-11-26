use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::error::{Error, Result};
use kernel::ids::new_id;
use kernel::permissions::names::PROPERTIES_CONFIGURE;
use sqlx::PgPool;

use super::types::{AmenityWrite, SavedCatalogItem};
use super::validation::{self, AMENITY_NOT_FOUND, CODE_EXHAUSTED, GROUP_NOT_FOUND};

pub async fn create_amenity(pool: &PgPool, actor: &Actor, input: AmenityWrite) -> Result<SavedCatalogItem> {
    actor.ensure_writable()?;
    actor.require_platform(PROPERTIES_CONFIGURE)?;
    let name_en = validation::name(&input.name_en)?;
    let name_fr = validation::name(&input.name_fr)?;
    let icon = validation::icon(&input.icon)?;
    let group_code = validation::code(&input.group_code)?;
    let base = validation::code_from_name(&name_en)?;
    let id = new_id("amenity");
    let mut tx = pool.begin().await?;
    let group_id = sqlx::query_scalar::<_, String>("SELECT id FROM property_amenity_groups WHERE code = $1")
        .bind(&group_code).fetch_optional(&mut *tx).await?
        .ok_or_else(|| Error::bad_request(GROUP_NOT_FOUND))?;
    let sort_order = match input.sort_order {
        Some(value) => value,
        None => sqlx::query_scalar::<_, i32>("SELECT COALESCE(MAX(sort_order), 0) + 10 FROM property_amenities WHERE group_id = $1")
            .bind(&group_id).fetch_one(&mut *tx).await?,
    };
    let mut chosen = None;
    for number in 1..=99 {
        let candidate = if number == 1 { base.clone() } else { format!("{base}_{number}") };
        let inserted = sqlx::query_scalar::<_, String>(
            r#"INSERT INTO property_amenities (id, code, group_id, source_locale, icon, sort_order, active, "nameEn", "nameFr") VALUES ($1, $2, $3, 'en', $4, $5, $6, $7, $8) ON CONFLICT (code) DO NOTHING RETURNING code"#,
        ).bind(&id).bind(&candidate).bind(&group_id).bind(&icon).bind(sort_order)
            .bind(input.active.unwrap_or(true)).bind(&name_en).bind(&name_fr).fetch_optional(&mut *tx).await?;
        if inserted.is_some() { chosen = inserted; break; }
    }
    let code = chosen.ok_or_else(|| Error::conflict(CODE_EXHAUSTED))?;
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "amenity.created", resource_type: "amenity", resource_id: &id,
        reason: None, change: serde_json::json!({ "code": code, "groupCode": group_code }),
    }).await?;
    kernel::outbox::enqueue(
        &mut tx, actor.city, "amenity", &id, "amenity.created",
        serde_json::json!({ "code": code, "groupCode": group_code }),
    ).await?;
    tx.commit().await?;
    Ok(SavedCatalogItem { code })
}

pub async fn update_amenity(pool: &PgPool, actor: &Actor, code: &str, input: AmenityWrite) -> Result<SavedCatalogItem> {
    actor.ensure_writable()?;
    actor.require_platform(PROPERTIES_CONFIGURE)?;
    let code = validation::code(code)?;
    let name_en = validation::name(&input.name_en)?;
    let name_fr = validation::name(&input.name_fr)?;
    let icon = validation::icon(&input.icon)?;
    let group_code = validation::code(&input.group_code)?;
    let mut tx = pool.begin().await?;
    let group_id = sqlx::query_scalar::<_, String>("SELECT id FROM property_amenity_groups WHERE code = $1")
        .bind(&group_code).fetch_optional(&mut *tx).await?
        .ok_or_else(|| Error::bad_request(GROUP_NOT_FOUND))?;
    let id = sqlx::query_scalar::<_, String>(
        r#"UPDATE property_amenities SET group_id = $2, icon = $3, sort_order = COALESCE($4, sort_order), active = COALESCE($5, active), "nameEn" = $6, "nameFr" = $7, revision = revision + 1, updated_at = now() WHERE code = $1 RETURNING id"#,
    ).bind(&code).bind(&group_id).bind(&icon).bind(input.sort_order).bind(input.active).bind(&name_en).bind(&name_fr)
        .fetch_optional(&mut *tx).await?.ok_or_else(|| Error::not_found(AMENITY_NOT_FOUND))?;
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "amenity.updated", resource_type: "amenity", resource_id: &id,
        reason: None, change: serde_json::json!({ "code": code, "groupCode": group_code }),
    }).await?;
    kernel::outbox::enqueue(
        &mut tx, actor.city, "amenity", &id, "amenity.updated",
        serde_json::json!({ "code": code, "groupCode": group_code }),
    ).await?;
    tx.commit().await?;
    Ok(SavedCatalogItem { code })
}
