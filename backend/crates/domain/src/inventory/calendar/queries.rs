use std::collections::HashMap;

use chrono::NaiveDate;
use kernel::error::Result;
use kernel::paging::PageRequest;
use sqlx::{FromRow, PgPool};

use super::types::{CalendarBlock, CalendarReservation};

#[derive(Debug, FromRow)]
pub struct UnitRow {
    pub unit_type_id: String,
    pub name_en: String,
    pub name_fr: String,
    pub inventory_model: String,
    pub default_nightly_capacity: bool,
    pub inventory_version: Option<i32>,
}

#[derive(Debug, FromRow)]
pub struct NightRow {
    pub unit_type_id: String,
    pub stay_date: NaiveDate,
    pub capacity: i32,
    pub allocated: i32,
}

pub async fn units(pool: &PgPool, property_id: &str, page: PageRequest) -> Result<Vec<UnitRow>> {
    let rows = sqlx::query_as::<_, UnitRow>(
        r#"SELECT u.id AS unit_type_id, u."nameEn" AS name_en, u."nameFr" AS name_fr,
                  u."inventoryModel" AS inventory_model,
                  COALESCE(p."defaultNightlyCapacity", false) AS default_nightly_capacity,
                  p.version AS inventory_version
           FROM stay_unit_types u
           LEFT JOIN stay_inventory_pools p ON p."unitTypeId" = u.id
           WHERE u."propertyId" = $1 AND u.status <> 'ARCHIVED'
           ORDER BY u."nameEn", u.id LIMIT $2 OFFSET $3"#,
    )
    .bind(property_id)
    .bind(page.limit())
    .bind(page.offset())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn count_units(pool: &PgPool, property_id: &str) -> Result<i64> {
    let total = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM stay_unit_types WHERE "propertyId" = $1 AND status <> 'ARCHIVED'"#,
    )
    .bind(property_id)
    .fetch_one(pool)
    .await?;
    Ok(total)
}

pub async fn nights(
    pool: &PgPool,
    property_id: &str,
    from: NaiveDate,
    to: NaiveDate,
    unit_ids: &[String],
) -> Result<Vec<NightRow>> {
    let rows = sqlx::query_as::<_, NightRow>(
        r#"SELECT p."unitTypeId" AS unit_type_id, ($2::date + d.day_offset) AS stay_date,
                  COALESCE(n."sellableCapacity", CASE WHEN p."defaultNightlyCapacity" THEN p."sellableQuantity" ELSE 0 END) AS capacity,
                  (COALESCE(n."allocatedCount", 0) + stay_unallocated_physical_blocks(p.id, ($2::date + d.day_offset)))::int AS allocated
           FROM stay_inventory_pools p
           CROSS JOIN LATERAL generate_series(0, $3::date - $2::date - 1) AS d(day_offset)
           LEFT JOIN stay_night_inventory n ON n."poolId" = p.id AND n."stayDate" = ($2::date + d.day_offset)
           WHERE p."propertyId" = $1 AND p."unitTypeId" = ANY($4)"#,
    )
    .bind(property_id)
    .bind(from)
    .bind(to)
    .bind(unit_ids)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn reservations(
    pool: &PgPool,
    property_id: &str,
    from: NaiveDate,
    to: NaiveDate,
    with_guest_names: bool,
    unit_ids: Option<&[String]>,
    today: NaiveDate,
) -> Result<Vec<CalendarReservation>> {
    let rows = sqlx::query_as::<_, CalendarReservation>(
        r#"SELECT r.id, r.reference, r.state, r.version, r."totalAmount"::text AS total,
                  i."adultCount" AS adult_count, i."childCount" AS child_count, i."unitTypeId" AS unit_type_id,
                  COALESCE((SELECT a."physicalUnitId" FROM stay_unit_assignments a
                            WHERE a."reservationItemId" = i.id AND a."releasedAt" IS NULL
                            ORDER BY CASE WHEN a."startDate" <= $6 AND a."endDate" > $6 THEN 0
                                          WHEN a."startDate" > $6 THEN 1 ELSE 2 END,
                                     CASE WHEN a."startDate" > $6 THEN a."startDate" END ASC,
                                     a."endDate" DESC
                            LIMIT 1), i."physicalUnitId") AS physical_unit_id,
                  i."checkIn" AS check_in, i."checkOut" AS check_out,
                  CASE WHEN $4 THEN g."fullName" ELSE NULL END AS guest_name,
                  pending.proposal AS pending_amendment
           FROM stay_reservations r
           JOIN stay_reservation_items i ON i."reservationId" = r.id
           LEFT JOIN stay_reservation_guests g ON g."reservationItemId" = i.id AND g.role = 'PRIMARY'
           LEFT JOIN LATERAL (
               SELECT jsonb_build_object(
                   'checkIn', a."proposedContract"->>'checkIn',
                   'checkOut', a."proposedContract"->>'checkOut',
                   'total', a."proposedContract"->>'total',
                   'delta', a."deltaAmount"::text,
                   'expiresAt', a."expiresAt"
               ) AS proposal
               FROM stay_amendments a
               WHERE a."reservationId" = r.id AND a.state = 'PROPOSED'
                 AND a."expiresAt" > now() AND a."originalVersion" = r.version
               LIMIT 1
           ) pending ON true
           WHERE r."propertyId" = $1
             AND r.state IN ('PENDING_PAYMENT', 'REQUESTED', 'CONFIRMED', 'CHECKED_IN', 'CHECKED_OUT')
             AND i."checkIn" < $3 AND i."checkOut" > $2
             AND ($5::text[] IS NULL OR i."unitTypeId" = ANY($5))
           ORDER BY i."checkIn", r.id"#,
    )
    .bind(property_id)
    .bind(from)
    .bind(to)
    .bind(with_guest_names)
    .bind(unit_ids)
    .bind(today)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn blocks(
    pool: &PgPool,
    property_id: &str,
    from: NaiveDate,
    to: NaiveDate,
    unit_ids: Option<&[String]>,
) -> Result<Vec<CalendarBlock>> {
    let rows = sqlx::query_as::<_, CalendarBlock>(
        r#"SELECT b.id, COALESCE(b."unitTypeId", u."unitTypeId") AS unit_type_id,
                  b."physicalUnitId" AS physical_unit_id, b."startDate" AS start_date,
                  b."endDate" AS end_date, b.reason, b.version
           FROM stay_availability_blocks b
           LEFT JOIN stay_physical_units u ON u.id = b."physicalUnitId"
           WHERE b."propertyId" = $1 AND b."removedAt" IS NULL
             AND b."startDate" < $3 AND b."endDate" > $2
             AND ($4::text[] IS NULL OR COALESCE(b."unitTypeId", u."unitTypeId") = ANY($4))
           ORDER BY b."startDate", b.id"#,
    )
    .bind(property_id)
    .bind(from)
    .bind(to)
    .bind(unit_ids)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn cover_keys(pool: &PgPool, unit_type_ids: &[String]) -> Result<HashMap<String, String>> {
    let rows: Vec<(String, String)> = sqlx::query_as(
        r#"SELECT DISTINCT ON (u.id) u.id, m."storageKey"
           FROM stay_unit_types u
           JOIN stay_template_media m ON m."templateId" = u."templateId" AND m.status = 'READY' AND m."storageKey" IS NOT NULL
           WHERE u.id = ANY($1)
           ORDER BY u.id, m."isCover" DESC, m."sortOrder", m."createdAt", m.id"#,
    )
    .bind(unit_type_ids)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().collect())
}
