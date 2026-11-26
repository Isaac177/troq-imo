use chrono::{DateTime, NaiveDate, Utc};
use kernel::{City, Error, Result};
use serde_json::Value;
use sqlx::{FromRow, PgPool, Postgres, Transaction};

use crate::reservations::codes::AMENDMENT_NOT_PENDING;
use crate::reservations::lifecycle::rooms;

#[derive(Debug, FromRow)]
pub struct ReservationRow {
    pub id: String,
    pub guest_user_id: String,
    pub state: String,
    pub version: i32,
    pub total: String,
    pub property_id: String,
    pub operator_id: String,
    pub item_id: String,
    pub unit_type_id: String,
    pub rate_plan_id: String,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub adult_count: i32,
    pub child_count: i32,
    pub timezone: String,
    pub contract: Value,
}

#[derive(Debug, FromRow)]
pub struct AmendmentRow {
    pub id: String,
    pub reservation_id: String,
    pub state: String,
    pub original_version: i32,
    pub contract: Value,
    pub expires_at: DateTime<Utc>,
}

pub async fn find_reservation(pool: &PgPool, city: City, reservation_id: &str) -> Result<Option<ReservationRow>> {
    let row = sqlx::query_as::<_, ReservationRow>(
        r#"SELECT r.id, r."guestUserId" AS guest_user_id, r.state, r.version, r."totalAmount"::text AS total,
                  r."propertyId" AS property_id, r."operatorId" AS operator_id, i.id AS item_id,
                  i."unitTypeId" AS unit_type_id, i."ratePlanId" AS rate_plan_id, i."checkIn" AS check_in,
                  i."checkOut" AS check_out, i."adultCount" AS adult_count, i."childCount" AS child_count,
                  p.timezone, r."contractSnapshot" AS contract
           FROM stay_reservations r
           JOIN stay_reservation_items i ON i."reservationId" = r.id
           JOIN stay_properties p ON p.id = r."propertyId"
           WHERE r.id = $1 AND r.city_id = $2"#,
    )
    .bind(reservation_id)
    .bind(city.code())
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn find_amendment(pool: &PgPool, city: City, amendment_id: &str) -> Result<Option<AmendmentRow>> {
    let row = sqlx::query_as::<_, AmendmentRow>(
        r#"SELECT id, "reservationId" AS reservation_id, state, "originalVersion" AS original_version,
                  "proposedContract" AS contract, "expiresAt" AS expires_at
           FROM stay_amendments WHERE id = $1 AND city_id = $2"#,
    )
    .bind(amendment_id)
    .bind(city.code())
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn close_stale_amendments(tx: &mut Transaction<'_, Postgres>, reservation_id: &str, version: i32) -> Result<()> {
    sqlx::query(
        r#"UPDATE stay_amendments SET state = CASE WHEN "expiresAt" <= now() THEN 'EXPIRED' ELSE 'SUPERSEDED' END,
               "respondedAt" = now()
           WHERE "reservationId" = $1 AND state = 'PROPOSED'
             AND ("expiresAt" <= now() OR "originalVersion" <> $2)"#,
    )
    .bind(reservation_id)
    .bind(version)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub struct NewAmendment<'a> {
    pub id: &'a str,
    pub city: City,
    pub reservation_id: &'a str,
    pub original_version: i32,
    pub contract: &'a Value,
    pub delta: &'a str,
    pub expires_at: DateTime<Utc>,
}

pub async fn insert_amendment(tx: &mut Transaction<'_, Postgres>, new: &NewAmendment<'_>) -> std::result::Result<(), sqlx::Error> {
    sqlx::query(
        r#"INSERT INTO stay_amendments (id, city_id, "reservationId", "originalVersion", "proposedContract",
               "deltaAmount", state, "expiresAt")
           VALUES ($1, $2, $3, $4, $5, $6::text::numeric, 'PROPOSED', $7)"#,
    )
    .bind(new.id)
    .bind(new.city.code())
    .bind(new.reservation_id)
    .bind(new.original_version)
    .bind(new.contract)
    .bind(new.delta)
    .bind(new.expires_at)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn set_amendment_state(tx: &mut Transaction<'_, Postgres>, amendment_id: &str, state: &str) -> Result<()> {
    let result = sqlx::query(
        r#"UPDATE stay_amendments SET state = $2, "respondedAt" = now()
           WHERE id = $1 AND state = 'PROPOSED' AND "expiresAt" > now()"#,
    )
    .bind(amendment_id)
    .bind(state)
    .execute(&mut **tx)
    .await?;
    if result.rows_affected() != 1 {
        return Err(Error::conflict(AMENDMENT_NOT_PENDING));
    }
    Ok(())
}

pub async fn release_unit_assignment(tx: &mut Transaction<'_, Postgres>, item_id: &str) -> Result<()> {
    crate::reservations::links::inventory::release_for_item(tx, item_id).await?;
    sqlx::query(r#"UPDATE stay_reservation_items SET "physicalUnitId" = NULL WHERE id = $1"#)
        .bind(item_id)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

pub struct NewDates<'a> {
    pub item_id: &'a str,
    pub reservation_id: &'a str,
    pub original_version: i32,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub nights: &'a Value,
    pub subtotal: &'a str,
    pub total: &'a str,
    pub contract_patch: &'a Value,
    pub retain_until: NaiveDate,
}

pub async fn apply_new_dates(tx: &mut Transaction<'_, Postgres>, new: &NewDates<'_>) -> Result<bool> {
    sqlx::query(
        r#"UPDATE stay_reservation_items
           SET "checkIn" = $2, "checkOut" = $3, "nightlyCharges" = $4, "subtotalAmount" = $5::text::numeric
           WHERE id = $1"#,
    )
    .bind(new.item_id)
    .bind(new.check_in)
    .bind(new.check_out)
    .bind(new.nights)
    .bind(new.subtotal)
    .execute(&mut **tx)
    .await?;
    sqlx::query(r#"UPDATE stay_reservation_guests SET "retainUntil" = $2 WHERE "reservationItemId" = $1 AND "anonymizedAt" IS NULL"#)
        .bind(new.item_id)
        .bind(new.retain_until)
        .execute(&mut **tx)
        .await?;
    let result = sqlx::query(
        r#"UPDATE stay_reservations
           SET "totalAmount" = $3::text::numeric,
               "commissionAmount" = TRUNC($3::text::numeric * "commissionRateBps" / 10000, 2),
               "operatorNetAmount" = $3::text::numeric - TRUNC($3::text::numeric * "commissionRateBps" / 10000, 2),
               "contractSnapshot" = "contractSnapshot" || $4::jsonb, version = version + 1, "updatedAt" = now()
           WHERE id = $1 AND version = $2 AND state = 'CONFIRMED'"#,
    )
    .bind(new.reservation_id)
    .bind(new.original_version)
    .bind(new.total)
    .bind(new.contract_patch)
    .execute(&mut **tx)
    .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn reserve_new_room(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    reservation: &ReservationRow,
    check_in: NaiveDate,
    check_out: NaiveDate,
) -> Result<()> {
    rooms::reserve_room(tx, city, &reservation.item_id, &reservation.unit_type_id, &reservation.id, check_in, check_out, "system").await
}
