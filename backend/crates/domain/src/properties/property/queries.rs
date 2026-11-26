use kernel::city::City;
use kernel::error::Result;
use kernel::paging::{Page, PageRequest};
use sqlx::{PgConnection, PgPool, Postgres, QueryBuilder, Transaction};

use super::types::{PropertyListQuery, PropertyView};
use super::validation::ValidatedProperty;
use crate::properties::facts::property_facts_sql;
use crate::properties::http::like_pattern;

const BASE_COLUMNS: &str = r#"p.id, p."operatorId" AS operator_id, p.city_id AS city, p.timezone, p.status,
    p."publishedRevision" AS published_revision, p."nameEn" AS name_en, p."nameFr" AS name_fr,
    p."descriptionEn" AS description_en, p."descriptionFr" AS description_fr, p."addressLine" AS address_line,
    p.latitude::float8 AS latitude, p.longitude::float8 AS longitude, p.neighborhood,
    p."checkInFrom" AS check_in_from, p."checkInUntil" AS check_in_until, p."checkOutBy" AS check_out_by,
    p."houseRulesEn" AS house_rules_en, p."houseRulesFr" AS house_rules_fr,
    p."petsPolicy" AS pets_policy, p."smokingPolicy" AS smoking_policy,
    p."guestContactPhone" AS guest_contact_phone, p."guestContactEmail" AS guest_contact_email,
    COALESCE(NULLIF(p."guestContactPhone", ''), (SELECT h.guest_contact_phone FROM hosts h WHERE h.id = p."operatorId"), '') AS effective_guest_contact_phone,
    COALESCE(NULLIF(p."guestContactEmail", ''), (SELECT h.guest_contact_email FROM hosts h WHERE h.id = p."operatorId"), '') AS effective_guest_contact_email,
    p.amenities, p."verificationStatus" AS verification_status, p.version,
    p."createdAt" AS created_at, p."updatedAt" AS updated_at"#;

pub fn columns() -> String {
    format!("{BASE_COLUMNS}, {}", property_facts_sql("p"))
}

pub struct ListScope {
    pub member_hosts: Vec<String>,
    pub full_hosts: Vec<String>,
    pub property_ids: Vec<String>,
}

pub async fn find(pool: &PgPool, property_id: &str) -> Result<Option<PropertyView>> {
    let sql = format!("SELECT {} FROM stay_properties p WHERE p.id = $1", columns());
    Ok(sqlx::query_as::<_, PropertyView>(&sql).bind(property_id).fetch_optional(pool).await?)
}

pub async fn lock(conn: &mut PgConnection, property_id: &str) -> Result<Option<PropertyView>> {
    let sql = format!("SELECT {} FROM stay_properties p WHERE p.id = $1 FOR UPDATE OF p", columns());
    Ok(sqlx::query_as::<_, PropertyView>(&sql).bind(property_id).fetch_optional(&mut *conn).await?)
}

pub async fn list(
    pool: &PgPool,
    city: City,
    query: &PropertyListQuery,
    scope: &ListScope,
    page: PageRequest,
) -> Result<Page<PropertyView>> {
    let search = like_pattern(query.search.as_deref());
    let mut total = QueryBuilder::<Postgres>::new("SELECT COUNT(*) FROM stay_properties p WHERE TRUE");
    push_filters(&mut total, city, query, search.as_deref(), scope);
    let total: i64 = total.build_query_scalar().fetch_one(pool).await?;
    let mut items = QueryBuilder::<Postgres>::new(format!("SELECT {} FROM stay_properties p WHERE TRUE", columns()));
    push_filters(&mut items, city, query, search.as_deref(), scope);
    items
        .push(r#" ORDER BY p."nameEn", p.id LIMIT "#)
        .push_bind(page.limit())
        .push(" OFFSET ")
        .push_bind(page.offset());
    let rows = items.build_query_as::<PropertyView>().fetch_all(pool).await?;
    Ok(Page::new(rows, total, page))
}

fn push_filters(
    builder: &mut QueryBuilder<'_, Postgres>,
    city: City,
    query: &PropertyListQuery,
    search: Option<&str>,
    scope: &ListScope,
) {
    builder.push(" AND p.city_id = ").push_bind(city.code().to_string());
    builder
        .push(r#" AND p."operatorId" = ANY("#)
        .push_bind(scope.member_hosts.clone())
        .push(r#") AND (p."operatorId" = ANY("#)
        .push_bind(scope.full_hosts.clone())
        .push(") OR p.id = ANY(")
        .push_bind(scope.property_ids.clone())
        .push("))");
    if let Some(status) = query.status.as_deref() {
        builder.push(" AND p.status = ").push_bind(status.to_string());
    }
    if let Some(host_id) = query.host_id.as_deref() {
        builder.push(r#" AND p."operatorId" = "#).push_bind(host_id.to_string());
    }
    if let Some(search) = search {
        builder
            .push(r#" AND (p."nameEn" ILIKE "#)
            .push_bind(search.to_string())
            .push(r#" ESCAPE '\' OR p."nameFr" ILIKE "#)
            .push_bind(search.to_string())
            .push(r#" ESCAPE '\')"#);
    }
}

pub async fn host_status(pool: &PgPool, host_id: &str, city: City) -> Result<Option<String>> {
    Ok(sqlx::query_scalar("SELECT status FROM hosts WHERE id = $1 AND city_id = $2")
        .bind(host_id)
        .bind(city.code())
        .fetch_optional(pool)
        .await?)
}

pub async fn insert(
    tx: &mut Transaction<'_, Postgres>,
    property_id: &str,
    city: City,
    input: &ValidatedProperty,
) -> Result<()> {
    let (latitude, longitude) = match input.coordinates {
        Some((lat, lng)) => (Some(lat), Some(lng)),
        None => (None, None),
    };
    sqlx::query(
        r#"INSERT INTO stay_properties
           (id, city_id, "operatorId", timezone, status, "nameEn", "nameFr", "descriptionEn", "descriptionFr",
            "addressLine", latitude, longitude)
           VALUES ($1, $2, $3, $4, 'DRAFT', $5, $6, $7, $8, $9, $10::float8::numeric, $11::float8::numeric)"#,
    )
    .bind(property_id)
    .bind(city.code())
    .bind(&input.host_id)
    .bind(&input.timezone)
    .bind(&input.name_en)
    .bind(&input.name_fr)
    .bind(&input.description_en)
    .bind(&input.description_fr)
    .bind(&input.address_line)
    .bind(latitude)
    .bind(longitude)
    .execute(&mut **tx)
    .await?;
    Ok(())
}
