use chrono::{DateTime, NaiveDate, Utc};
use kernel::{City, Error, Kernel, Result};
use serde_json::Value;
use sqlx::FromRow;

use super::types::{GuestView, ReservationView};
use crate::reservations::contract::nights_from;
use crate::reservations::links::properties::covers_for_unit_types;

#[derive(Debug, FromRow)]
struct ReservationRow {
    id: String,
    reference: String,
    state: String,
    property_id: String,
    total: String,
    currency: String,
    approval_deadline_at: Option<DateTime<Utc>>,
    version: i32,
    special_request: Option<String>,
    special_request_state: String,
}

#[derive(Debug, FromRow)]
struct ItemRow {
    unit_type_id: String,
    rate_plan_id: String,
    check_in: NaiveDate,
    check_out: NaiveDate,
    adult_count: i32,
    child_count: i32,
    nightly_charges: Value,
    full_name: String,
    contact_phone: Option<String>,
    contact_email: Option<String>,
}

pub async fn view(kernel: &Kernel, city: City, reservation_id: &str) -> Result<Option<ReservationView>> {
    let pool = kernel.db.pool();
    let reservation = sqlx::query_as::<_, ReservationRow>(
        r#"SELECT id, reference, state, "propertyId" AS property_id, "totalAmount"::text AS total, currency,
                  "approvalDeadlineAt" AS approval_deadline_at, version,
                  "specialRequest" AS special_request, "specialRequestState" AS special_request_state
           FROM stay_reservations WHERE id = $1 AND city_id = $2"#,
    )
    .bind(reservation_id)
    .bind(city.code())
    .fetch_optional(pool)
    .await?;
    let Some(reservation) = reservation else {
        return Ok(None);
    };
    let item = sqlx::query_as::<_, ItemRow>(
        r#"SELECT i."unitTypeId" AS unit_type_id, i."ratePlanId" AS rate_plan_id, i."checkIn" AS check_in,
                  i."checkOut" AS check_out, i."adultCount" AS adult_count, i."childCount" AS child_count,
                  i."nightlyCharges" AS nightly_charges, g."fullName" AS full_name,
                  g."contactPhone" AS contact_phone, g."contactEmail" AS contact_email
           FROM stay_reservation_items i
           JOIN stay_reservation_guests g ON g."reservationItemId" = i.id AND g.role = 'PRIMARY'
           WHERE i."reservationId" = $1"#,
    )
    .bind(reservation_id)
    .fetch_one(pool)
    .await?;
    let hold_expires_at = sqlx::query_scalar::<_, Option<DateTime<Utc>>>(
        r#"SELECT MIN("expiresAt") FROM stay_inventory_allocations
           WHERE "referenceType" = 'HOLD' AND "referenceId" = $1 AND state = 'ACTIVE'"#,
    )
    .bind(reservation_id)
    .fetch_one(pool)
    .await?;
    let nights = nights_from(item.nightly_charges)?;
    let unit_type_cover_url = covers_for_unit_types(kernel, std::slice::from_ref(&item.unit_type_id))
        .await?
        .remove(&item.unit_type_id);
    let payment_required = reservation.state == super::state::PENDING_PAYMENT;
    Ok(Some(ReservationView {
        id: reservation.id,
        reference: reservation.reference,
        state: reservation.state,
        property_id: reservation.property_id,
        unit_type_cover_url,
        unit_type_id: item.unit_type_id,
        rate_plan_id: item.rate_plan_id,
        check_in: item.check_in,
        check_out: item.check_out,
        adult_count: item.adult_count,
        child_count: item.child_count,
        currency: reservation.currency,
        total: reservation.total,
        nights,
        guest: GuestView {
            full_name: item.full_name,
            contact_phone: item.contact_phone,
            contact_email: item.contact_email,
        },
        hold_expires_at,
        payment_required,
        approval_deadline_at: reservation.approval_deadline_at,
        version: reservation.version,
        special_request: reservation.special_request,
        special_request_state: reservation.special_request_state,
    }))
}

pub async fn require_view(kernel: &Kernel, city: City, reservation_id: &str) -> Result<ReservationView> {
    view(kernel, city, reservation_id)
        .await?
        .ok_or_else(|| Error::not_found(crate::reservations::codes::RESERVATION_NOT_FOUND))
}
