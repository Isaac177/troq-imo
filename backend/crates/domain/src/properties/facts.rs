pub fn property_facts_sql(alias: &str) -> String {
    format!(
        r#"(SELECT m.id FROM stay_property_media m WHERE m."propertyId" = {alias}.id AND m."isCover" AND m.status = 'READY') AS cover_media_id,
    (SELECT m.version FROM stay_property_media m WHERE m."propertyId" = {alias}.id AND m."isCover" AND m.status = 'READY') AS cover_media_version,
    (SELECT m."mimeType" FROM stay_property_media m WHERE m."propertyId" = {alias}.id AND m."isCover" AND m.status = 'READY') AS cover_media_mime,
    (SELECT COUNT(*) FROM stay_property_media m WHERE m."propertyId" = {alias}.id AND m.status = 'READY') AS photo_count,
    (SELECT COUNT(*) FROM stay_unit_types u WHERE u."propertyId" = {alias}.id AND u.status = 'ACTIVE') AS unit_type_count,
    (SELECT COALESCE(SUM(ip."sellableQuantity"), 0)::bigint FROM stay_inventory_pools ip
        JOIN stay_unit_types u ON u.id = ip."unitTypeId" WHERE u."propertyId" = {alias}.id AND u.status = 'ACTIVE') AS unit_count,
    (SELECT MIN(rr.amount)::text FROM stay_rate_plans rp
        JOIN stay_unit_types u ON u.id = rp."unitTypeId" AND u.status = 'ACTIVE'
        JOIN stay_rate_rules rr ON rr."ratePlanId" = rp.id AND rr."adjustmentType" = 'OVERRIDE' AND rr."startDate" IS NULL
        WHERE rp."propertyId" = {alias}.id AND rp.status = 'ACTIVE') AS from_amount,
    (SELECT COUNT(*) FROM stay_reservations r WHERE r."propertyId" = {alias}.id
        AND r.state IN ('REQUESTED', 'CONFIRMED', 'CHECKED_IN')) AS open_reservations,
    (SELECT COUNT(*) FROM stay_reservations r WHERE r."propertyId" = {alias}.id AND r.state = 'REQUESTED') AS pending_requests"#
    )
}

pub const READINESS_FACTS_SQL: &str = r#"SELECT
    EXISTS (SELECT 1 FROM stay_unit_types u
            JOIN stay_physical_units pu ON pu."unitTypeId" = u.id AND pu."propertyId" = u."propertyId"
            WHERE u."propertyId" = $1 AND u.status = 'ACTIVE' AND pu.status = 'ACTIVE') AS has_sellable_unit,
    EXISTS (SELECT 1 FROM stay_rate_plans p
            JOIN stay_unit_types u ON u.id = p."unitTypeId" AND u."propertyId" = p."propertyId"
            JOIN stay_rate_rules r ON r."ratePlanId" = p.id
            WHERE p."propertyId" = $1 AND p.status = 'ACTIVE' AND u.status = 'ACTIVE'
              AND r."adjustmentType" = 'OVERRIDE' AND r."startDate" IS NULL AND r."endDate" IS NULL
              AND r."weekdayMask" = 127) AS has_base_rate,
    EXISTS (SELECT 1 FROM stay_property_media m
            WHERE m."propertyId" = $1 AND m."isCover" AND m.status = 'READY') AS has_cover_photo"#;

pub const UPCOMING_STAYS_SQL: &str = r#"SELECT COUNT(*) FROM stay_reservation_items i
    JOIN stay_reservations r ON r.id = i."reservationId"
    WHERE i."propertyId" = $1
      AND r.state IN ('PENDING_PAYMENT', 'REQUESTED', 'CONFIRMED', 'CHECKED_IN')
      AND i."checkOut" > (now() AT TIME ZONE $2)::date"#;
