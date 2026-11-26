use chrono::NaiveDate;
use kernel::auth::Actor;
use kernel::error::Result;
use kernel::paging::{Page, PageRequest};
use kernel::permissions::names::PROPERTIES_MODERATE;
use kernel::Kernel;
use serde::Serialize;
use utoipa::ToSchema;

use super::dashboard_types::DashboardFigures;

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HostFigures {
    pub host_id: String,
    pub host_name: String,
    pub city: String,
    #[serde(flatten)]
    pub figures: DashboardFigures,
}

struct Row {
    host_id: String,
    host_name: String,
    city: String,
    figures: DashboardFigures,
}

impl sqlx::FromRow<'_, sqlx::postgres::PgRow> for Row {
    fn from_row(row: &sqlx::postgres::PgRow) -> sqlx::Result<Self> {
        use sqlx::Row as _;
        Ok(Self {
            host_id: row.try_get("host_id")?,
            host_name: row.try_get("host_name")?,
            city: row.try_get("city")?,
            figures: DashboardFigures {
                bookings: row.try_get("bookings")?,
                revenue: row.try_get("revenue")?,
                cancellations: row.try_get("cancellations")?,
                nights_sold: row.try_get("nights_sold")?,
                nights_available: row.try_get("nights_available")?,
                pending_requests: row.try_get("pending_requests")?,
                open_cases: row.try_get("open_cases")?,
                commission_upcoming: row.try_get("commission_upcoming")?,
                commission_completed: row.try_get("commission_completed")?,
                commission_taken: row.try_get("commission_taken")?,
                credits_topped_up: row.try_get("credits_topped_up")?,
                credit_balance: row.try_get("credit_balance")?,
                disabled_businesses: row.try_get("disabled_businesses")?,
                payouts_pending: row.try_get("payouts_pending")?,
                payouts_paid: row.try_get("payouts_paid")?,
            },
        })
    }
}

