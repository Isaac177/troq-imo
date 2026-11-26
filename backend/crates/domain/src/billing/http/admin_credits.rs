use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::extract::{Json, Path, Query, State};
use kernel::auth::Actor;
use kernel::envelope::Envelope;
use kernel::error::ErrorBody;
use kernel::{KernelState, Page};
use serde::Deserialize;
use utoipa::IntoParams;

use super::extract::{body, paging, query, reply, Reply};
use crate::billing::admin::credit_types::{
    AccountFilter, AccountRow, AdjustInput, AdminLedgerRow, BalanceView, CreditDetail, CreditSummary, TopupInput,
};
use crate::billing::admin::{accounts, credit_writes, ledger};

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct AccountsQuery {
    pub host_id: Option<String>,
    pub status: Option<String>,
    pub search: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct SummaryQuery {
    pub host_id: Option<String>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct LedgerQuery {
    #[serde(rename = "type")]
    pub entry_type: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[utoipa::path(
    get, path = "/api/admin/credits/accounts", operation_id = "admin_credit_accounts_list",
    params(AccountsQuery),
    responses((status = 200, body = Envelope<Page<AccountRow>>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody))
)]
pub async fn list<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
    params: std::result::Result<Query<AccountsQuery>, QueryRejection>,
) -> Reply<Page<AccountRow>> {
    let params = query(params)?;
    let filter = AccountFilter { host_id: params.host_id, status: params.status, search: params.search };
    let page = accounts::list(state.kernel().db.pool(), &actor, filter, paging(params.page, params.page_size)).await?;
    reply(page, &actor)
}

#[utoipa::path(
    get, path = "/api/admin/credits/summary", operation_id = "admin_credit_summary",
    params(SummaryQuery),
    responses((status = 200, body = Envelope<CreditSummary>), (status = 403, body = ErrorBody))
)]
pub async fn summary<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
    params: std::result::Result<Query<SummaryQuery>, QueryRejection>,
) -> Reply<CreditSummary> {
    let params = query(params)?;
    let view = accounts::summary(state.kernel().db.pool(), &actor, params.host_id).await?;
    reply(view, &actor)
}

#[utoipa::path(
    get, path = "/api/admin/credits/hosts/{hostId}", operation_id = "admin_credit_detail",
    params(("hostId" = String, Path)),
    responses((status = 200, body = Envelope<CreditDetail>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn detail<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
    Path(host_id): Path<String>,
) -> Reply<CreditDetail> {
    let view = accounts::detail(state.kernel().db.pool(), &actor, &host_id).await?;
    reply(view, &actor)
}

#[utoipa::path(
    get, path = "/api/admin/credits/hosts/{hostId}/ledger", operation_id = "admin_credit_ledger",
    params(("hostId" = String, Path), LedgerQuery),
    responses((status = 200, body = Envelope<Page<AdminLedgerRow>>), (status = 400, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn ledger_page<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
    Path(host_id): Path<String>,
    params: std::result::Result<Query<LedgerQuery>, QueryRejection>,
) -> Reply<Page<AdminLedgerRow>> {
    let params = query(params)?;
    let request = paging(params.page, params.page_size);
    let page = ledger::ledger(state.kernel().db.pool(), &actor, &host_id, params.entry_type.as_deref(), request).await?;
    reply(page, &actor)
}

#[utoipa::path(
    post, path = "/api/admin/credits/hosts/{hostId}/topup", operation_id = "admin_credit_topup",
    params(("hostId" = String, Path)), request_body = TopupInput,
    responses((status = 200, body = Envelope<BalanceView>), (status = 400, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn topup<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
    Path(host_id): Path<String>,
    input: std::result::Result<Json<TopupInput>, JsonRejection>,
) -> Reply<BalanceView> {
    let input = body(input)?;
    let view = credit_writes::topup(state.kernel().db.pool(), &actor, &host_id, input).await?;
    reply(view, &actor)
}

#[utoipa::path(
    post, path = "/api/admin/credits/hosts/{hostId}/adjust", operation_id = "admin_credit_adjust",
    params(("hostId" = String, Path)), request_body = AdjustInput,
    responses((status = 200, body = Envelope<BalanceView>), (status = 400, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn adjust<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
    Path(host_id): Path<String>,
    input: std::result::Result<Json<AdjustInput>, JsonRejection>,
) -> Reply<BalanceView> {
    let input = body(input)?;
    let view = credit_writes::adjust(state.kernel().db.pool(), &actor, &host_id, input).await?;
    reply(view, &actor)
}
