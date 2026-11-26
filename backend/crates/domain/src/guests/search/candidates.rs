use std::collections::HashMap;

use kernel::city::City;
use kernel::error::Result;
use serde_json::Value;
use sqlx::{FromRow, PgConnection, PgPool};

use super::query::{SearchQuery, Window, PUBLIC_COORDINATE_DECIMALS};
use crate::guests::listing::{listed, media_alt, property_name, unit_type_name};

const CANDIDATE_SQL: &str = r#"
WITH cand AS (
    SELECT u.id AS unit_type_id, u."propertyId" AS property_id, u."inventoryModel" AS inventory_model,
           u.accessibility, u."categoryCode" AS category_code, @UNIT_NAME@ AS unit_name,
           u."bedroomCount" AS bedroom_count, u."bedCount" AS bed_count, u."bathroomCount" AS bathroom_count
    FROM stay_unit_types u
    JOIN stay_properties p ON p.id = u."propertyId"
    JOIN hosts h ON h.id = p."operatorId"
    WHERE p.city_id = $1 AND u.status = 'ACTIVE' AND @LISTED@
      AND u."maxAdults" >= $2 AND u."maxChildren" >= $3
      AND (cardinality($4::text[]) = 0 OR u.accessibility ?& $4::text[])
      AND (cardinality($5::text[]) = 0
           OR (CASE WHEN u."inventoryModel" = 'WHOLE_UNIT' THEN 'ENTIRE_PLACE' ELSE 'ROOM' END) = ANY($5::text[]))
      AND ($7::text IS NULL OR p.id = $7::text)
      AND ($22::text IS NULL OR strpos(lower(p."nameEn"), lower($22)) > 0 OR strpos(lower(p."nameFr"), lower($22)) > 0)
      AND (cardinality($20::text[]) = 0 OR u."categoryCode" = ANY($20::text[]))
      AND ($10::float8 IS NULL OR ST_DWithin(@EXACT_POINT@, ST_SetSRID(ST_MakePoint($9::float8, $8::float8), 4326)::geography,
            $10::float8 * 1000.0 + 2000.0))
),
open AS (
    SELECT pl."unitTypeId" AS unit_type_id,
           MIN(COALESCE(n."sellableCapacity", CASE WHEN pl."defaultNightlyCapacity" THEN pl."sellableQuantity" ELSE 0 END) - COALESCE(n."allocatedCount", 0) - stay_unallocated_physical_blocks(pl.id, ($11::date + d.day_offset)))::bigint AS remaining
    FROM stay_inventory_pools pl
    CROSS JOIN LATERAL generate_series(0, $12::date - $11::date - 1) AS d(day_offset)
    LEFT JOIN stay_night_inventory n ON n."poolId" = pl.id AND n."stayDate" = ($11::date + d.day_offset)
    WHERE pl."unitTypeId" IN (SELECT unit_type_id FROM cand)
    GROUP BY pl."unitTypeId"
    HAVING COUNT(*) = $13::bigint
       AND stay_room_available(pl."unitTypeId", $11::date, $12::date, '')
       AND MIN(COALESCE(n."sellableCapacity", CASE WHEN pl."defaultNightlyCapacity" THEN pl."sellableQuantity" ELSE 0 END) - COALESCE(n."allocatedCount", 0) - stay_unallocated_physical_blocks(pl.id, ($11::date + d.day_offset))) > 0
    UNION ALL
    SELECT unit_type_id, NULL::bigint FROM cand WHERE $13::bigint = 0
)
SELECT c.property_id, @PROPERTY_NAME@ AS name,
       p."verificationStatus" AS verification_status, p.timezone,
       c.unit_type_id, c.category_code, c.unit_name, c.inventory_model, c.accessibility,
       c.bedroom_count, c.bed_count, c.bathroom_count, p.amenities,
       r.id AS plan_id, r."occupancyBasis" AS occupancy_basis, r."mealPlan" AS meal_plan,
       r."bookingMode" AS booking_mode, r."childNightlyAmount"::text AS child_nightly_amount,
       cp.content AS cancellation_content, pp.content AS payment_content,
       ou.remaining AS remaining_rooms,
       @DISTANCE@ AS distance_km
FROM cand c
JOIN open ou ON ou.unit_type_id = c.unit_type_id
JOIN stay_properties p ON p.id = c.property_id
JOIN stay_rate_plans r ON r."unitTypeId" = c.unit_type_id AND r.status = 'ACTIVE'
JOIN stay_policy_versions cp ON cp.id = r."cancellationPolicyVersionId"
JOIN stay_policy_versions pp ON pp.id = r."paymentPolicyVersionId"
WHERE ($14::bool = FALSE OR r."bookingMode" = 'INSTANT')
  AND (cardinality($15::text[]) = 0 OR r."mealPlan" = ANY($15::text[]))
  AND (cardinality($16::text[]) = 0 OR pp.content->>'collection' = ANY($16::text[]))
  AND ($17::bigint IS NULL OR COALESCE((cp.content->>'freeUntilHours')::bigint, -1) >= $17::bigint)
  AND ($18::bigint IS NULL OR COALESCE((cp.content->>'penaltyPercent')::bigint, 101) <= $18::bigint)
  AND ($10::float8 IS NULL OR @DISTANCE@ <= $10::float8)
  AND (cardinality($6::text[]) = 0 OR p.amenities ?& $6::text[])
