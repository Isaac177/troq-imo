use chrono::NaiveDate;
use kernel::Result;
use sqlx::PgPool;

use super::dashboard_types::{DashboardDay, DashboardFigures};

pub async fn figures(
    pool: &PgPool,
    from: NaiveDate,
    to: NaiveDate,
    host_id: Option<&str>,
    cities: &[&str],
    city: Option<&str>,
) -> Result<DashboardFigures> {
    let disabled = crate::billing::credits::state::disabled_sql("h.id", "h.city_id");
    let sql = format!(r#"
        WITH reservations AS (
            SELECT * FROM stay_reservations r
            WHERE r.city_id = ANY($4) AND ($5::text IS NULL OR r.city_id = $5)
              AND ($3::text IS NULL OR r."operatorId" = $3)
        ), cases AS (
            SELECT * FROM stay_cases c
            WHERE c.city_id = ANY($4) AND ($5::text IS NULL OR c.city_id = $5)
              AND ($3::text IS NULL OR c."operatorId" = $3)
        ), hosts AS (
            SELECT * FROM hosts h
            WHERE h.city_id = ANY($4) AND ($5::text IS NULL OR h.city_id = $5)
              AND ($3::text IS NULL OR h.id = $3)
        ), period_reservations AS (
            SELECT * FROM reservations r
            WHERE (r."createdAt" AT TIME ZONE CASE r.city_id WHEN 'KIN' THEN 'Africa/Kinshasa' ELSE 'Africa/Lubumbashi' END)::date BETWEEN $1 AND $2
        ), period_cases AS (
            SELECT * FROM cases c
            WHERE (c."createdAt" AT TIME ZONE CASE c.city_id WHEN 'KIN' THEN 'Africa/Kinshasa' ELSE 'Africa/Lubumbashi' END)::date BETWEEN $1 AND $2
        ), period_credits AS (
            SELECT e.* FROM stay_credit_entries e JOIN hosts h ON h.id = e."operatorId" AND h.city_id = e.city_id
            WHERE (e."createdAt" AT TIME ZONE CASE e.city_id WHEN 'KIN' THEN 'Africa/Kinshasa' ELSE 'Africa/Lubumbashi' END)::date BETWEEN $1 AND $2
        )
        SELECT
            (SELECT COUNT(*) FROM period_reservations WHERE state IN ('CONFIRMED', 'CHECKED_IN', 'CHECKED_OUT')) AS bookings,
            (SELECT COALESCE(SUM("totalAmount"), 0)::text FROM period_reservations WHERE state IN ('CONFIRMED', 'CHECKED_IN', 'CHECKED_OUT')) AS revenue,
            (SELECT COUNT(*) FROM period_reservations WHERE state = 'CANCELLED') AS cancellations,
            (SELECT COALESCE(SUM(n."allocatedCount"), 0)::bigint FROM stay_night_inventory n
                JOIN stay_inventory_pools pl ON pl.id = n."poolId"
                JOIN stay_properties p ON p.id = pl."propertyId" AND p.city_id = pl.city_id
                JOIN hosts h ON h.id = p."operatorId" AND h.city_id = p.city_id
                WHERE n."stayDate" BETWEEN $1 AND $2) AS nights_sold,
            (SELECT COALESCE(SUM(n."sellableCapacity"), 0)::bigint FROM stay_night_inventory n
                JOIN stay_inventory_pools pl ON pl.id = n."poolId"
                JOIN stay_properties p ON p.id = pl."propertyId" AND p.city_id = pl.city_id
                JOIN hosts h ON h.id = p."operatorId" AND h.city_id = p.city_id
                WHERE n."stayDate" BETWEEN $1 AND $2) AS nights_available,
            (SELECT COUNT(*) FROM reservations WHERE state = 'REQUESTED') AS pending_requests,
            (SELECT COUNT(*) FROM cases WHERE state IN ('OPEN', 'AWAITING_RESPONSE')) AS open_cases,
            (SELECT COALESCE(SUM("commissionChargedAmount"), 0)::text FROM period_reservations WHERE state IN ('CONFIRMED', 'CHECKED_IN')) AS commission_upcoming,
            (SELECT COALESCE(SUM("commissionChargedAmount"), 0)::text FROM period_reservations WHERE state = 'CHECKED_OUT') AS commission_completed,
            (SELECT COALESCE(-SUM(amount), 0)::text FROM period_credits WHERE "entryType" IN ('COMMISSION', 'REFUND')) AS commission_taken,
            (SELECT COALESCE(SUM(amount), 0)::text FROM period_credits WHERE "entryType" = 'TOPUP') AS credits_topped_up,
            (SELECT COALESCE(SUM(a.balance), 0)::text FROM stay_credit_accounts a JOIN hosts h ON h.id = a."operatorId" AND h.city_id = a.city_id) AS credit_balance,
            (SELECT COUNT(*) FROM hosts h WHERE {disabled}) AS disabled_businesses,
            (SELECT COALESCE(SUM(d."approvedAmount"), 0)::text FROM stay_claim_decisions d JOIN period_cases c ON c.id = d."caseId" WHERE d."payoutState" = 'PENDING') AS payouts_pending,
            (SELECT COALESCE(SUM(d."approvedAmount"), 0)::text FROM stay_claim_decisions d JOIN period_cases c ON c.id = d."caseId" WHERE d."payoutState" = 'PAID') AS payouts_paid
    "#);
    Ok(sqlx::query_as::<_, DashboardFigures>(&sql)
        .bind(from).bind(to).bind(host_id).bind(cities).bind(city).fetch_one(pool).await?)
}

pub async fn series(
    pool: &PgPool,
    from: NaiveDate,
    to: NaiveDate,
    host_id: Option<&str>,
    cities: &[&str],
) -> Result<Vec<DashboardDay>> {
    Ok(sqlx::query_as::<_, DashboardDay>(r#"
        SELECT d::date AS date,
            COUNT(r.id) FILTER (WHERE r.state IN ('CONFIRMED', 'CHECKED_IN', 'CHECKED_OUT')) AS bookings,
            COALESCE(SUM(r."totalAmount") FILTER (WHERE r.state IN ('CONFIRMED', 'CHECKED_IN', 'CHECKED_OUT')), 0)::text AS revenue,
            COUNT(r.id) FILTER (WHERE r.state = 'CANCELLED') AS cancellations
        FROM generate_series($1::date, $2::date, interval '1 day') d
        LEFT JOIN stay_reservations r
          ON (r."createdAt" AT TIME ZONE CASE r.city_id WHEN 'KIN' THEN 'Africa/Kinshasa' ELSE 'Africa/Lubumbashi' END)::date = d::date
         AND r.city_id = ANY($4) AND ($3::text IS NULL OR r."operatorId" = $3)
        GROUP BY d ORDER BY d
    "#).bind(from).bind(to).bind(host_id).bind(cities).fetch_all(pool).await?)
}
