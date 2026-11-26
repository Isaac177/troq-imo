use kernel::error::Result;
use kernel::paging::{Page, PageRequest};
use sqlx::PgPool;

use super::queries::list_groups;
use super::types::{Amenity, AmenityGroup, GroupFilters};

pub async fn list_active_groups(pool: &PgPool, locale: &str, page: PageRequest) -> Result<Page<AmenityGroup>> {
    list_groups(pool, locale, &GroupFilters { active: Some(true), search: None }, page).await
}

pub async fn list_active_amenities(
    pool: &PgPool,
    locale: &str,
    group_code: Option<&str>,
    page: PageRequest,
) -> Result<Page<Amenity>> {
    let page = page.validated()?;
    let group = group_code.map(str::trim).filter(|code| !code.is_empty());
    let total = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM property_amenities a JOIN property_amenity_groups g ON g.id = a.group_id WHERE a.active AND g.active AND ($1::text IS NULL OR g.code = $1)",
    ).bind(group).fetch_one(pool).await?;
    let items = sqlx::query_as::<_, Amenity>(r#"
        SELECT a.code, g.code AS group_code, CASE WHEN lower($2) LIKE 'fr%' THEN COALESCE(NULLIF(a."nameFr", ''), a."nameEn") ELSE COALESCE(NULLIF(a."nameEn", ''), a."nameFr") END AS name,
               a.icon, a.sort_order, a.active, a.revision, a.updated_at
        FROM property_amenities a JOIN property_amenity_groups g ON g.id = a.group_id
        WHERE a.active AND g.active AND ($1::text IS NULL OR g.code = $1)
        ORDER BY g.sort_order, g.code, a.sort_order, a.code LIMIT $3 OFFSET $4
    "#).bind(group).bind(locale).bind(page.limit()).bind(page.offset()).fetch_all(pool).await?;
    Ok(Page::new(items, total, page))
}
