use axum::extract::rejection::QueryRejection;
use axum::extract::{Query, State};
use axum::Json;
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{codes, Error, ErrorBody, Result};
use kernel::KernelState;

use super::dashboard;
use super::dashboard_hosts;
use super::dashboard_types::{DashboardHostQuery, DashboardQuery, DashboardFigures, DashboardSummary};
use super::dashboard_hosts::HostFigures;
use kernel::paging::PageRequest;
use kernel::Page;

#[utoipa::path(
    get,
    path = "/api/admin/dashboard",
    operation_id = "admin_dashboard_summary",
    params(DashboardQuery),
    responses(
        (status = 200, body = Envelope<DashboardSummary>),
        (status = 400, body = ErrorBody),
        (status = 403, body = ErrorBody)
    )
)]
pub async fn summary<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    query: std::result::Result<Query<DashboardQuery>, QueryRejection>,
) -> Result<Json<Envelope<DashboardSummary>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let Query(query) = query.map_err(|_| Error::bad_request(codes::MALFORMED_REQUEST))?;
    let result = dashboard::summary(state.kernel(), &actor, query).await?;
    Ok(Json(Envelope::ok(result, request_id)))
}

#[utoipa::path(
    get, path = "/api/admin/dashboard/hosts", operation_id = "admin_dashboard_hosts",
    params(DashboardHostQuery),
    responses((status = 200, body = Envelope<Page<HostFigures>>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody))
)]
pub async fn hosts<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    query: std::result::Result<Query<DashboardHostQuery>, QueryRejection>,
) -> Result<Json<Envelope<Page<HostFigures>>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let Query(query) = query.map_err(|_| Error::bad_request(codes::MALFORMED_REQUEST))?;
    let today = chrono::Utc::now().date_naive();
    let to = query.to.unwrap_or(today);
    let from = query.from.unwrap_or(to - chrono::Duration::days(29));
    if to < from || (to - from).num_days() >= 366 {
        return Err(Error::bad_request(codes::MALFORMED_REQUEST));
    }
    let page = PageRequest { page: query.page.unwrap_or(1), page_size: query.page_size.unwrap_or(20) };
    let host_id = query.host_id.as_deref().map(str::trim).filter(|value| !value.is_empty());
    let result = dashboard_hosts::list_page(state.kernel(), &actor, from, to, host_id, page).await?;
    Ok(Json(Envelope::ok(result, request_id)))
}

#[utoipa::path(
    get, path = "/api/host/dashboard", operation_id = "host_dashboard_summary",
    params(("hostId" = String, Query), ("from" = Option<String>, Query), ("to" = Option<String>, Query)),
    responses((status = 200, body = Envelope<DashboardFigures>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn host_summary<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    query: std::result::Result<Query<HostDashboardQuery>, QueryRejection>,
) -> Result<Json<Envelope<DashboardFigures>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let Query(query) = query.map_err(|_| Error::bad_request(codes::MALFORMED_REQUEST))?;
    let result = dashboard::host_summary(
        state.kernel(),
        &actor,
        &query.host_id,
        DashboardQuery { from: query.from, to: query.to, host_id: None },
    )
    .await?;
    Ok(Json(Envelope::ok(result, request_id)))
}

#[derive(Debug, serde::Deserialize, utoipa::IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct HostDashboardQuery {
    pub host_id: String,
    pub from: Option<chrono::NaiveDate>,
    pub to: Option<chrono::NaiveDate>,
}
