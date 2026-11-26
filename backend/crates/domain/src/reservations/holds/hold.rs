use chrono::Utc;
use kernel::audit::{self, AuditEntry};
use kernel::ids::new_id;
use kernel::{Error, Kernel, Result};
use kernel::auth::Actor;
use serde_json::json;

use super::preflight::{prepare, Prepared};
use crate::reservations::codes::{NIGHT_TAKEN, QUOTE_ALREADY_BOOKED};
use crate::reservations::lifecycle::queries::{self, NewItem, NewReservation};
use crate::reservations::lifecycle::rooms::reserve_room;
use crate::reservations::lifecycle::types::{HoldRequest, ReservationView};
use crate::reservations::lifecycle::view::require_view;
use crate::reservations::events::{self, types::HELD};
use crate::reservations::links::inventory::{insert_allocation, take_night};
use crate::reservations::links::{billing, pricing, properties};
use crate::reservations::support::{new_reference, retain_until, trimmed, unique_violation};

pub async fn create_hold(kernel: &Kernel, actor: &Actor, input: HoldRequest) -> Result<ReservationView> {
    let pool = kernel.db.pool();
    let city = actor.city;
    let session = actor.user_id().as_str();
    let prepared = prepare(pool, city, session, &input).await?;
    let Prepared { quote, stored, context, accepted_policies, special_request, nights, initial, approval, commission } = prepared;

    let (reference_type, allocation_expiry) = match approval {
        Some(deadline) => ("HOLD", Some(deadline)),
        None => ("RESERVATION", None),
    };
    let reservation_id = new_id("res");
    let reference = new_reference(city);
    let nights_json = serde_json::to_value(&stored.nights).map_err(|error| Error::internal(error.to_string()))?;
    let contract = json!({
        "quoteId": quote.id,
        "commission": commission,
        "nights": nights_json,
        "policies": stored.policies,
        "cancellationDeadline": stored.cancellation_deadline,
        "discount": stored.discount,
        "acceptedPolicyVersionIds": accepted_policies,
        "acceptedAt": Utc::now(),
    });

    let mut tx = pool.begin().await?;
    properties::lock_property(&mut tx, &quote.property_id).await?;
    if let Some(discount) = stored.discount.as_ref() {
        pricing::lock_budget(&mut tx, &discount.promotion_id).await?;
    }
    let inserted = queries::insert_reservation(
        &mut tx,
        &NewReservation {
            id: &reservation_id,
            reference: &reference,
            city,
            property_id: &quote.property_id,
            operator_id: &context.host_id,
            guest_user_id: session,
            quote_id: &quote.id,
            total: &quote.total_amount,
            commission: &commission,
            contract: &contract,
            state: initial,
            booking_mode: &context.booking_mode,
            approval_deadline: approval,
        },
    )
    .await;
    if let Err(error) = inserted {
        if unique_violation(&error, "stay_reservations_quote_unique_idx") {
            return Err(Error::conflict(QUOTE_ALREADY_BOOKED));
        }
        return Err(error.into());
    }
    if let Some(text) = special_request.as_deref() {
        queries::set_special_request(&mut tx, &reservation_id, text).await?;
    }

    let item_id = new_id("item");
    queries::insert_item(
        &mut tx,
        &NewItem {
            id: &item_id,
            reservation_id: &reservation_id,
            property_id: &quote.property_id,
            unit_type_id: &stored.unit_type_id,
            rate_plan_id: &stored.rate_plan_id,
            adults: quote.adult_count,
            children: quote.child_count,
            check_in: quote.check_in,
            check_out: quote.check_out,
            nights: &nights_json,
            subtotal: &quote.total_amount,
        },
    )
    .await?;
    queries::insert_primary_guest(
        &mut tx,
        &new_id("guest"),
        &item_id,
        input.guest.full_name.trim(),
        trimmed(&input.guest.contact_phone).as_deref(),
        trimmed(&input.guest.contact_email).as_deref(),
        retain_until(quote.check_out, kernel.config.guest_retention_days),
    )
    .await?;

    for night in &nights {
        let pool_id = take_night(&mut tx, &stored.unit_type_id, *night)
            .await?
            .ok_or_else(|| Error::conflict(NIGHT_TAKEN))?;
        insert_allocation(&mut tx, city, &pool_id, reference_type, &reservation_id, *night, allocation_expiry).await?;
    }

    reserve_room(&mut tx, city, &item_id, &stored.unit_type_id, &reservation_id, quote.check_in, quote.check_out, "system").await?;
    if initial == "CONFIRMED" {
        billing::take_commission(&mut tx, &reservation_id).await?;
    }

    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "reservation.hold",
            resource_type: "reservation",
            resource_id: &reservation_id,
            reason: None,
            change: json!({ "quoteId": quote.id, "total": quote.total_amount, "city": city.code(), "state": initial }),
        },
    )
    .await?;
    events::reservation(&mut tx, city, &reservation_id, HELD).await?;
    tx.commit().await?;
    require_view(kernel, city, &reservation_id).await
}
