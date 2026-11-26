use kernel::auth::Actor;
use kernel::permissions::names::RESERVATIONS_READ;
use kernel::{Error, Kernel, Page, Result};

use super::access::can_view;
use super::queries;
use super::rules::CASE_STATES;
use super::types::{CaseListQuery, CaseView, MyCasesQuery};
use crate::reservations::codes::{CASE_NOT_FOUND, CASE_STATE_UNKNOWN, HOST_NOT_FOUND, HOST_REQUIRED};
use crate::reservations::support::paging;

pub fn ensure_case_state(state: &str) -> Result<()> {
    if CASE_STATES.contains(&state) {
        Ok(())
    } else {
        Err(Error::bad_request(CASE_STATE_UNKNOWN).with_params(serde_json::json!({ "state": state })))
    }
}

pub async fn get_case(kernel: &Kernel, actor: &Actor, case_id: &str) -> Result<CaseView> {
    let case = queries::find_case(kernel.db.pool(), actor.city, case_id)
        .await?
        .ok_or_else(|| Error::not_found(CASE_NOT_FOUND))?;
    if !can_view(actor, &case) {
        return Err(Error::not_found(CASE_NOT_FOUND));
    }
    Ok(case)
}

pub async fn list_cases(kernel: &Kernel, actor: &Actor, query: CaseListQuery) -> Result<Page<CaseView>> {
    actor.require(RESERVATIONS_READ)?;
    if query.host_id.trim().is_empty() {
        return Err(Error::bad_request(HOST_REQUIRED));
    }
    if let Some(state) = query.state.as_deref() {
        ensure_case_state(state)?;
    }
    if !actor.access.is_member_of(&query.host_id) {
        return Err(Error::not_found(HOST_NOT_FOUND));
    }
    actor.require_scoped(RESERVATIONS_READ, Some(query.host_id.as_str()), None)?;
    let page = paging(query.page, query.page_size)?;
    let (items, total) = queries::list_for_host(kernel.db.pool(), actor.city, &query.host_id, query.state.as_deref(), page).await?;
    Ok(Page::new(items, total, page))
}

pub async fn my_cases(kernel: &Kernel, actor: &Actor, query: MyCasesQuery) -> Result<Page<CaseView>> {
    let page = paging(query.page, query.page_size)?;
    let (items, total) =
        queries::list_for_guest(kernel.db.pool(), actor.city, actor.user_id().as_str(), query.state.as_deref(), page).await?;
    Ok(Page::new(items, total, page))
}
