use kernel::city::City;
use kernel::error::Result;
use serde_json::Value;
use sqlx::{FromRow, PgPool};

use crate::guests::listing::{
    listed, media_alt, property_description, property_name, template_media_alt, unit_type_description,
    unit_type_name,
};

#[derive(Debug, Clone, FromRow)]
pub struct PropertyRow {
    pub id: String,
    pub name: String,
    pub description: String,
    pub timezone: String,
    pub verification_status: String,
    pub host_name: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct UnitRow {
    pub id: String,
    pub name: String,
    pub description: String,
    pub max_adults: i32,
    pub max_children: i32,
    pub bedroom_count: i32,
    pub bed_count: i32,
    pub bathroom_count: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct PlanRow {
    pub id: String,
    pub unit_type_id: String,
    pub occupancy_basis: String,
    pub meal_plan: String,
    pub booking_mode: String,
    pub cancellation_policy_id: String,
    pub payment_policy_id: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct MediaRow {
    pub id: String,
    pub alt: String,
    pub is_cover: bool,
    pub mime_type: String,
    pub storage_key: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct UnitImageRow {
    pub unit_type_id: String,
    pub id: String,
    pub alt: String,
    pub is_cover: bool,
    pub storage_key: String,
}

pub async fn published_count(pool: &PgPool, city: City) -> Result<i64> {
    let sql = format!(
        r#"SELECT COUNT(*) FROM stay_properties p JOIN hosts h ON h.id = p."operatorId"
           WHERE p.city_id = $1 AND {}"#,
        listed("p", "h")
    );
    Ok(sqlx::query_scalar(&sql).bind(city.code()).fetch_one(pool).await?)
}

pub async fn property_is_listed(pool: &PgPool, city: City, property_id: &str) -> Result<bool> {
    let sql = format!(
        r#"SELECT EXISTS (SELECT 1 FROM stay_properties p JOIN hosts h ON h.id = p."operatorId"
           WHERE p.id = $1 AND p.city_id = $2 AND {})"#,
        listed("p", "h")
    );
    Ok(sqlx::query_scalar(&sql).bind(property_id).bind(city.code()).fetch_one(pool).await?)
}

pub async fn published_property(
    pool: &PgPool,
    city: City,
    property_id: &str,
    locale: &str,
) -> Result<Option<PropertyRow>> {
    let sql = format!(
        r#"SELECT p.id, {name} AS name, {description} AS description, p.timezone,
                  p."verificationStatus" AS verification_status, h.display_name AS host_name
           FROM stay_properties p JOIN hosts h ON h.id = p."operatorId"
           WHERE p.id = $1 AND p.city_id = $2 AND {listed}"#,
        name = property_name("p", "$3"),
        description = property_description("p", "$3"),
        listed = listed("p", "h")
    );
    Ok(sqlx::query_as::<_, PropertyRow>(&sql)
        .bind(property_id)
        .bind(city.code())
        .bind(locale)
        .fetch_optional(pool)
        .await?)
}

pub async fn active_unit_types(pool: &PgPool, property_id: &str, locale: &str) -> Result<Vec<UnitRow>> {
    let sql = format!(
        r#"SELECT u.id, {name} AS name, {description} AS description,
                  u."maxAdults" AS max_adults, u."maxChildren" AS max_children,
                  u."bedroomCount" AS bedroom_count, u."bedCount" AS bed_count, u."bathroomCount" AS bathroom_count
           FROM stay_unit_types u WHERE u."propertyId" = $1 AND u.status = 'ACTIVE' ORDER BY u.id"#,
        name = unit_type_name("u", "$2"),
        description = unit_type_description("u", "$2")
    );
    Ok(sqlx::query_as::<_, UnitRow>(&sql).bind(property_id).bind(locale).fetch_all(pool).await?)
}

pub async fn active_plans(pool: &PgPool, unit_type_ids: &[String]) -> Result<Vec<PlanRow>> {
    Ok(sqlx::query_as::<_, PlanRow>(
        r#"SELECT id, "unitTypeId" AS unit_type_id, "occupancyBasis" AS occupancy_basis,
                  "mealPlan" AS meal_plan, "bookingMode" AS booking_mode,
                  "cancellationPolicyVersionId" AS cancellation_policy_id,
                  "paymentPolicyVersionId" AS payment_policy_id
           FROM stay_rate_plans WHERE "unitTypeId" = ANY($1) AND status = 'ACTIVE' ORDER BY id"#,
    )
    .bind(unit_type_ids)
    .fetch_all(pool)
    .await?)
}

pub async fn amenity_codes(pool: &PgPool, property_id: &str) -> Result<Vec<String>> {
    let value: Value = sqlx::query_scalar("SELECT amenities FROM stay_properties WHERE id = $1")
        .bind(property_id)
        .fetch_one(pool)
        .await?;
    Ok(crate::guests::search::string_list(&value))
}

pub async fn policy_contents(pool: &PgPool, ids: &[String]) -> Result<Vec<(String, Value)>> {
    Ok(sqlx::query_as(
        r#"SELECT id, content FROM stay_policy_versions WHERE id = ANY($1) AND "tombstonedAt" IS NULL"#,
    )
    .bind(ids)
    .fetch_all(pool)
    .await?)
}

pub async fn ready_media(pool: &PgPool, property_id: &str, locale: &str) -> Result<Vec<MediaRow>> {
    let sql = format!(
        r#"SELECT m.id, {alt}, m."isCover" AS is_cover, m."mimeType" AS mime_type, m."storageKey" AS storage_key
           FROM stay_property_media m
           WHERE m."propertyId" = $1 AND m.status = 'READY' AND m."storageKey" IS NOT NULL
           ORDER BY m."isCover" DESC, m."sortOrder", m."createdAt", m.id"#,
        alt = media_alt("m", "$2")
    );
    Ok(sqlx::query_as::<_, MediaRow>(&sql).bind(property_id).bind(locale).fetch_all(pool).await?)
}

pub async fn public_media(pool: &PgPool, city: City, media_id: &str, locale: &str) -> Result<Option<MediaRow>> {
    let sql = format!(
        r#"SELECT m.id, {alt}, m."isCover" AS is_cover, m."mimeType" AS mime_type, m."storageKey" AS storage_key
           FROM stay_property_media m
           JOIN stay_properties p ON p.id = m."propertyId"
           JOIN hosts h ON h.id = p."operatorId"
           WHERE m.id = $1 AND m.status = 'READY' AND m."storageKey" IS NOT NULL
             AND p.city_id = $2 AND {listed}"#,
        alt = media_alt("m", "$3"),
        listed = listed("p", "h")
    );
    Ok(sqlx::query_as::<_, MediaRow>(&sql)
        .bind(media_id)
        .bind(city.code())
        .bind(locale)
        .fetch_optional(pool)
        .await?)
}

pub async fn public_template_media(
    pool: &PgPool,
    city: City,
    media_id: &str,
    locale: &str,
) -> Result<Option<MediaRow>> {
    let sql = format!(
        r#"SELECT m.id, {alt}, m."isCover" AS is_cover, m."mimeType" AS mime_type, m."storageKey" AS storage_key
           FROM stay_template_media m
           WHERE m.id = $1 AND m.status = 'READY' AND m."storageKey" IS NOT NULL AND EXISTS (
               SELECT 1 FROM stay_unit_types u
               JOIN stay_properties p ON p.id = u."propertyId"
               JOIN hosts h ON h.id = p."operatorId"
               WHERE u."templateId" = m."templateId" AND u.status = 'ACTIVE' AND p.city_id = $2 AND {listed})"#,
        alt = template_media_alt("m", "$3"),
        listed = listed("p", "h")
    );
    Ok(sqlx::query_as::<_, MediaRow>(&sql)
        .bind(media_id)
        .bind(city.code())
        .bind(locale)
        .fetch_optional(pool)
        .await?)
}

pub async fn unit_images(pool: &PgPool, unit_type_ids: &[String], locale: &str) -> Result<Vec<UnitImageRow>> {
    let sql = format!(
        r#"SELECT u.id AS unit_type_id, m.id, {alt}, m."isCover" AS is_cover, m."storageKey" AS storage_key
           FROM stay_unit_types u
           JOIN stay_template_media m ON m."templateId" = u."templateId" AND m.status = 'READY'
                AND m."storageKey" IS NOT NULL
           WHERE u.id = ANY($1)
           ORDER BY u.id, m."isCover" DESC, m."sortOrder", m."createdAt", m.id"#,
        alt = template_media_alt("m", "$2")
    );
    Ok(sqlx::query_as::<_, UnitImageRow>(&sql).bind(unit_type_ids).bind(locale).fetch_all(pool).await?)
}
