use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::error::{Error, Result};
use kernel::ids::new_id;
use kernel::permissions::names::PROPERTIES_CONFIGURE;
use sqlx::PgPool;

use super::types::{GroupWrite, SavedCatalogItem};
use super::validation::{self, CODE_EXHAUSTED, GROUP_NOT_FOUND};

pub async fn create_group(pool: &PgPool, actor: &Actor, input: GroupWrite) -> Result<SavedCatalogItem> {
    actor.ensure_writable()?;
    actor.require_platform(PROPERTIES_CONFIGURE)?;
    let name = validation::name(&input.name)?;
    let locale = validation::locale(&input.locale)?;
    let base = validation::code_from_name(&name)?;
    let id = new_id("amenity-group");
    let mut tx = pool.begin().await?;
    let sort_order = match input.sort_order {
        Some(value) => value,
        None => sqlx::query_scalar::<_, i32>("SELECT COALESCE(MAX(sort_order), 0) + 10 FROM property_amenity_groups")
            .fetch_one(&mut *tx).await?,
    };
    let mut chosen = None;
    for number in 1..=99 {
        let candidate = if number == 1 { base.clone() } else { format!("{base}_{number}") };
        let inserted = sqlx::query_scalar::<_, String>(
            "INSERT INTO property_amenity_groups (id, code, source_locale, sort_order, active) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (code) DO NOTHING RETURNING code",
        ).bind(&id).bind(&candidate).bind(&locale).bind(sort_order).bind(input.active.unwrap_or(true))
            .fetch_optional(&mut *tx).await?;
        if inserted.is_some() { chosen = inserted; break; }
    }
    let code = chosen.ok_or_else(|| Error::conflict(CODE_EXHAUSTED))?;
    sqlx::query("INSERT INTO property_amenity_group_content (group_id, locale, name) VALUES ($1, $2, $3)")
        .bind(&id).bind(&locale).bind(&name).execute(&mut *tx).await?;
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "amenity_group.created", resource_type: "amenity_group", resource_id: &id,
        reason: None, change: serde_json::json!({ "code": code, "locale": locale }),
    }).await?;
    kernel::outbox::enqueue(
        &mut tx, actor.city, "amenity_group", &id, "amenity_group.created",
        serde_json::json!({ "code": code, "locale": locale }),
    ).await?;
    tx.commit().await?;
    Ok(SavedCatalogItem { code })
}

pub async fn update_group(pool: &PgPool, actor: &Actor, code: &str, input: GroupWrite) -> Result<SavedCatalogItem> {
    actor.ensure_writable()?;
    actor.require_platform(PROPERTIES_CONFIGURE)?;
    let code = validation::code(code)?;
    let name = validation::name(&input.name)?;
    let locale = validation::locale(&input.locale)?;
    let mut tx = pool.begin().await?;
    let id = sqlx::query_scalar::<_, String>(
        "UPDATE property_amenity_groups SET sort_order = COALESCE($2, sort_order), active = COALESCE($3, active), revision = revision + 1, updated_at = now() WHERE code = $1 RETURNING id",
    ).bind(&code).bind(input.sort_order).bind(input.active).fetch_optional(&mut *tx).await?
        .ok_or_else(|| Error::not_found(GROUP_NOT_FOUND))?;
    sqlx::query("INSERT INTO property_amenity_group_content (group_id, locale, name) VALUES ($1, $2, $3) ON CONFLICT (group_id, locale) DO UPDATE SET name = EXCLUDED.name")
        .bind(&id).bind(&locale).bind(&name).execute(&mut *tx).await?;
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "amenity_group.updated", resource_type: "amenity_group", resource_id: &id,
        reason: None, change: serde_json::json!({ "code": code, "locale": locale }),
    }).await?;
    kernel::outbox::enqueue(
        &mut tx, actor.city, "amenity_group", &id, "amenity_group.updated",
        serde_json::json!({ "code": code, "locale": locale }),
    ).await?;
    tx.commit().await?;
    Ok(SavedCatalogItem { code })
}