const SELECT: &str = r#"
WITH reservations AS (
    SELECT * FROM stay_reservations r WHERE r.city_id = ANY($4)
), cases AS (
    SELECT * FROM stay_cases c WHERE c.city_id = ANY($4)
), period_reservations AS (
    SELECT * FROM reservations r
    WHERE (r."createdAt" AT TIME ZONE CASE r.city_id WHEN 'KIN' THEN 'Africa/Kinshasa' ELSE 'Africa/Lubumbashi' END)::date BETWEEN $1 AND $2
), period_cases AS (
    SELECT * FROM cases c
    WHERE (c."createdAt" AT TIME ZONE CASE c.city_id WHEN 'KIN' THEN 'Africa/Kinshasa' ELSE 'Africa/Lubumbashi' END)::date BETWEEN $1 AND $2
), period_credits AS (
    SELECT e.* FROM stay_credit_entries e
    WHERE (e."createdAt" AT TIME ZONE CASE e.city_id WHEN 'KIN' THEN 'Africa/Kinshasa' ELSE 'Africa/Lubumbashi' END)::date BETWEEN $1 AND $2
)
SELECT h.id AS host_id, h.display_name AS host_name, h.city_id AS city,
    (SELECT COUNT(*) FROM period_reservations r WHERE r."operatorId" = h.id AND r.state IN ('CONFIRMED','CHECKED_IN','CHECKED_OUT')) AS bookings,
    (SELECT COALESCE(SUM(r."totalAmount"), 0)::text FROM period_reservations r WHERE r."operatorId" = h.id AND r.state IN ('CONFIRMED','CHECKED_IN','CHECKED_OUT')) AS revenue,
    (SELECT COUNT(*) FROM period_reservations r WHERE r."operatorId" = h.id AND r.state = 'CANCELLED') AS cancellations,
    (SELECT COALESCE(SUM(n."allocatedCount"), 0)::bigint FROM stay_night_inventory n
       JOIN stay_inventory_pools pl ON pl.id = n."poolId"
       JOIN stay_properties p ON p.id = pl."propertyId" AND p.city_id = pl.city_id
       WHERE p."operatorId" = h.id AND n."stayDate" BETWEEN $1 AND $2) AS nights_sold,
    (SELECT COALESCE(SUM(n."sellableCapacity"), 0)::bigint FROM stay_night_inventory n
       JOIN stay_inventory_pools pl ON pl.id = n."poolId"
       JOIN stay_properties p ON p.id = pl."propertyId" AND p.city_id = pl.city_id
       WHERE p."operatorId" = h.id AND n."stayDate" BETWEEN $1 AND $2) AS nights_available,
    (SELECT COUNT(*) FROM reservations r WHERE r."operatorId" = h.id AND r.state = 'REQUESTED') AS pending_requests,
    (SELECT COUNT(*) FROM cases c WHERE c."operatorId" = h.id AND c.state IN ('OPEN','AWAITING_RESPONSE')) AS open_cases,
    (SELECT COALESCE(SUM(r."commissionChargedAmount"), 0)::text FROM period_reservations r WHERE r."operatorId" = h.id AND r.state IN ('CONFIRMED','CHECKED_IN')) AS commission_upcoming,
    (SELECT COALESCE(SUM(r."commissionChargedAmount"), 0)::text FROM period_reservations r WHERE r."operatorId" = h.id AND r.state = 'CHECKED_OUT') AS commission_completed,
    (SELECT COALESCE(-SUM(e.amount), 0)::text FROM period_credits e WHERE e."operatorId" = h.id AND e."entryType" IN ('COMMISSION','REFUND')) AS commission_taken,
    (SELECT COALESCE(SUM(e.amount), 0)::text FROM period_credits e WHERE e."operatorId" = h.id AND e."entryType" = 'TOPUP') AS credits_topped_up,
    COALESCE((SELECT SUM(a.balance) FROM stay_credit_accounts a WHERE a."operatorId" = h.id AND a.city_id = h.city_id), 0)::text AS credit_balance,
    CASE WHEN {disabled} THEN 1 ELSE 0 END::bigint AS disabled_businesses,
    (SELECT COALESCE(SUM(d."approvedAmount"), 0)::text FROM stay_claim_decisions d JOIN period_cases c ON c.id = d."caseId" WHERE c."operatorId" = h.id AND d."payoutState" = 'PENDING') AS payouts_pending,
    (SELECT COALESCE(SUM(d."approvedAmount"), 0)::text FROM stay_claim_decisions d JOIN period_cases c ON c.id = d."caseId" WHERE c."operatorId" = h.id AND d."payoutState" = 'PAID') AS payouts_paid
FROM hosts h
WHERE h.city_id = ANY($4) AND ($3::text IS NULL OR h.id = $3)
"#;

pub async fn list_page(
    kernel: &Kernel,
    actor: &Actor,
    from: NaiveDate,
    to: NaiveDate,
    host_id: Option<&str>,
    page: PageRequest,
) -> Result<Page<HostFigures>> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    let page = page.validated()?;
    let city_codes: Vec<&str> = kernel.config.cities.iter().map(|city| city.code()).collect();
    let disabled = crate::billing::credits::state::disabled_sql("h.id", "h.city_id");
    let sql = SELECT.replace("{disabled}", &disabled);
    let pool = kernel.db.pool();
    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM hosts h WHERE h.city_id = ANY($1) AND ($2::text IS NULL OR h.id = $2)",
    )
    .bind(&city_codes)
    .bind(host_id)
    .fetch_one(pool)
    .await?;
    let query = format!("{sql} ORDER BY revenue DESC, host_name, host_id LIMIT $5 OFFSET $6");
    let rows = sqlx::query_as::<_, Row>(&query)
        .bind(from)
        .bind(to)
        .bind(host_id)
        .bind(&city_codes)
        .bind(page.limit())
        .bind(page.offset())
        .fetch_all(pool)
        .await?;
    let items = rows
        .into_iter()
        .map(|row| HostFigures { host_id: row.host_id, host_name: row.host_name, city: row.city, figures: row.figures })
        .collect();
    Ok(Page::new(items, total, page))
}
