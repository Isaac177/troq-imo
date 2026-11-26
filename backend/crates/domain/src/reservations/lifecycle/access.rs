use kernel::auth::Actor;
use kernel::permissions::names::RESERVATIONS_WRITE;
use kernel::{Error, Result};

use super::decision_queries::DecisionRow;
use crate::reservations::codes::RESERVATION_NOT_FOUND;

pub fn ensure_manage(actor: &Actor, row: &DecisionRow) -> Result<()> {
    actor.require(RESERVATIONS_WRITE)?;
    actor
        .ensure_host_scope(RESERVATIONS_WRITE, &row.operator_id, &row.property_id)
        .map_err(hide_membership)
}

pub fn hide_membership(error: Error) -> Error {
    match error {
        Error::NotFound(_) => Error::not_found(RESERVATION_NOT_FOUND),
        other => other,
    }
}

pub fn has_permission(actor: &Actor, permission: &str) -> bool {
    actor.access.grants.iter().any(|grant| grant.permission == permission)
}
