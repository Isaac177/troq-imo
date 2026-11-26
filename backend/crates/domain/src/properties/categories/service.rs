use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::error::{Error, Result};
use kernel::outbox;
use kernel::paging::{Page, PageRequest};
use kernel::permissions::names::PROPERTIES_CONFIGURE;
use serde_json::json;
use sqlx::PgPool;

use super::queries;
use super::types::{CategoryInput, CategoryListQuery, CategoryView};
use super::validation::{code_from_name, existing_code, validate};
use crate::properties::messages::{CATEGORY_CODE_EXHAUSTED, CATEGORY_NOT_FOUND, CATEGORY_UNKNOWN};

const MAX_CODE_ATTEMPTS: u32 = 99;

pub async fn catalog(pool: &PgPool) -> Result<Vec<CategoryView>> {
    queries::catalog(pool).await
}

pub async fn list(pool: &PgPool, actor: &Actor, query: &CategoryListQuery, page: PageRequest) -> Result<Page<CategoryView>> {
    actor.require_platform(PROPERTIES_CONFIGURE)?;
    queries::list(pool, query, page).await
}

pub async fn ensure_selectable(pool: &PgPool, code: &str, current: Option<&str>) -> Result<()> {
    let found = if current == Some(code) {
        queries::exists(pool, code).await?
    } else {
        queries::is_active(pool, code).await?
    };
    if found {
        Ok(())
    } else {
        Err(Error::bad_request(CATEGORY_UNKNOWN).with_params(json!({ "code": code })))
    }
}

pub async fn create(pool: &PgPool, actor: &Actor, input: CategoryInput) -> Result<CategoryView> {
    actor.ensure_writable()?;
    actor.require_platform(PROPERTIES_CONFIGURE)?;
    let valid = validate(&input)?;
    let base = code_from_name(&valid.name_en)?;
    let mut tx = pool.begin().await?;
    let sort_order = match input.sort_order {
        Some(value) => value,
        None => sqlx::query_scalar::<_, i32>(
            r#"SELECT COALESCE(MAX("sortOrder"), 0) + 10 FROM stay_accommodation_categories"#,
        )
        .fetch_one(&mut *tx)
        .await?,
    };
    let mut chosen = None;
    for number in 1..=MAX_CODE_ATTEMPTS {
        let candidate = if number == 1 { base.clone() } else { format!("{base}_{number}") };
        chosen = sqlx::query_scalar::<_, String>(
            r#"INSERT INTO stay_accommodation_categories (code, "nameEn", "nameFr", icon, "sortOrder", active)
               VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (code) DO NOTHING RETURNING code"#,
        )
        .bind(&candidate)
        .bind(&valid.name_en)
        .bind(&valid.name_fr)
        .bind(&valid.icon)
        .bind(sort_order)
        .bind(input.active.unwrap_or(true))
        .fetch_optional(&mut *tx)
        .await?;
        if chosen.is_some() {
            break;
        }
    }
    let code = chosen.ok_or_else(|| Error::conflict(CATEGORY_CODE_EXHAUSTED))?;
    record(&mut tx, actor, "accommodation_category.created", &code).await?;
    tx.commit().await?;
    queries::find(pool, &code).await?.ok_or_else(|| Error::internal("created category missing"))
}

pub async fn update(pool: &PgPool, actor: &Actor, code: &str, input: CategoryInput) -> Result<CategoryView> {
    actor.ensure_writable()?;
    actor.require_platform(PROPERTIES_CONFIGURE)?;
    let code = existing_code(code)?;
    let valid = validate(&input)?;
    let mut tx = pool.begin().await?;
    let updated = sqlx::query(
        r#"UPDATE stay_accommodation_categories SET "nameEn" = $2, "nameFr" = $3, icon = $4,
           "sortOrder" = COALESCE($5, "sortOrder"), active = COALESCE($6, active),
           revision = revision + 1, "updatedAt" = now() WHERE code = $1"#,
    )
    .bind(&code)
    .bind(&valid.name_en)
    .bind(&valid.name_fr)
    .bind(&valid.icon)
    .bind(input.sort_order)
    .bind(input.active)
    .execute(&mut *tx)
    .await?;
    if updated.rows_affected() != 1 {
        return Err(Error::not_found(CATEGORY_NOT_FOUND));
    }
    record(&mut tx, actor, "accommodation_category.updated", &code).await?;
    tx.commit().await?;
    queries::find(pool, &code).await?.ok_or_else(|| Error::internal("updated category missing"))
}

async fn record(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    actor: &Actor,
    action: &str,
    code: &str,
) -> Result<()> {
    audit::record(tx, &actor.audit(), AuditEntry {
        action,
        resource_type: "accommodation_category",
        resource_id: code,
        reason: None,
        change: json!({ "code": code }),
    })
    .await?;
    outbox::enqueue(tx, actor.city, "accommodation_category", code, action, json!({ "code": code })).await?;
    Ok(())
}
