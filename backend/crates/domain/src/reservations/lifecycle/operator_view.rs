use chrono::Utc;
use kernel::auth::Actor;
use kernel::permissions::names::{OPERATIONS_CHECKIN, RESERVATIONS_GUEST_DETAILS, RESERVATIONS_WRITE};
use kernel::Result;

use super::operator_queries::OperatorReservationRow;
use super::state::{CHECKED_IN, CONFIRMED, REQUESTED};
use super::types::{GuestView, OperatorReservationView};
use crate::reservations::contract::nights_from;

fn can(actor: &Actor, permission: &str, row: &OperatorReservationRow) -> bool {
    actor
        .require_scoped(permission, Some(row.operator_id.as_str()), Some(row.property_id.as_str()))
        .is_ok()
}

pub fn allowed_actions(actor: &Actor, row: &OperatorReservationRow) -> Vec<String> {
    let today = kernel::timezone::local_date(&row.timezone, Utc::now());
    let manage = can(actor, RESERVATIONS_WRITE, row);
    let mut actions: Vec<&str> = Vec::new();
    if row.state == REQUESTED {
        let open = row.approval_deadline_at.map_or(true, |deadline| deadline > Utc::now());
        if open && manage {
            actions.extend(["ACCEPT", "DECLINE"]);
        }
    }
    if matches!(row.state.as_str(), REQUESTED | CONFIRMED) && row.special_request_state == "REQUESTED" && manage {
        actions.extend(["ACCEPT_SPECIAL_REQUEST", "DECLINE_SPECIAL_REQUEST"]);
    }
    if row.state == CONFIRMED && manage && today.is_some_and(|today| row.check_in > today) {
        actions.push("AMEND");
    }
    if row.state == CONFIRMED && manage {
        actions.push("CANCEL");
    }
    let checkin = can(actor, OPERATIONS_CHECKIN, row);
    match row.state.as_str() {
        CONFIRMED if checkin => {
            if let Some(today) = today {
                if row.check_in <= today && today < row.check_out {
                    actions.push("CHECK_IN");
                }
                if row.check_in < today {
                    actions.push("NO_SHOW");
                }
            }
        }
        CHECKED_IN if checkin => actions.push("CHECK_OUT"),
        _ => {}
    }
    actions.into_iter().map(str::to_string).collect()
}

pub fn build(actor: &Actor, row: OperatorReservationRow, french: bool) -> Result<OperatorReservationView> {
    let allowed_actions = allowed_actions(actor, &row);
    let guest_visible = can(actor, RESERVATIONS_GUEST_DETAILS, &row);
    let nights = nights_from(row.nightly_charges)?;
    let guest = if guest_visible {
        row.full_name.map(|full_name| GuestView {
            full_name,
            contact_phone: row.contact_phone,
            contact_email: row.contact_email,
        })
    } else {
        None
    };
    let (property_name, unit_type_name) = if french {
        (row.property_name_fr, row.unit_type_name_fr)
    } else {
        (row.property_name_en, row.unit_type_name_en)
    };
    Ok(OperatorReservationView {
        id: row.id,
        reference: row.reference,
        state: row.state,
        property_id: row.property_id,
        property_name,
        unit_type_id: row.unit_type_id,
        unit_type_name,
        unit_type_cover_url: None,
        unit_type: None,
        item_id: row.item_id,
        assigned_unit_code: row.assigned_unit_code,
        assignment_id: row.assignment_id,
        assignment_start: row.assignment_start,
        approval_deadline_at: row.approval_deadline_at,
        created_at: row.created_at,
        allowed_actions,
        special_request: row.special_request,
        special_request_state: row.special_request_state,
        check_in: row.check_in,
        check_out: row.check_out,
        adult_count: row.adult_count,
        child_count: row.child_count,
        currency: row.currency,
        total: row.total,
        commission_rate_bps: row.commission_rate_bps,
        commission_amount: row.commission_amount,
        net_amount: row.net_amount,
        version: row.version,
        nights,
        guest,
    })
}
