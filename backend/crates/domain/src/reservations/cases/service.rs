use chrono::{Duration, Utc};
use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::ids::new_id;
use kernel::permissions::names::CASES_WRITE;
use kernel::{Error, Kernel, Money, Result};
use serde_json::json;

use super::access::can_manage;
use super::queries::{self, NewCase};
use super::rules::{
    ensure_claim_amount, ensure_claim_window, ensure_closable, ensure_openable, validate_reason, DISPUTE_WINDOW_DAYS,
};
use super::types::{CaseView, CloseCaseInput, OpenCaseInput};
use crate::reservations::codes::{
    CASE_HOST_MISMATCH, CASE_HOST_REQUIRED, CASE_NOT_FOUND, CASE_RESERVATION_REQUIRED, CASE_VERSION_CONFLICT,
    CLAIM_AMOUNT_REQUIRED, DAMAGE_CLAIM_NEEDS_CHECKOUT, HOST_NOT_FOUND, RESERVATION_NOT_FOUND,
};
use crate::reservations::events::{self, types::{CASE_CLOSED, CASE_OPENED, DAMAGE_CLAIMED}};
use crate::reservations::links::settings;

pub async fn open_case(kernel: &Kernel, actor: &Actor, input: OpenCaseInput) -> Result<CaseView> {
    let pool = kernel.db.pool();
    let city = actor.city;
    let user = actor.user_id().as_str().to_string();
    let reason = validate_reason(&input.reason)?;
    let reservation = match input.reservation_id.as_deref() {
        Some(id) => Some(
            queries::reservation_for_case(pool, city, id)
                .await?
                .ok_or_else(|| Error::not_found(RESERVATION_NOT_FOUND))?,
        ),
        None => None,
    };
    let by_guest = reservation.as_ref().is_some_and(|row| row.guest_user_id == user);

    let host_id = if by_guest {
        ensure_openable(&input.case_type, true)?;
        reservation
            .as_ref()
            .map(|row| row.operator_id.clone())
            .ok_or_else(|| Error::bad_request(CASE_RESERVATION_REQUIRED))?
    } else {
        ensure_openable(&input.case_type, false)?;
        actor.require(CASES_WRITE)?;
        let host = match (&reservation, input.host_id.as_deref()) {
            (Some(row), Some(requested)) if row.operator_id != requested => return Err(Error::bad_request(CASE_HOST_MISMATCH)),
            (Some(row), _) => row.operator_id.clone(),
            (None, Some(requested)) => requested.to_string(),
            (None, None) => return Err(Error::bad_request(CASE_HOST_REQUIRED)),
        };
        if !actor.access.is_member_of(&host) {
            return Err(Error::not_found(HOST_NOT_FOUND));
        }
        actor.require_scoped(CASES_WRITE, Some(host.as_str()), None)?;
        host
    };

    let claim_amount = if input.case_type == "DAMAGE_CLAIM" {
        let row = reservation.as_ref().ok_or_else(|| Error::bad_request(CASE_RESERVATION_REQUIRED))?;
        if row.state != "CHECKED_OUT" {
            return Err(Error::conflict(DAMAGE_CLAIM_NEEDS_CHECKOUT));
        }
        let requested = input.claim_amount.as_deref().ok_or_else(|| Error::bad_request(CLAIM_AMOUNT_REQUIRED))?;
        let claim = Money::parse_input(requested)?;
        ensure_claim_amount(claim.minor(), Money::parse(&row.total_amount)?.minor())?;
        ensure_claim_window(row.check_out, Utc::now().date_naive(), settings::claim_window_days(pool, city).await?)?;
        Some(claim.to_wire())
    } else {
        None
    };

    let case_id = new_id("case");
    let parties = json!({
        "guestUserId": reservation.as_ref().map(|row| row.guest_user_id.clone()),
        "openedBy": if by_guest { "GUEST" } else { "OPERATOR" },
        "openedByUserId": user,
    });
    let deadline = Utc::now() + Duration::days(settings::case_response_days(pool, city).await?);
    let mut tx = pool.begin().await?;
    queries::insert_case(
        &mut tx,
        &NewCase {
            id: &case_id,
            city,
            operator_id: &host_id,
            reservation_id: input.reservation_id.as_deref(),
            case_type: &input.case_type,
            reason: &reason,
            parties: &parties,
            deadline_at: deadline,
        },
    )
    .await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "case.open",
            resource_type: "case",
            resource_id: &case_id,
            reason: Some(&reason),
            change: json!({ "caseType": input.case_type, "operatorId": host_id }),
        },
    )
    .await?;
    events::emit(
        &mut tx,
        city,
        "case",
        &case_id,
        CASE_OPENED,
        json!({ "caseId": case_id, "operatorId": host_id, "caseType": input.case_type }),
    )
    .await?;
    if let (Some(amount), Some(reservation_id)) = (claim_amount.as_deref(), input.reservation_id.as_deref()) {
        queries::set_claim_amount(&mut tx, &case_id, amount).await?;
        events::reservation_with(
            &mut tx,
            city,
            reservation_id,
            DAMAGE_CLAIMED,
            json!({
                "caseId": case_id,
                "claimAmount": amount,
                "claimReason": reason,
                "disputeDeadline": (Utc::now() + Duration::days(DISPUTE_WINDOW_DAYS)).to_rfc3339(),
            }),
        )
        .await?;
    }
    tx.commit().await?;
    queries::find_case(pool, city, &case_id).await?.ok_or_else(|| Error::internal("opened case not readable"))
}

pub async fn close_case(kernel: &Kernel, actor: &Actor, case_id: &str, input: CloseCaseInput) -> Result<CaseView> {
    let pool = kernel.db.pool();
    let city = actor.city;
    let resolution = validate_reason(&input.resolution)?;
    let case = queries::find_case(pool, city, case_id).await?.ok_or_else(|| Error::not_found(CASE_NOT_FOUND))?;
    if !can_manage(actor, &case) {
        return Err(Error::not_found(CASE_NOT_FOUND));
    }
    ensure_closable(&case.state)?;
    let mut tx = pool.begin().await?;
    if !queries::close_if_version(&mut tx, &case.id, input.expected_version, &resolution).await? {
        return Err(Error::conflict(CASE_VERSION_CONFLICT));
    }
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "case.close",
            resource_type: "case",
            resource_id: &case.id,
            reason: Some(&resolution),
            change: json!({ "from": case.state, "to": "CLOSED" }),
        },
    )
    .await?;
    events::emit(&mut tx, city, "case", &case.id, CASE_CLOSED, json!({ "caseId": case.id, "operatorId": case.operator_id })).await?;
    tx.commit().await?;
    queries::find_case(pool, city, &case.id).await?.ok_or_else(|| Error::internal("closed case not readable"))
}
