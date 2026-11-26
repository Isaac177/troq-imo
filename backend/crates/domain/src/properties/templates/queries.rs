use kernel::city::City;
use kernel::error::Result;
use kernel::money::Money;
use kernel::paging::{Page, PageRequest};
use sqlx::{PgPool, Postgres, QueryBuilder, Transaction};

use super::types::{TemplateInput, TemplateListQuery, TemplateView};
use crate::properties::http::like_pattern;

const COLUMNS: &str = r#"t.id, t."operatorId" AS operator_id, t.city_id, t."categoryCode" AS category_code,
    t."inventoryModel" AS inventory_model, t."nameEn" AS name_en, t."nameFr" AS name_fr,
    t."descriptionEn" AS description_en, t."descriptionFr" AS description_fr,
    t."maxAdults" AS max_adults, t."maxChildren" AS max_children, t."bedroomCount" AS bedroom_count,
    t."bedCount" AS bed_count, t."bathroomCount" AS bathroom_count, t.accessibility,
    t."baseNightlyAmount"::text AS base_nightly_amount, t.status, t.version, t."updatedAt" AS updated_at,
    (SELECT COUNT(*) FROM stay_unit_types u WHERE u."templateId" = t.id) AS unit_type_count,
    (SELECT COUNT(*) FROM stay_template_media m WHERE m."templateId" = t.id AND m.status = 'READY') AS image_count"#;

fn push_filters(builder: &mut QueryBuilder<'_, Postgres>, city: City, query: &TemplateListQuery, search: Option<&str>) {
    builder.push(" AND t.city_id = ").push_bind(city.code().to_string());
    if let Some(host_id) = query.host_id.as_deref().map(str::trim).filter(|value| !value.is_empty()) {
        builder.push(r#" AND t."operatorId" = "#).push_bind(host_id.to_string());
    }
    if let Some(status) = query.status.as_deref().filter(|value| ["ACTIVE", "ARCHIVED"].contains(value)) {
        builder.push(" AND t.status = ").push_bind(status.to_string());
    }
    if let Some(category) = query.category_code.as_deref().map(str::trim).filter(|value| !value.is_empty()) {
        builder.push(r#" AND t."categoryCode" = "#).push_bind(category.to_string());
    }
    if let Some(search) = search {
        builder
            .push(r#" AND (t."nameEn" ILIKE "#)
            .push_bind(search.to_string())
            .push(r#" ESCAPE '\' OR t."nameFr" ILIKE "#)
            .push_bind(search.to_string())
            .push(r#" ESCAPE '\')"#);
    }
}

pub async fn list(pool: &PgPool, city: City, query: &TemplateListQuery, page: PageRequest) -> Result<Page<TemplateView>> {
    let search = like_pattern(query.search.as_deref());
    let mut total = QueryBuilder::<Postgres>::new("SELECT COUNT(*) FROM stay_accommodation_templates t WHERE TRUE");
    push_filters(&mut total, city, query, search.as_deref());
    let total: i64 = total.build_query_scalar().fetch_one(pool).await?;
    let mut items = QueryBuilder::<Postgres>::new(format!(
        "SELECT {COLUMNS} FROM stay_accommodation_templates t WHERE TRUE"
    ));
    push_filters(&mut items, city, query, search.as_deref());
    items
        .push(r#" ORDER BY t."createdAt", t.id LIMIT "#)
        .push_bind(page.limit())
        .push(" OFFSET ")
        .push_bind(page.offset());
    let rows = items.build_query_as::<TemplateView>().fetch_all(pool).await?;
    Ok(Page::new(rows, total, page))
}

pub async fn find(pool: &PgPool, id: &str) -> Result<Option<TemplateView>> {
    let sql = format!("SELECT {COLUMNS} FROM stay_accommodation_templates t WHERE t.id = $1");
    Ok(sqlx::query_as::<_, TemplateView>(&sql).bind(id).fetch_optional(pool).await?)
}

pub async fn insert(
    tx: &mut Transaction<'_, Postgres>,
    id: &str,
    city: City,
    input: &TemplateInput,
    amount: Money,
) -> Result<()> {
    sqlx::query(
        r#"INSERT INTO stay_accommodation_templates
           (id, city_id, "operatorId", "categoryCode", "inventoryModel", "nameEn", "nameFr", "descriptionEn",
            "descriptionFr", "maxAdults", "maxChildren", "bedroomCount", "bedCount", "bathroomCount",
            accessibility, "baseNightlyAmount")
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16::text::numeric)"#,
    )
    .bind(id)
    .bind(city.code())
    .bind(input.host_id.trim())
    .bind(input.category_code.trim())
    .bind(&input.inventory_model)
    .bind(input.name_en.trim())
    .bind(input.name_fr.trim())
    .bind(input.description_en.trim())
    .bind(input.description_fr.trim())
    .bind(input.max_adults)
    .bind(input.max_children)
    .bind(input.bedroom_count)
    .bind(input.bed_count)
    .bind(input.bathroom_count)
    .bind(serde_json::json!(input.accessibility))
    .bind(amount.to_wire())
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn update_if_version(
    tx: &mut Transaction<'_, Postgres>,
    id: &str,
    expected_version: i32,
    input: &TemplateInput,
    status: &str,
    amount: Money,
) -> Result<bool> {
    let result = sqlx::query(
        r#"UPDATE stay_accommodation_templates SET "categoryCode" = $1, "inventoryModel" = $2,
           "nameEn" = $3, "nameFr" = $4, "descriptionEn" = $5, "descriptionFr" = $6, "maxAdults" = $7,
           "maxChildren" = $8, "bedroomCount" = $9, "bedCount" = $10, "bathroomCount" = $11,
           accessibility = $12, "baseNightlyAmount" = $13::text::numeric, status = $14,
           version = version + 1, "updatedAt" = now() WHERE id = $15 AND version = $16"#,
    )
    .bind(input.category_code.trim())
    .bind(&input.inventory_model)
    .bind(input.name_en.trim())
    .bind(input.name_fr.trim())
    .bind(input.description_en.trim())
    .bind(input.description_fr.trim())
    .bind(input.max_adults)
    .bind(input.max_children)
    .bind(input.bedroom_count)
    .bind(input.bed_count)
    .bind(input.bathroom_count)
    .bind(serde_json::json!(input.accessibility))
    .bind(amount.to_wire())
    .bind(status)
    .bind(id)
    .bind(expected_version)
    .execute(&mut **tx)
    .await?;
    Ok(result.rows_affected() == 1)
}
