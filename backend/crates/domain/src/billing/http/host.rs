use axum::extract::rejection::QueryRejection;
use axum::extract::{Query, State};
use kernel::auth::Actor;
use kernel::envelope::Envelope;
use kernel::error::ErrorBody;
use kernel::{KernelState, Page};
use serde::Deserialize;
use utoipa::IntoParams;

use super::extract::{paging, query, reply, Reply};
use crate::billing::credits::host_view;
use crate::billing::credits::types::{MineLedgerRow, MineSummary};

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct HostCreditsQuery {
    pub host_id: String,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[utoipa::path(
    get, path = "/api/host/credits", operation_id = "host_credits_summary",
    params(HostCreditsQuery),
    responses((status = 200, body = Envelope<MineSummary>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn summary<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
    params: std::result::Result<Query<HostCreditsQuery>, QueryRejection>,
) -> Reply<MineSummary> {
    let params = query(params)?;
    let view = host_view::summary(state.kernel().db.pool(), &actor, &params.host_id).await?;
    reply(view, &actor)
}

#[utoipa::path(
    get, path = "/api/host/credits/ledger", operation_id = "host_credits_ledger",
    params(HostCreditsQuery),
    responses((status = 200, body = Envelope<Page<MineLedgerRow>>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn ledger<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
    params: std::result::Result<Query<HostCreditsQuery>, QueryRejection>,
) -> Reply<Page<MineLedgerRow>> {
    let params = query(params)?;
    let request = paging(params.page, params.page_size);
    let page = host_view::ledger(state.kernel().db.pool(), &actor, &params.host_id, request).await?;
    reply(page, &actor)
}
