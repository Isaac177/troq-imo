use chrono::{DateTime, NaiveDate, Utc};
use kernel::{City, PageRequest, Result};
use serde_json::Value;
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder, Transaction};

use super::types::CaseView;

const CASE_COLUMNS: &str = r#"id, "operatorId" AS operator_id, "reservationId" AS reservation_id,
    "caseType" AS case_type, reason, parties, state, "deadlineAt" AS deadline_at, resolution,
    version, "createdAt" AS created_at, "closedAt" AS closed_at,
    (SELECT r.reference FROM stay_reservations r WHERE r.id = stay_cases."reservationId") AS reservation_reference"#;

#[derive(Debug, FromRow)]
pub struct ReservationForCase {
    pub operator_id: String,
    pub property_id: String,
    pub guest_user_id: String,
    pub state: String,
    pub total_amount: String,
    pub check_out: NaiveDate,
}

pub async fn reservation_for_case(pool: &PgPool, city: City, reservation_id: &str) -> Result<Option<ReservationForCase>> {
    let row = sqlx::query_as::<_, ReservationForCase>(
        r#"SELECT r."operatorId" AS operator_id, r."propertyId" AS property_id, r."guestUserId" AS guest_user_id,
                  r.state, r."totalAmount"::text AS total_amount, i."checkOut" AS check_out
           FROM stay_reservations r
           JOIN stay_reservation_items i ON i."reservationId" = r.id
           WHERE r.id = $1 AND r.city_id = $2
           ORDER BY i."checkOut" DESC
           LIMIT 1"#,
    )
    .bind(reservation_id)
    .bind(city.code())
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub struct NewCase<'a> {
    pub id: &'a str,
    pub city: City,
    pub operator_id: &'a str,
    pub reservation_id: Option<&'a str>,
    pub case_type: &'a str,
    pub reason: &'a str,
    pub parties: &'a Value,
    pub deadline_at: DateTime<Utc>,
}

pub async fn insert_case(tx: &mut Transaction<'_, Postgres>, new: &NewCase<'_>) -> Result<()> {
    sqlx::query(
        r#"INSERT INTO stay_cases (id, city_id, "operatorId", "reservationId", "caseType", reason, parties, state, "deadlineAt")
           VALUES ($1, $2, $3, $4, $5, $6, $7, 'OPEN', $8)"#,
    )
    .bind(new.id)
    .bind(new.city.code())
    .bind(new.operator_id)
    .bind(new.reservation_id)
    .bind(new.case_type)
    .bind(new.reason)
    .bind(new.parties)
    .bind(new.deadline_at)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn find_case(pool: &PgPool, city: City, case_id: &str) -> Result<Option<CaseView>> {
    let sql = format!("SELECT {CASE_COLUMNS} FROM stay_cases WHERE id = $1 AND city_id = $2");
    let row = sqlx::query_as::<_, CaseView>(&sql).bind(case_id).bind(city.code()).fetch_optional(pool).await?;
    Ok(row)
}

fn push_host_filters(builder: &mut QueryBuilder<'_, Postgres>, city: City, host_id: &str, state: Option<&str>) {
    builder.push(" AND city_id = ").push_bind(city.code());
    builder.push(r#" AND "operatorId" = "#).push_bind(host_id.to_string());
    if let Some(state) = state {
        builder.push(" AND state = ").push_bind(state.to_string());
    }
}

pub async fn list_for_host(
    pool: &PgPool,
    city: City,
    host_id: &str,
    state: Option<&str>,
    page: PageRequest,
) -> Result<(Vec<CaseView>, i64)> {
    let mut count = QueryBuilder::<Postgres>::new("SELECT COUNT(*) FROM stay_cases WHERE TRUE");
    push_host_filters(&mut count, city, host_id, state);
    let total: i64 = count.build_query_scalar().fetch_one(pool).await?;
    let mut items = QueryBuilder::<Postgres>::new(format!("SELECT {CASE_COLUMNS} FROM stay_cases WHERE TRUE"));
    push_host_filters(&mut items, city, host_id, state);
    items
        .push(r#" ORDER BY "createdAt" DESC, id LIMIT "#)
        .push_bind(page.limit())
        .push(" OFFSET ")
        .push_bind(page.offset());
    let rows = items.build_query_as::<CaseView>().fetch_all(pool).await?;
    Ok((rows, total))
}

pub async fn list_for_guest(
    pool: &PgPool,
    city: City,
    guest_user_id: &str,
    state: Option<&str>,
    page: PageRequest,
) -> Result<(Vec<CaseView>, i64)> {
    let total: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM stay_cases
           WHERE city_id = $3 AND parties->>'guestUserId' = $1 AND ($2::text IS NULL OR state = $2)"#,
    )
    .bind(guest_user_id)
    .bind(state)
    .bind(city.code())
    .fetch_one(pool)
    .await?;
    let sql = format!(
        r#"SELECT {CASE_COLUMNS} FROM stay_cases
           WHERE city_id = $5 AND parties->>'guestUserId' = $1 AND ($4::text IS NULL OR state = $4)
           ORDER BY "createdAt" DESC, id LIMIT $2 OFFSET $3"#
    );
    let rows = sqlx::query_as::<_, CaseView>(&sql)
        .bind(guest_user_id)
        .bind(page.limit())
        .bind(page.offset())
        .bind(state)
        .bind(city.code())
        .fetch_all(pool)
        .await?;
    Ok((rows, total))
}

pub async fn close_if_version(tx: &mut Transaction<'_, Postgres>, case_id: &str, expected_version: i32, resolution: &str) -> Result<bool> {
    let result = sqlx::query(
        r#"UPDATE stay_cases
           SET state = 'CLOSED', resolution = to_jsonb($3::text), version = version + 1, "closedAt" = now(), "updatedAt" = now()
           WHERE id = $1 AND version = $2 AND state IN ('OPEN', 'AWAITING_RESPONSE')"#,
    )
    .bind(case_id)
    .bind(expected_version)
    .bind(resolution)
    .execute(&mut **tx)
    .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn set_claim_amount(tx: &mut Transaction<'_, Postgres>, case_id: &str, amount: &str) -> Result<()> {
    sqlx::query(r#"UPDATE stay_cases SET "claimAmount" = $2::numeric WHERE id = $1"#)
        .bind(case_id)
        .bind(amount)
        .execute(&mut **tx)
        .await?;
    Ok(())
}
