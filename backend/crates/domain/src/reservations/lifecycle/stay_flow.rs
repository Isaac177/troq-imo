use chrono::{NaiveDate, Utc};
use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::permissions::names::OPERATIONS_CHECKIN;
use kernel::{Error, Kernel, Result};
use serde_json::json;
use sqlx::{FromRow, PgPool};

use super::no_show::record_charge_and_release;
use super::queries::{move_if_version, release_item_assignments};
use super::rooms;
use super::state::{ensure_move, CHECKED_IN, CHECKED_OUT, CONFIRMED, NO_SHOW};
use super::types::{ReservationView, VersionedAction};
use super::view::require_view;
use crate::reservations::codes::{
    ARRIVAL_NOT_PASSED, ARRIVAL_NOT_REACHED, ASSIGNED_ROOM_UNAVAILABLE, NO_ROOM_ASSIGNED, NO_UNIT_FREE,
    RESERVATION_ITEM_NOT_FOUND, RESERVATION_NOT_FOUND, RESERVATION_VERSION_CONFLICT, STAY_ENDED,
};
use crate::reservations::events::{self, types};
use crate::reservations::holds::release::release_reservation_nights;
use crate::reservations::links::{inventory, properties};
use crate::reservations::support::today_in;

#[derive(Debug, FromRow)]
struct StayRow {
    operator_id: String,
    property_id: String,
    state: String,
    check_in: NaiveDate,
    check_out: NaiveDate,
    timezone: String,
    item_id: String,
    unit_type_id: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StayAction {
    CheckIn,
    CheckOut,
    NoShow,
}

impl StayAction {
    fn audit_name(self) -> &'static str {
        match self {
            Self::CheckIn => "reservation.check_in",
            Self::CheckOut => "reservation.check_out",
            Self::NoShow => "reservation.no_show",
        }
    }

    fn event(self) -> &'static str {
        match self {
            Self::CheckIn => types::CHECKED_IN,
            Self::CheckOut => types::CHECKED_OUT,
            Self::NoShow => types::MARKED_NO_SHOW,
        }
    }

    fn states(self) -> (&'static str, &'static str) {
        match self {
            Self::CheckIn => (CONFIRMED, CHECKED_IN),
            Self::CheckOut => (CHECKED_IN, CHECKED_OUT),
            Self::NoShow => (CONFIRMED, NO_SHOW),
        }
    }
}

async fn find(pool: &PgPool, actor: &Actor, reservation_id: &str) -> Result<Option<StayRow>> {
    let row = sqlx::query_as::<_, StayRow>(
        r#"SELECT r."operatorId" AS operator_id, r."propertyId" AS property_id, r.state, i."checkIn" AS check_in,
                  i."checkOut" AS check_out, p.timezone, i.id AS item_id, i."unitTypeId" AS unit_type_id
           FROM stay_reservations r
           JOIN stay_reservation_items i ON i."reservationId" = r.id
           JOIN stay_properties p ON p.id = r."propertyId"
           WHERE r.id = $1 AND r.city_id = $2"#,
    )
    .bind(reservation_id)
    .bind(actor.city.code())
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn transition(
    kernel: &Kernel,
    actor: &Actor,
    reservation_id: &str,
    input: VersionedAction,
    action: StayAction,
) -> Result<ReservationView> {
    actor.require(OPERATIONS_CHECKIN)?;
    let pool = kernel.db.pool();
    let city = actor.city;
    let row = find(pool, actor, reservation_id)
        .await?
        .ok_or_else(|| Error::not_found(RESERVATION_NOT_FOUND))?;
    actor
        .ensure_host_scope(OPERATIONS_CHECKIN, &row.operator_id, &row.property_id)
        .map_err(super::access::hide_membership)?;
    let (expected, next) = action.states();
    if row.state != expected {
        return Err(super::state::invalid_state(&row.state));
    }
    ensure_move(expected, next)?;
    let today = today_in(&row.timezone, Utc::now())?;
    if action == StayAction::NoShow && row.check_in >= today {
        return Err(Error::conflict(ARRIVAL_NOT_PASSED));
    }
    let mut tx = pool.begin().await?;
    properties::lock_property(&mut tx, &row.property_id).await?;
    let current = inventory::lock_item_context(&mut tx, &row.item_id)
        .await?
        .ok_or_else(|| Error::not_found(RESERVATION_ITEM_NOT_FOUND))?;
    if current.state != row.state
        || current.check_in != row.check_in
        || current.check_out != row.check_out
        || current.unit_type_id != row.unit_type_id
    {
        return Err(Error::conflict(RESERVATION_VERSION_CONFLICT));
    }
    let unit = match action {
        StayAction::CheckIn => Some(pick_unit_for_check_in(&mut tx, &row, reservation_id, today).await?),
        _ => None,
    };
    if !move_if_version(&mut tx, reservation_id, input.expected_version, expected, next).await? {
        return Err(Error::conflict(RESERVATION_VERSION_CONFLICT));
    }
    match (action, &unit) {
        (StayAction::CheckIn, Some(unit_id)) => {
            rooms::assign_at_check_in(&mut tx, city, &row.item_id, unit_id, actor.user_id().as_str()).await?
        }
        (StayAction::CheckOut, _) => rooms::record_departure(&mut tx, &row.item_id).await?,
        (StayAction::NoShow, _) => {
            release_reservation_nights(&mut tx, reservation_id).await?;
            release_item_assignments(&mut tx, reservation_id).await?;
            record_charge_and_release(&mut tx, reservation_id).await?;
        }
        (StayAction::CheckIn, None) => {}
    }
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: action.audit_name(),
            resource_type: "reservation",
            resource_id: reservation_id,
            reason: None,
            change: json!({ "from": expected, "to": next }),
        },
    )
    .await?;
    events::reservation(&mut tx, city, reservation_id, action.event()).await?;
    tx.commit().await?;
    require_view(kernel, city, reservation_id).await
}

async fn pick_unit_for_check_in(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    row: &StayRow,
    reservation_id: &str,
    today: NaiveDate,
) -> Result<String> {
    if row.check_in > today {
        return Err(Error::conflict(ARRIVAL_NOT_REACHED));
    }
    if today >= row.check_out {
        return Err(Error::conflict(STAY_ENDED));
    }
    match rooms::active_unit_for_item(tx, &row.item_id, today).await? {
        Some(unit) => {
            inventory::lock_unit(tx, &unit).await?;
            if !rooms::assigned_unit_is_usable(tx, &row.item_id, reservation_id, &unit, today).await? {
                return Err(Error::conflict(ASSIGNED_ROOM_UNAVAILABLE));
            }
            Ok(unit)
        }
        None if rooms::has_active_assignments(tx, &row.item_id).await? => Err(Error::conflict(NO_ROOM_ASSIGNED)),
        None => rooms::lock_free_unit(tx, &row.unit_type_id, row.check_in, row.check_out, reservation_id)
            .await?
            .ok_or_else(|| Error::conflict(NO_UNIT_FREE)),
    }
}
