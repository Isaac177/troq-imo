use kernel::error::Result;
use kernel::paging::{Page, PageRequest};
use sqlx::PgPool;

use super::types::{Amenity, AmenityFilters, AmenityGroup, GroupFilters};

const GROUP_FROM: &str = "FROM property_amenity_groups g";
const GROUP_FILTER: &str = r#"
    WHERE ($1::boolean IS NULL OR g.active = $1)
      AND ($2::text IS NULL OR g."nameEn" ILIKE $2 ESCAPE '\' OR g."nameFr" ILIKE $2 ESCAPE '\')
"#;

const AMENITY_FROM: &str = "FROM property_amenities a JOIN property_amenity_groups g ON g.id = a.group_id";
const AMENITY_FILTER: &str = r#"
    WHERE ($1::boolean IS NULL OR a.active = $1)
      AND ($2::text IS NULL OR g.code = $2)
      AND ($3::text IS NULL OR a.code ILIKE $3 ESCAPE '\' OR a."nameEn" ILIKE $3 ESCAPE '\' OR a."nameFr" ILIKE $3 ESCAPE '\')
"#;

fn search_pattern(value: Option<&str>) -> Option<String> {
    value.map(str::trim).filter(|s| !s.is_empty()).map(|s| {
        format!("%{}%", s.replace('\\', "\\\\").replace('%', "\\%").replace('_', "\\_"))
    })
}

pub async fn list_groups(
    pool: &PgPool,
    locale: &str,
    filters: &GroupFilters,
    page: PageRequest,
) -> Result<Page<AmenityGroup>> {
    let page = page.validated()?;
    let search = search_pattern(filters.search.as_deref());
    let total = sqlx::query_scalar::<_, i64>(&format!("SELECT COUNT(*) {GROUP_FROM} {GROUP_FILTER}"))
        .bind(filters.active).bind(&search).fetch_one(pool).await?;
    let query = format!(r#"
        SELECT g.code, CASE WHEN lower($3) LIKE 'fr%' THEN COALESCE(NULLIF(g."nameFr", ''), g."nameEn") ELSE COALESCE(NULLIF(g."nameEn", ''), g."nameFr") END AS name,
               g.sort_order, g.active, g.revision, g.updated_at,
               (SELECT COUNT(*) FROM property_amenities a WHERE a.group_id = g.id) AS amenity_count
        {GROUP_FROM}
        {GROUP_FILTER}
        ORDER BY g.sort_order, g.code LIMIT $4 OFFSET $5
    "#);
    let items = sqlx::query_as::<_, AmenityGroup>(&query)
        .bind(filters.active).bind(&search).bind(locale).bind(page.limit()).bind(page.offset())
        .fetch_all(pool).await?;
    Ok(Page::new(items, total, page))
}

pub async fn list_amenities(
    pool: &PgPool,
    locale: &str,
    filters: &AmenityFilters,
    page: PageRequest,
) -> Result<Page<Amenity>> {
    let page = page.validated()?;
    let search = search_pattern(filters.search.as_deref());
    let group = filters.group_code.as_deref().map(str::trim).filter(|s| !s.is_empty());
    let total = sqlx::query_scalar::<_, i64>(&format!("SELECT COUNT(*) {AMENITY_FROM} {AMENITY_FILTER}"))
        .bind(filters.active).bind(group).bind(&search).fetch_one(pool).await?;
    let query = format!(r#"
        SELECT a.code, g.code AS group_code, CASE WHEN lower($4) LIKE 'fr%' THEN COALESCE(NULLIF(a."nameFr", ''), a."nameEn") ELSE COALESCE(NULLIF(a."nameEn", ''), a."nameFr") END AS name,
               a.icon, a.sort_order, a.active, a.revision, a.updated_at
        {AMENITY_FROM}
        {AMENITY_FILTER}
        ORDER BY g.code, a.sort_order, a.code LIMIT $5 OFFSET $6
    "#);
    let items = sqlx::query_as::<_, Amenity>(&query)
        .bind(filters.active).bind(group).bind(&search).bind(locale).bind(page.limit()).bind(page.offset())
        .fetch_all(pool).await?;
    Ok(Page::new(items, total, page))
}

#[cfg(test)]
mod tests {
    use super::search_pattern;

    #[test]
    fn search_treats_wildcards_as_text() {
        assert_eq!(search_pattern(Some(" a_%\\ ")), Some("%a\\_\\%\\\\%".to_string()));
        assert_eq!(search_pattern(Some("   ")), None);
    }
}
