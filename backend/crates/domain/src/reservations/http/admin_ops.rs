use axum::extract::{Path, State};
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::ErrorBody;
use kernel::Page;

use super::extract::{reply, AppState, Body, Params, Reply};
use super::guest::app_kernel;
use crate::reservations::claims::types::{
    ClaimDecisionResult, ClaimDetail, ClaimFilter, ClaimRow, ClaimSummary, DecideInput, PayoutInput, PayoutResult,
    StatementInput, StatementRow,
};
use crate::reservations::claims::{decide, detail, list, payout, statements};
use crate::reservations::operations::admin::{self as tasks, AdminTaskItem, AdminTaskQuery};
use crate::reservations::operations::types::{CreateTaskInput, TransitionInput};

#[utoipa::path(
    get, path = "/api/admin/tasks", operation_id = "admin_tasks_list", params(AdminTaskQuery),
    responses((status = 200, body = Envelope<Page<AdminTaskItem>>), (status = 403, body = ErrorBody))
)]
pub async fn tasks_list<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Params(query): Params<AdminTaskQuery>) -> Reply<Page<AdminTaskItem>> {
    Ok(reply(tasks::list_tasks(app_kernel(&state), &actor, query).await?, id))
}

#[utoipa::path(
    post, path = "/api/admin/tasks", operation_id = "admin_tasks_create", request_body = CreateTaskInput,
    responses((status = 200, body = Envelope<AdminTaskItem>), (status = 404, body = ErrorBody))
)]
pub async fn tasks_create<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Body(input): Body<CreateTaskInput>) -> Reply<AdminTaskItem> {
    Ok(reply(tasks::create_task(app_kernel(&state), &actor, input).await?, id))
}

#[utoipa::path(
    get, path = "/api/admin/tasks/{taskId}", operation_id = "admin_tasks_get", params(("taskId" = String, Path)),
    responses((status = 200, body = Envelope<AdminTaskItem>), (status = 404, body = ErrorBody))
)]
pub async fn tasks_get<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(task_id): Path<String>) -> Reply<AdminTaskItem> {
    Ok(reply(tasks::get_task(app_kernel(&state), &actor, &task_id).await?, id))
}

#[utoipa::path(
    post, path = "/api/admin/tasks/{taskId}/transition", operation_id = "admin_tasks_transition",
    params(("taskId" = String, Path)), request_body = TransitionInput,
    responses((status = 200, body = Envelope<AdminTaskItem>), (status = 409, body = ErrorBody))
)]
pub async fn tasks_transition<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(task_id): Path<String>, Body(input): Body<TransitionInput>) -> Reply<AdminTaskItem> {
    Ok(reply(tasks::transition_task(app_kernel(&state), &actor, &task_id, input).await?, id))
}

#[utoipa::path(
    get, path = "/api/admin/claims", operation_id = "admin_claims_list", params(ClaimFilter),
    responses((status = 200, body = Envelope<Page<ClaimRow>>), (status = 403, body = ErrorBody))
)]
pub async fn claims_list<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Params(filter): Params<ClaimFilter>) -> Reply<Page<ClaimRow>> {
    Ok(reply(list::list(app_kernel(&state), &actor, filter).await?, id))
}

#[utoipa::path(
    get, path = "/api/admin/claims/summary", operation_id = "admin_claims_summary", params(ClaimFilter),
    responses((status = 200, body = Envelope<ClaimSummary>), (status = 403, body = ErrorBody))
)]
pub async fn claims_summary<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Params(filter): Params<ClaimFilter>) -> Reply<ClaimSummary> {
    Ok(reply(list::summary(app_kernel(&state), &actor, filter).await?, id))
}

#[utoipa::path(
    get, path = "/api/admin/claims/{caseId}", operation_id = "admin_claims_get", params(("caseId" = String, Path)),
    responses((status = 200, body = Envelope<ClaimDetail>), (status = 404, body = ErrorBody))
)]
pub async fn claims_get<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(case_id): Path<String>) -> Reply<ClaimDetail> {
    Ok(reply(detail::get(app_kernel(&state), &actor, &case_id).await?, id))
}

#[utoipa::path(
    post, path = "/api/admin/claims/{caseId}/statement", operation_id = "admin_claims_statement",
    params(("caseId" = String, Path)), request_body = StatementInput,
    responses((status = 200, body = Envelope<StatementRow>), (status = 404, body = ErrorBody))
)]
pub async fn claims_statement<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(case_id): Path<String>, Body(input): Body<StatementInput>) -> Reply<StatementRow> {
    Ok(reply(statements::add(app_kernel(&state), &actor, &case_id, input).await?, id))
}

#[utoipa::path(
    post, path = "/api/admin/claims/{caseId}/decision", operation_id = "admin_claims_decide",
    params(("caseId" = String, Path)), request_body = DecideInput,
    responses((status = 200, body = Envelope<ClaimDecisionResult>), (status = 409, body = ErrorBody))
)]
pub async fn claims_decide<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(case_id): Path<String>, Body(input): Body<DecideInput>) -> Reply<ClaimDecisionResult> {
    Ok(reply(decide::decide(app_kernel(&state), &actor, &case_id, input).await?, id))
}

#[utoipa::path(
    post, path = "/api/admin/claims/{caseId}/payout", operation_id = "admin_claims_payout",
    params(("caseId" = String, Path)), request_body = PayoutInput,
    responses((status = 200, body = Envelope<PayoutResult>), (status = 409, body = ErrorBody))
)]
pub async fn claims_payout<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(case_id): Path<String>, Body(input): Body<PayoutInput>) -> Reply<PayoutResult> {
    Ok(reply(payout::update(app_kernel(&state), &actor, &case_id, input).await?, id))
}