ORDER BY c.property_id, c.unit_type_id, r.id
LIMIT $19::bigint OFFSET $21::bigint
"#;

#[derive(Debug, Clone, FromRow)]
pub struct CandidateRow {
    pub property_id: String,
    pub name: String,
    pub verification_status: String,
    pub timezone: String,
    pub unit_type_id: String,
    pub category_code: Option<String>,
    pub unit_name: String,
    pub inventory_model: String,
    pub accessibility: Value,
    pub bedroom_count: i32,
    pub bed_count: i32,
    pub bathroom_count: i32,
    pub amenities: Value,
    pub plan_id: String,
    pub occupancy_basis: String,
    pub meal_plan: String,
    pub booking_mode: String,
    pub child_nightly_amount: String,
    pub cancellation_content: Value,
    pub payment_content: Value,
    pub remaining_rooms: Option<i64>,
    pub distance_km: Option<f64>,
}

const EXACT_POINT: &str = "ST_SetSRID(ST_MakePoint(p.longitude::float8, p.latitude::float8), 4326)::geography";

fn distance_sql() -> String {
    format!(
        "(ST_Distance(ST_SetSRID(ST_MakePoint(round(p.longitude::numeric, {PUBLIC_COORDINATE_DECIMALS})::float8, \
         round(p.latitude::numeric, {PUBLIC_COORDINATE_DECIMALS})::float8), 4326)::geography, \
         ST_SetSRID(ST_MakePoint($9::float8, $8::float8), 4326)::geography) / 1000.0)"
    )
}

fn candidate_sql() -> String {
    CANDIDATE_SQL
        .replace("@LISTED@", &listed("p", "h"))
        .replace("@UNIT_NAME@", &unit_type_name("u", "$23"))
        .replace("@PROPERTY_NAME@", &property_name("p", "$23"))
        .replace("@EXACT_POINT@", EXACT_POINT)
        .replace("@DISTANCE@", &distance_sql())
}

pub async fn candidates(
    connection: &mut PgConnection,
    city: City,
    locale: &str,
    query: &SearchQuery,
    window: Option<&Window>,
    limit: i64,
    offset: i64,
) -> Result<Vec<CandidateRow>> {
    let origin = query.origin.as_ref();
    Ok(sqlx::query_as::<_, CandidateRow>(&candidate_sql())
        .bind(city.code())
        .bind(query.adults)
        .bind(query.children)
        .bind(&query.accessibility)
        .bind(&query.property_types)
        .bind(&query.amenities)
        .bind(query.property_id.as_deref())
        .bind(origin.map(|origin| origin.latitude))
        .bind(origin.map(|origin| origin.longitude))
        .bind(query.max_distance_km)
        .bind(window.map(|window| window.check_in))
        .bind(window.map(|window| window.check_out))
        .bind(window.map_or(0, |window| window.nights))
        .bind(query.instant_booking)
        .bind(&query.meal_plans)
        .bind(&query.payment_collections)
        .bind(query.min_free_cancellation_hours)
        .bind(query.max_penalty_percent)
        .bind(limit)
        .bind(&query.categories)
        .bind(offset)
        .bind(query.search.as_deref())
        .bind(locale)
        .fetch_all(connection)
        .await?)
}

pub struct CoverRef {
    pub media_id: String,
    pub alt: String,
    pub storage_key: String,
}

pub async fn cover_keys(
    pool: &PgPool,
    property_ids: &[String],
    locale: &str,
) -> Result<HashMap<String, CoverRef>> {
    if property_ids.is_empty() {
        return Ok(HashMap::new());
    }
    let rows: Vec<(String, String, String, String)> = sqlx::query_as(&format!(
        r#"SELECT DISTINCT ON (m."propertyId") m."propertyId", m.id, {alt}, m."storageKey"
           FROM stay_property_media m
           WHERE m."propertyId" = ANY($1) AND m.status = 'READY' AND m."storageKey" IS NOT NULL
           ORDER BY m."propertyId", m."isCover" DESC, m."sortOrder", m."createdAt", m.id"#,
        alt = media_alt("m", "$2")
    ))
    .bind(property_ids)
    .bind(locale)
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|(property_id, media_id, alt, storage_key)| (property_id, CoverRef { media_id, alt, storage_key }))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn candidate_sql_has_no_unreplaced_tokens_and_uses_postgis() {
        let sql = candidate_sql();
        assert!(!sql.contains('@'));
        assert!(sql.contains("ST_Distance"));
        assert!(sql.contains("ST_DWithin"));
        assert!(sql.contains("p.city_id = $1"));
    }

    #[test]
    fn distance_rounds_property_coordinates_like_the_search_point() {
        let sql = distance_sql();
        assert!(sql.contains(&format!("round(p.latitude::numeric, {PUBLIC_COORDINATE_DECIMALS})")));
        assert!(sql.contains(&format!("round(p.longitude::numeric, {PUBLIC_COORDINATE_DECIMALS})")));
    }

    #[test]
    fn search_text_matches_both_language_columns() {
        let sql = candidate_sql();
        assert!(sql.contains(r#"lower(p."nameEn")"#));
        assert!(sql.contains(r#"lower(p."nameFr")"#));
    }
}
