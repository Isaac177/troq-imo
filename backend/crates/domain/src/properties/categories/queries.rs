use kernel::error::Result;
use kernel::paging::{Page, PageRequest};
use sqlx::PgPool;

use super::types::{CategoryListQuery, CategoryView};
use crate::properties::http::like_pattern;

const COLUMNS: &str = r#"c.code, c."nameEn" AS name_en, c."nameFr" AS name_fr, c.icon,
    c."sortOrder" AS sort_order, c.active, c.revision, c."updatedAt" AS updated_at,
    (SELECT COUNT(*) FROM stay_unit_types u WHERE u."categoryCode" = c.code) AS unit_type_count"#;

const FILTER: &str = r#"WHERE ($1::boolean IS NULL OR c.active = $1)
    AND ($2::text IS NULL OR c."nameEn" ILIKE $2 ESCAPE '\' OR c."nameFr" ILIKE $2 ESCAPE '\' OR c.code ILIKE $2 ESCAPE '\')"#;

pub async fn catalog(pool: &PgPool) -> Result<Vec<CategoryView>> {
    let sql = format!(r#"SELECT {COLUMNS} FROM stay_accommodation_categories c WHERE c.active ORDER BY c."sortOrder", c.code"#);
    Ok(sqlx::query_as::<_, CategoryView>(&sql).fetch_all(pool).await?)
}

pub async fn list(pool: &PgPool, query: &CategoryListQuery, page: PageRequest) -> Result<Page<CategoryView>> {
    let active = match query.status.as_deref() {
        Some("ACTIVE") => Some(true),
        Some("RETIRED") => Some(false),
        _ => None,
    };
    let search = like_pattern(query.search.as_deref());
    let total: i64 = sqlx::query_scalar(&format!("SELECT COUNT(*) FROM stay_accommodation_categories c {FILTER}"))
        .bind(active)
        .bind(&search)
        .fetch_one(pool)
        .await?;
    let sql = format!(
        r#"SELECT {COLUMNS} FROM stay_accommodation_categories c {FILTER} ORDER BY c."sortOrder", c.code LIMIT $3 OFFSET $4"#
    );
    let items = sqlx::query_as::<_, CategoryView>(&sql)
        .bind(active)
        .bind(&search)
        .bind(page.limit())
        .bind(page.offset())
        .fetch_all(pool)
        .await?;
    Ok(Page::new(items, total, page))
}

pub async fn find(pool: &PgPool, code: &str) -> Result<Option<CategoryView>> {
    let sql = format!("SELECT {COLUMNS} FROM stay_accommodation_categories c WHERE c.code = $1");
    Ok(sqlx::query_as::<_, CategoryView>(&sql).bind(code).fetch_optional(pool).await?)
}

pub async fn exists(pool: &PgPool, code: &str) -> Result<bool> {
    Ok(sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM stay_accommodation_categories WHERE code = $1)")
        .bind(code)
        .fetch_one(pool)
        .await?)
}

pub async fn is_active(pool: &PgPool, code: &str) -> Result<bool> {
    Ok(sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM stay_accommodation_categories WHERE code = $1 AND active)")
        .bind(code)
        .fetch_one(pool)
        .await?)
}

pub async fn active_codes(pool: &PgPool) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar("SELECT code FROM stay_accommodation_categories WHERE active ORDER BY code")
        .fetch_all(pool)
        .await?)
}
