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
use crate::reservations::cases::types::{CaseView, CloseCaseInput, MyCasesQuery, OpenCaseInput};

#[utoipa::path(
    post, path = "/api/guest/cases", operation_id = "cases_open_guest", request_body = OpenCaseInput,
    responses((status = 200, body = Envelope<CaseView>), (status = 409, body = ErrorBody))
)]
pub async fn open<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Body(input): Body<OpenCaseInput>) -> Reply<CaseView> {
    Ok(reply(service::open_case(app_kernel(&state), &actor, input).await?, id))
}

#[utoipa::path(
    get, path = "/api/guest/cases/mine", operation_id = "cases_mine", params(MyCasesQuery),
    responses((status = 200, body = Envelope<Page<CaseView>>), (status = 400, body = ErrorBody))
)]
pub async fn mine<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Params(query): Params<MyCasesQuery>) -> Reply<Page<CaseView>> {
    Ok(reply(read::my_cases(app_kernel(&state), &actor, query).await?, id))
}

#[utoipa::path(
    get, path = "/api/guest/cases/{caseId}", operation_id = "cases_get_guest", params(("caseId" = String, Path)),
    responses((status = 200, body = Envelope<CaseView>), (status = 404, body = ErrorBody))
)]
pub async fn get<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(case_id): Path<String>) -> Reply<CaseView> {
    Ok(reply(read::get_case(app_kernel(&state), &actor, &case_id).await?, id))
}

#[utoipa::path(
    post, path = "/api/guest/cases/{caseId}/close", operation_id = "cases_close_guest",
    params(("caseId" = String, Path)), request_body = CloseCaseInput,
    responses((status = 200, body = Envelope<CaseView>), (status = 409, body = ErrorBody))
)]
pub async fn close<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(case_id): Path<String>, Body(input): Body<CloseCaseInput>) -> Reply<CaseView> {
    Ok(reply(service::close_case(app_kernel(&state), &actor, &case_id, input).await?, id))
}

#[utoipa::path(
    get, path = "/api/guest/cases/{caseId}/evidence", operation_id = "cases_evidence_list_guest", params(("caseId" = String, Path)),
    responses((status = 200, body = Envelope<Vec<EvidenceView>>), (status = 404, body = ErrorBody))
)]
pub async fn evidence_list<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(case_id): Path<String>) -> Reply<Vec<EvidenceView>> {
    Ok(reply(evidence::list_evidence(app_kernel(&state), &actor, &case_id).await?, id))
}

#[utoipa::path(
    post, path = "/api/guest/cases/{caseId}/evidence", operation_id = "cases_evidence_add_guest",
    params(("caseId" = String, Path)), request_body = AddEvidenceInput,
    responses((status = 200, body = Envelope<EvidenceView>), (status = 400, body = ErrorBody))
)]
pub async fn evidence_add<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(case_id): Path<String>, Body(input): Body<AddEvidenceInput>) -> Reply<EvidenceView> {
    Ok(reply(evidence::add_evidence(app_kernel(&state), &actor, &case_id, input).await?, id))
}

#[utoipa::path(
    get, path = "/api/guest/cases/evidence/{evidenceId}", operation_id = "cases_evidence_get_guest", params(("evidenceId" = String, Path)),
    responses((status = 200, body = Envelope<EvidenceContent>), (status = 404, body = ErrorBody))
)]
pub async fn evidence_get<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(evidence_id): Path<String>) -> Reply<EvidenceContent> {
    Ok(reply(evidence::get_evidence(app_kernel(&state), &actor, &evidence_id).await?, id))
}
