use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::extract::{Json, Path, Query, State};
use kernel::auth::Actor;
use kernel::envelope::Envelope;
use kernel::error::ErrorBody;
use kernel::{KernelState, Page};
use serde::Deserialize;
use utoipa::IntoParams;

use super::extract::{body, paging, query, reply, Reply};
use crate::billing::admin::rate_types::{CurrentRate, RateInput, RateRow, RateSet};
use crate::billing::admin::rates;

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct PagingQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[utoipa::path(
    get, path = "/api/admin/commission/hosts/{hostId}/rate", operation_id = "admin_commission_rate_current",
    params(("hostId" = String, Path)),
    responses((status = 200, body = Envelope<CurrentRate>), (status = 403, body = ErrorBody))
)]
pub async fn current<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
    Path(host_id): Path<String>,
) -> Reply<CurrentRate> {
    let view = rates::current(state.kernel().db.pool(), &actor, &host_id).await?;
    reply(view, &actor)
}

#[utoipa::path(
    get, path = "/api/admin/commission/hosts/{hostId}/rate/history", operation_id = "admin_commission_rate_history",
    params(("hostId" = String, Path), PagingQuery),
    responses((status = 200, body = Envelope<Page<RateRow>>), (status = 403, body = ErrorBody))
)]
pub async fn history<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
    Path(host_id): Path<String>,
    params: std::result::Result<Query<PagingQuery>, QueryRejection>,
) -> Reply<Page<RateRow>> {
    let params = query(params)?;
    let page = rates::history(state.kernel().db.pool(), &actor, &host_id, paging(params.page, params.page_size)).await?;
    reply(page, &actor)
}

#[utoipa::path(
    post, path = "/api/admin/commission/hosts/{hostId}/rate", operation_id = "admin_commission_rate_set",
    params(("hostId" = String, Path)), request_body = RateInput,
    responses((status = 200, body = Envelope<RateSet>), (status = 400, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn set<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
    Path(host_id): Path<String>,
    input: std::result::Result<Json<RateInput>, JsonRejection>,
) -> Reply<RateSet> {
    let input = body(input)?;
    let view = rates::set(state.kernel().db.pool(), &actor, &host_id, input).await?;
    reply(view, &actor)
}
