use kernel::{Page, PageRequest, Result};
use sqlx::{PgPool, postgres::PgRow, FromRow};

use super::reservation_detail_types::{Amendment, Case, Core, Guest, Item, Timeline};

pub async fn core(pool: &PgPool, city: &str, id: &str) -> Result<Option<Core>> {
    Ok(sqlx::query_as::<_, Core>(r#"SELECT r.id, r.reference, r.state, r."bookingMode" AS booking_mode, r.source, r.city_id AS city, r.version,
        r."operatorId" AS operator_id, h.display_name AS operator_name, r."propertyId" AS property_id,
        p."nameEn" AS property_name_en, p."nameFr" AS property_name_fr, r."totalAmount"::text AS total_amount,
        r."commissionRateBps" AS commission_rate_bps, r."commissionAmount"::text AS commission_amount,
        r."operatorNetAmount"::text AS operator_net_amount, r."commissionChargedAmount"::text AS commission_held,
        COALESCE((SELECT -SUM(e.amount) FROM stay_credit_entries e WHERE e."reservationId" = r.id AND e.city_id = r.city_id AND e."entryType" = 'COMMISSION'), 0)::text AS commission_taken,
        COALESCE((SELECT SUM(e.amount) FROM stay_credit_entries e WHERE e."reservationId" = r.id AND e.city_id = r.city_id AND e."entryType" = 'REFUND'), 0)::text AS commission_refunded,
        r."paymentRef" AS payment_ref, r."approvalDeadlineAt" AS approval_deadline_at, r."cancelledAt" AS cancelled_at,
        r."cancellationReason" AS cancellation_reason, r."cancellationChargeAmount"::text AS cancellation_charge_amount,
        r."specialRequest" AS special_request, r."createdAt" AS created_at, r."updatedAt" AS updated_at, r."contractSnapshot" AS contract
        FROM stay_reservations r JOIN hosts h ON h.id = r."operatorId" AND h.city_id = r.city_id
        JOIN stay_properties p ON p.id = r."propertyId" AND p.city_id = r.city_id
        WHERE r.id = $1 AND r.city_id = $2"#).bind(id).bind(city).fetch_optional(pool).await?)
}

async fn page<T>(pool: &PgPool, id: &str, city: &str, request: PageRequest, count_sql: &str, rows_sql: &str) -> Result<Page<T>>
where T: for<'r> FromRow<'r, PgRow> + Send + Unpin + serde::Serialize {
    let total = sqlx::query_scalar::<_, i64>(count_sql).bind(id).bind(city).fetch_one(pool).await?;
    let rows = sqlx::query_as::<_, T>(rows_sql).bind(id).bind(city).bind(request.limit()).bind(request.offset()).fetch_all(pool).await?;
    Ok(Page::new(rows, total, request))
}

pub async fn items(pool: &PgPool, id: &str, city: &str, request: PageRequest) -> Result<Page<Item>> {
    page(pool, id, city, request,
        r#"SELECT COUNT(*) FROM stay_reservation_items i JOIN stay_unit_types t ON t.id = i."unitTypeId" AND t.city_id = $2 WHERE i."reservationId" = $1 AND EXISTS (SELECT 1 FROM stay_reservations scoped_reservation WHERE scoped_reservation.id = $1 AND scoped_reservation.city_id = $2)"#,
        r#"SELECT i.id, t.id AS unit_type_id, t."nameEn" AS unit_type_name_en, t."nameFr" AS unit_type_name_fr, u."localCode" AS unit_code,
           i."checkIn" AS check_in, i."checkOut" AS check_out, i.quantity, i."adultCount" AS adult_count,
           i."childCount" AS child_count, i."subtotalAmount"::text AS subtotal_amount
           FROM stay_reservation_items i JOIN stay_unit_types t ON t.id = i."unitTypeId" AND t.city_id = $2
           LEFT JOIN stay_physical_units u ON u.id = i."physicalUnitId" AND u.city_id = $2 WHERE i."reservationId" = $1
           AND EXISTS (SELECT 1 FROM stay_reservations scoped_reservation WHERE scoped_reservation.id = $1 AND scoped_reservation.city_id = $2) ORDER BY i."checkIn", i.id LIMIT $3 OFFSET $4"#).await
}

pub async fn guests(pool: &PgPool, id: &str, city: &str, request: PageRequest) -> Result<Page<Guest>> {
    page(pool, id, city, request,
        r#"SELECT COUNT(*) FROM stay_reservation_guests g JOIN stay_reservation_items i ON i.id = g."reservationItemId" WHERE i."reservationId" = $1 AND EXISTS (SELECT 1 FROM stay_reservations scoped_reservation WHERE scoped_reservation.id = $1 AND scoped_reservation.city_id = $2)"#,
        r#"SELECT g."reservationItemId" AS item_id, g."fullName" AS full_name, g."contactPhone" AS contact_phone, g."contactEmail" AS contact_email, g.role
           FROM stay_reservation_guests g JOIN stay_reservation_items i ON i.id = g."reservationItemId" WHERE i."reservationId" = $1
           AND EXISTS (SELECT 1 FROM stay_reservations scoped_reservation WHERE scoped_reservation.id = $1 AND scoped_reservation.city_id = $2) ORDER BY g.role, g."fullName", g.id LIMIT $3 OFFSET $4"#).await
}

pub async fn timeline(pool: &PgPool, id: &str, city: &str, request: PageRequest) -> Result<Page<Timeline>> {
    page(pool, id, city, request,
        "SELECT COUNT(*) FROM kernel_audit_records WHERE resource_type = 'reservation' AND resource_id = $1 AND city_id = $2 AND EXISTS (SELECT 1 FROM stay_reservations scoped_reservation WHERE scoped_reservation.id = $1 AND scoped_reservation.city_id = $2)",
        "SELECT id, action, reason, created_at FROM kernel_audit_records WHERE resource_type = 'reservation' AND resource_id = $1 AND city_id = $2 AND EXISTS (SELECT 1 FROM stay_reservations scoped_reservation WHERE scoped_reservation.id = $1 AND scoped_reservation.city_id = $2) ORDER BY created_at DESC, id LIMIT $3 OFFSET $4").await
}

pub async fn amendments(pool: &PgPool, id: &str, city: &str, request: PageRequest) -> Result<Page<Amendment>> {
    page(pool, id, city, request,
        r#"SELECT COUNT(*) FROM stay_amendments WHERE "reservationId" = $1 AND EXISTS (SELECT 1 FROM stay_reservations scoped_reservation WHERE scoped_reservation.id = $1 AND scoped_reservation.city_id = $2)"#,
        r#"SELECT id, state, "deltaAmount"::text AS delta_amount, "expiresAt" AS expires_at, "createdAt" AS created_at, "respondedAt" AS responded_at
           FROM stay_amendments WHERE "reservationId" = $1 AND EXISTS (SELECT 1 FROM stay_reservations scoped_reservation WHERE scoped_reservation.id = $1 AND scoped_reservation.city_id = $2) ORDER BY "createdAt" DESC, id LIMIT $3 OFFSET $4"#).await
}

pub async fn cases(pool: &PgPool, id: &str, city: &str, request: PageRequest) -> Result<Page<Case>> {
    page(pool, id, city, request,
        r#"SELECT COUNT(*) FROM stay_cases WHERE "reservationId" = $1 AND EXISTS (SELECT 1 FROM stay_reservations scoped_reservation WHERE scoped_reservation.id = $1 AND scoped_reservation.city_id = $2)"#,
        r#"SELECT id, "caseType" AS case_type, state, "deadlineAt" AS deadline_at FROM stay_cases WHERE "reservationId" = $1 AND EXISTS (SELECT 1 FROM stay_reservations scoped_reservation WHERE scoped_reservation.id = $1 AND scoped_reservation.city_id = $2) ORDER BY "createdAt" DESC, id LIMIT $3 OFFSET $4"#).await
}
