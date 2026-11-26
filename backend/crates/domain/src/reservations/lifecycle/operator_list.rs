use kernel::auth::Actor;
use kernel::permissions::names::{ANALYTICS_EXPORT, RESERVATIONS_READ};
use kernel::{Error, Kernel, Page, Result};

use super::access::hide_membership;
use super::operator_queries::{self, ListFilter};
use super::operator_view::build;
use super::state::ensure_known;
use super::types::{OperatorReservationQuery, OperatorReservationView};
use crate::reservations::codes::{HOST_NOT_FOUND, HOST_REQUIRED, RESERVATION_NOT_FOUND};
use crate::reservations::links::properties::{covers_for_unit_types, unit_type_summary};
use crate::reservations::support::paging;

pub async fn list_operator_reservations(kernel: &Kernel, actor: &Actor, query: OperatorReservationQuery) -> Result<Page<OperatorReservationView>> {
    list_scoped(kernel, actor, query, RESERVATIONS_READ).await
}

pub async fn export_operator_reservations(kernel: &Kernel, actor: &Actor, query: OperatorReservationQuery) -> Result<Page<OperatorReservationView>> {
    list_scoped(kernel, actor, query, ANALYTICS_EXPORT).await
}

async fn list_scoped(kernel: &Kernel, actor: &Actor, query: OperatorReservationQuery, permission: &str) -> Result<Page<OperatorReservationView>> {
    actor.require(permission)?;
    if query.host_id.trim().is_empty() {
        return Err(Error::bad_request(HOST_REQUIRED));
    }
    if let Some(state) = query.state.as_deref() {
        ensure_known(state)?;
    }
    if !actor.access.is_member_of(&query.host_id) {
        return Err(Error::not_found(HOST_NOT_FOUND));
    }
    actor.require_scoped(permission, Some(query.host_id.as_str()), query.property_id.as_deref())?;
    let page = paging(query.page, query.page_size)?;
    let search = query.search.as_deref().map(str::trim).filter(|value| !value.is_empty());
    let pool = kernel.db.pool();
    let filter = ListFilter {
        city: actor.city,
        host_id: &query.host_id,
        property_id: query.property_id.as_deref(),
        state: query.state.as_deref(),
        search,
    };
    let (rows, total) = operator_queries::list(pool, &filter, page).await?;
    let french = query.locale.as_deref() == Some("fr");
    let mut items = rows.into_iter().map(|row| build(actor, row, french)).collect::<Result<Vec<_>>>()?;
    attach_covers(kernel, &mut items).await?;
    Ok(Page::new(items, total, page))
}

pub async fn get_operator_reservation(kernel: &Kernel, actor: &Actor, reservation_id: &str, locale: Option<&str>) -> Result<OperatorReservationView> {
    actor.require(RESERVATIONS_READ)?;
    let pool = kernel.db.pool();
    let row = operator_queries::find(pool, actor.city, reservation_id)
        .await?
        .ok_or_else(|| Error::not_found(RESERVATION_NOT_FOUND))?;
    actor
        .ensure_host_scope(RESERVATIONS_READ, &row.operator_id, &row.property_id)
        .map_err(hide_membership)?;
    let mut item = build(actor, row, locale == Some("fr"))?;
    let summary = unit_type_summary(kernel, &item.unit_type_id).await?;
    item.unit_type_cover_url = summary
        .as_ref()
        .and_then(|summary| summary.images.first())
        .map(|image| image.url.clone());
    item.unit_type = summary;
    Ok(item)
}

async fn attach_covers(kernel: &Kernel, items: &mut [OperatorReservationView]) -> Result<()> {
    let mut ids: Vec<String> = items.iter().map(|item| item.unit_type_id.clone()).collect();
    ids.sort();
    ids.dedup();
    let covers = covers_for_unit_types(kernel, &ids).await?;
    for item in items.iter_mut() {
        item.unit_type_cover_url = covers.get(&item.unit_type_id).cloned();
    }
    Ok(())
}
