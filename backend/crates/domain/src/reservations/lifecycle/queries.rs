use chrono::{DateTime, NaiveDate, Utc};
use kernel::{City, Result};
use serde_json::Value;
use sqlx::{PgPool, Postgres, Transaction};

use crate::reservations::contract::CommissionSnap;

pub struct NewReservation<'a> {
    pub id: &'a str,
    pub reference: &'a str,
    pub city: City,
    pub property_id: &'a str,
    pub operator_id: &'a str,
    pub guest_user_id: &'a str,
    pub quote_id: &'a str,
    pub total: &'a str,
    pub commission: &'a CommissionSnap,
    pub contract: &'a Value,
    pub state: &'a str,
    pub booking_mode: &'a str,
    pub approval_deadline: Option<DateTime<Utc>>,
}

pub async fn insert_reservation(tx: &mut Transaction<'_, Postgres>, new: &NewReservation<'_>) -> std::result::Result<(), sqlx::Error> {
    sqlx::query(
        r#"INSERT INTO stay_reservations (id, reference, city_id, "propertyId", "operatorId", "guestUserId",
               state, source, "bookingMode", "quoteId", currency, "totalAmount", "contractSnapshot",
               "approvalDeadlineAt", "commissionRateBps", "commissionAmount", "operatorNetAmount")
           VALUES ($1, $2, $3, $4, $5, $6, $7, 'DIRECT', $8, $9, 'CDF', $10::text::numeric, $11, $12,
               $13, $14::text::numeric, $15::text::numeric)"#,
    )
    .bind(new.id)
    .bind(new.reference)
    .bind(new.city.code())
    .bind(new.property_id)
    .bind(new.operator_id)
    .bind(new.guest_user_id)
    .bind(new.state)
    .bind(new.booking_mode)
    .bind(new.quote_id)
    .bind(new.total)
    .bind(new.contract)
    .bind(new.approval_deadline)
    .bind(new.commission.rate_bps)
    .bind(&new.commission.amount)
    .bind(&new.commission.net)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub struct NewItem<'a> {
    pub id: &'a str,
    pub reservation_id: &'a str,
    pub property_id: &'a str,
    pub unit_type_id: &'a str,
    pub rate_plan_id: &'a str,
    pub adults: i32,
    pub children: i32,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub nights: &'a Value,
    pub subtotal: &'a str,
}

pub async fn insert_item(tx: &mut Transaction<'_, Postgres>, new: &NewItem<'_>) -> Result<()> {
    sqlx::query(
        r#"INSERT INTO stay_reservation_items (id, "reservationId", "propertyId", "unitTypeId", "ratePlanId",
               quantity, "adultCount", "childCount", "checkIn", "checkOut", "nightlyCharges", "subtotalAmount")
           VALUES ($1, $2, $3, $4, $5, 1, $6, $7, $8, $9, $10, $11::text::numeric)"#,
    )
    .bind(new.id)
    .bind(new.reservation_id)
    .bind(new.property_id)
    .bind(new.unit_type_id)
    .bind(new.rate_plan_id)
    .bind(new.adults)
    .bind(new.children)
    .bind(new.check_in)
    .bind(new.check_out)
    .bind(new.nights)
    .bind(new.subtotal)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn insert_primary_guest(
    tx: &mut Transaction<'_, Postgres>,
    guest_id: &str,
    item_id: &str,
    full_name: &str,
    phone: Option<&str>,
    email: Option<&str>,
    retain_until: NaiveDate,
) -> Result<()> {
    sqlx::query(
        r#"INSERT INTO stay_reservation_guests (id, "reservationItemId", "fullName", "contactPhone",
               "contactEmail", role, "consentBasis", "retainUntil")
           VALUES ($1, $2, $3, $4, $5, 'PRIMARY', 'CONTRACT', $6)"#,
    )
    .bind(guest_id)
    .bind(item_id)
    .bind(full_name)
    .bind(phone)
    .bind(email)
    .bind(retain_until)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn reservation_owner(pool: &PgPool, city: City, reservation_id: &str) -> Result<Option<String>> {
    let owner = sqlx::query_scalar::<_, String>(
        r#"SELECT "guestUserId" FROM stay_reservations WHERE id = $1 AND city_id = $2"#,
    )
    .bind(reservation_id)
    .bind(city.code())
    .fetch_optional(pool)
    .await?;
    Ok(owner)
}

pub async fn set_special_request(tx: &mut Transaction<'_, Postgres>, reservation_id: &str, text: &str) -> Result<()> {
    sqlx::query(
        r#"UPDATE stay_reservations SET "specialRequest" = $2, "specialRequestState" = 'REQUESTED' WHERE id = $1"#,
    )
    .bind(reservation_id)
    .bind(text)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn move_if_version(
    tx: &mut Transaction<'_, Postgres>,
    reservation_id: &str,
    expected_version: i32,
    from: &str,
    to: &str,
) -> Result<bool> {
    let result = sqlx::query(
        r#"UPDATE stay_reservations SET state = $3, version = version + 1, "updatedAt" = now()
           WHERE id = $1 AND version = $2 AND state = $4"#,
    )
    .bind(reservation_id)
    .bind(expected_version)
    .bind(to)
    .bind(from)
    .execute(&mut **tx)
    .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn cancel_confirmed(
    tx: &mut Transaction<'_, Postgres>,
    reservation_id: &str,
    expected_version: i32,
    at: DateTime<Utc>,
    reason: &str,
    charge: &str,
) -> Result<bool> {
    let result = sqlx::query(
        r#"UPDATE stay_reservations
           SET state = 'CANCELLED', version = version + 1, "updatedAt" = now(),
               "cancelledAt" = $3, "cancellationReason" = $4, "cancellationChargeAmount" = $5::text::numeric
           WHERE id = $1 AND version = $2 AND state = 'CONFIRMED'"#,
    )
    .bind(reservation_id)
    .bind(expected_version)
    .bind(at)
    .bind(reason)
    .bind(charge)
    .execute(&mut **tx)
    .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn release_item_assignments(tx: &mut Transaction<'_, Postgres>, reservation_id: &str) -> Result<()> {
    crate::reservations::links::inventory::release_for_reservation(tx, reservation_id).await
}
