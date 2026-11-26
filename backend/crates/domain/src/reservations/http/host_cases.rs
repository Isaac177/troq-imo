use axum::extract::{Path, State};
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::ErrorBody;
use kernel::Page;

use super::extract::{reply, AppState, Body, Params, Reply};
use super::guest::app_kernel;
use crate::reservations::cases::evidence::{self, AddEvidenceInput, EvidenceContent, EvidenceView};
use crate::reservations::cases::read;
use crate::reservations::cases::service;
use crate::reservations::cases::types::{CaseListQuery, CaseView, CloseCaseInput, OpenCaseInput};
use crate::reservations::operations::service as tasks;
use crate::reservations::operations::types::{CreateTaskInput, TaskListQuery, TaskView, TransitionInput};

#[utoipa::path(
    get, path = "/api/host/cases", operation_id = "cases_list_host", params(CaseListQuery),
    responses((status = 200, body = Envelope<Page<CaseView>>), (status = 404, body = ErrorBody))
)]
pub async fn list<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Params(query): Params<CaseListQuery>) -> Reply<Page<CaseView>> {
    Ok(reply(read::list_cases(app_kernel(&state), &actor, query).await?, id))
}

#[utoipa::path(
    post, path = "/api/host/cases", operation_id = "cases_open_host", request_body = OpenCaseInput,
    responses((status = 200, body = Envelope<CaseView>), (status = 409, body = ErrorBody))
)]
pub async fn open<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Body(input): Body<OpenCaseInput>) -> Reply<CaseView> {
    Ok(reply(service::open_case(app_kernel(&state), &actor, input).await?, id))
}

#[utoipa::path(
    get, path = "/api/host/cases/{caseId}", operation_id = "cases_get_host", params(("caseId" = String, Path)),
    responses((status = 200, body = Envelope<CaseView>), (status = 404, body = ErrorBody))
)]
pub async fn get<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(case_id): Path<String>) -> Reply<CaseView> {
    Ok(reply(read::get_case(app_kernel(&state), &actor, &case_id).await?, id))
}

#[utoipa::path(
    post, path = "/api/host/cases/{caseId}/close", operation_id = "cases_close_host",
    params(("caseId" = String, Path)), request_body = CloseCaseInput,
    responses((status = 200, body = Envelope<CaseView>), (status = 409, body = ErrorBody))
)]
pub async fn close<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(case_id): Path<String>, Body(input): Body<CloseCaseInput>) -> Reply<CaseView> {
    Ok(reply(service::close_case(app_kernel(&state), &actor, &case_id, input).await?, id))
}

#[utoipa::path(
    get, path = "/api/host/cases/{caseId}/evidence", operation_id = "cases_evidence_list_host", params(("caseId" = String, Path)),
    responses((status = 200, body = Envelope<Vec<EvidenceView>>), (status = 404, body = ErrorBody))
)]
pub async fn evidence_list<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(case_id): Path<String>) -> Reply<Vec<EvidenceView>> {
    Ok(reply(evidence::list_evidence(app_kernel(&state), &actor, &case_id).await?, id))
}

#[utoipa::path(
    post, path = "/api/host/cases/{caseId}/evidence", operation_id = "cases_evidence_add_host",
    params(("caseId" = String, Path)), request_body = AddEvidenceInput,
    responses((status = 200, body = Envelope<EvidenceView>), (status = 400, body = ErrorBody))
)]
pub async fn evidence_add<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(case_id): Path<String>, Body(input): Body<AddEvidenceInput>) -> Reply<EvidenceView> {
    Ok(reply(evidence::add_evidence(app_kernel(&state), &actor, &case_id, input).await?, id))
}

#[utoipa::path(
    get, path = "/api/host/cases/evidence/{evidenceId}", operation_id = "cases_evidence_get_host", params(("evidenceId" = String, Path)),
    responses((status = 200, body = Envelope<EvidenceContent>), (status = 404, body = ErrorBody))
)]
pub async fn evidence_get<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(evidence_id): Path<String>) -> Reply<EvidenceContent> {
    Ok(reply(evidence::get_evidence(app_kernel(&state), &actor, &evidence_id).await?, id))
}

#[utoipa::path(
    get, path = "/api/host/tasks", operation_id = "operations_tasks_list_host", params(TaskListQuery),
    responses((status = 200, body = Envelope<Page<TaskView>>), (status = 403, body = ErrorBody))
)]
pub async fn tasks_list<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Params(query): Params<TaskListQuery>) -> Reply<Page<TaskView>> {
    Ok(reply(tasks::list_tasks(app_kernel(&state), &actor, query).await?, id))
}

#[utoipa::path(
    post, path = "/api/host/tasks", operation_id = "operations_tasks_create_host", request_body = CreateTaskInput,
    responses((status = 200, body = Envelope<TaskView>), (status = 403, body = ErrorBody))
)]
pub async fn tasks_create<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Body(input): Body<CreateTaskInput>) -> Reply<TaskView> {
    Ok(reply(tasks::create_task(app_kernel(&state), &actor, input).await?, id))
}

#[utoipa::path(
    get, path = "/api/host/tasks/{taskId}", operation_id = "operations_tasks_get_host", params(("taskId" = String, Path)),
    responses((status = 200, body = Envelope<TaskView>), (status = 404, body = ErrorBody))
)]
pub async fn tasks_get<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(task_id): Path<String>) -> Reply<TaskView> {
    Ok(reply(tasks::get_task(app_kernel(&state), &actor, &task_id).await?, id))
}

#[utoipa::path(
    post, path = "/api/host/tasks/{taskId}/transition", operation_id = "operations_tasks_transition_host",
    params(("taskId" = String, Path)), request_body = TransitionInput,
    responses((status = 200, body = Envelope<TaskView>), (status = 409, body = ErrorBody))
)]
pub async fn tasks_transition<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(task_id): Path<String>, Body(input): Body<TransitionInput>) -> Reply<TaskView> {
    Ok(reply(tasks::transition_task(app_kernel(&state), &actor, &task_id, input).await?, id))
}
