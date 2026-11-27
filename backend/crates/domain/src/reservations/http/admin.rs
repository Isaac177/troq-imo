use axum::extract::{Path, State};
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::ErrorBody;
use kernel::Page;

use super::extract::{reply, AppState, Body, Params, Reply};
use super::guest::app_kernel;
use crate::reservations::cases::admin::{self, AdminCaseDetail, AdminCaseItem, AdminCaseQuery};
use crate::reservations::cases::admin_actions::{
    self as actions, AdminEvidenceContent, AdminOpenCaseInput, AssignCaseInput, EscalateCaseInput,
};
use crate::reservations::cases::types::CloseCaseInput;
use crate::reservations::cases::assignees::{self, AssigneeQuery, CaseAssignee};
use crate::reservations::lifecycle::platform_cancel;
use crate::reservations::lifecycle::types::{CancelPreview, PlatformCancelInput, ReservationView};

#[utoipa::path(
    get, path = "/api/admin/cases", operation_id = "admin_cases_list", params(AdminCaseQuery),
    responses((status = 200, body = Envelope<Page<AdminCaseItem>>), (status = 403, body = ErrorBody))
)]
pub async fn cases_list<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Params(query): Params<AdminCaseQuery>) -> Reply<Page<AdminCaseItem>> {
    Ok(reply(admin::list_cases(app_kernel(&state), &actor, query).await?, id))
}

#[utoipa::path(
    post, path = "/api/admin/cases", operation_id = "admin_cases_open", request_body = AdminOpenCaseInput,
    responses((status = 200, body = Envelope<AdminCaseItem>), (status = 403, body = ErrorBody))
)]
pub async fn cases_open<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Body(input): Body<AdminOpenCaseInput>) -> Reply<AdminCaseItem> {
    Ok(reply(actions::open_case(app_kernel(&state), &actor, input).await?, id))
}

#[utoipa::path(
    get, path = "/api/admin/cases/{caseId}", operation_id = "admin_cases_get", params(("caseId" = String, Path)),
    responses((status = 200, body = Envelope<AdminCaseDetail>), (status = 404, body = ErrorBody))
)]
pub async fn cases_get<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(case_id): Path<String>) -> Reply<AdminCaseDetail> {
    Ok(reply(admin::get_case(app_kernel(&state), &actor, &case_id).await?, id))
}

#[utoipa::path(
    post, path = "/api/admin/cases/{caseId}/close", operation_id = "admin_cases_close",
    params(("caseId" = String, Path)), request_body = CloseCaseInput,
    responses((status = 200, body = Envelope<AdminCaseItem>), (status = 409, body = ErrorBody))
)]
pub async fn cases_close<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(case_id): Path<String>, Body(input): Body<CloseCaseInput>) -> Reply<AdminCaseItem> {
    Ok(reply(admin::close_case(app_kernel(&state), &actor, &case_id, input).await?, id))
}

#[utoipa::path(
    post, path = "/api/admin/cases/{caseId}/assign", operation_id = "admin_cases_assign",
    params(("caseId" = String, Path)), request_body = AssignCaseInput,
    responses((status = 200, body = Envelope<AdminCaseItem>), (status = 409, body = ErrorBody))
)]
pub async fn cases_assign<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(case_id): Path<String>, Body(input): Body<AssignCaseInput>) -> Reply<AdminCaseItem> {
    Ok(reply(actions::assign_case(app_kernel(&state), &actor, &case_id, input).await?, id))
}

#[utoipa::path(
    post, path = "/api/admin/cases/{caseId}/escalate", operation_id = "admin_cases_escalate",
    params(("caseId" = String, Path)), request_body = EscalateCaseInput,
    responses((status = 200, body = Envelope<AdminCaseItem>), (status = 409, body = ErrorBody))
)]
pub async fn cases_escalate<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(case_id): Path<String>, Body(input): Body<EscalateCaseInput>) -> Reply<AdminCaseItem> {
    Ok(reply(actions::escalate_case(app_kernel(&state), &actor, &case_id, input).await?, id))
}

#[utoipa::path(
    get, path = "/api/admin/cases/evidence/{evidenceId}", operation_id = "admin_cases_evidence", params(("evidenceId" = String, Path)),
    responses((status = 200, body = Envelope<AdminEvidenceContent>), (status = 404, body = ErrorBody))
)]
pub async fn cases_evidence<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(evidence_id): Path<String>) -> Reply<AdminEvidenceContent> {
    Ok(reply(actions::get_evidence(app_kernel(&state), &actor, &evidence_id).await?, id))
}

#[utoipa::path(
    get, path = "/api/admin/reservations/{reservationId}/cancel-preview", operation_id = "admin_reservations_cancel_preview",
    params(("reservationId" = String, Path)),
    responses((status = 200, body = Envelope<CancelPreview>), (status = 404, body = ErrorBody))
)]
pub async fn cancel_preview<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(reservation_id): Path<String>) -> Reply<CancelPreview> {
    Ok(reply(platform_cancel::preview(app_kernel(&state), &actor, &reservation_id).await?, id))
}

#[utoipa::path(
    post, path = "/api/admin/reservations/{reservationId}/cancel", operation_id = "admin_reservations_cancel",
    params(("reservationId" = String, Path)), request_body = PlatformCancelInput,
    responses((status = 200, body = Envelope<ReservationView>), (status = 409, body = ErrorBody))
)]
pub async fn reservation_cancel<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(reservation_id): Path<String>, Body(input): Body<PlatformCancelInput>) -> Reply<ReservationView> {
    Ok(reply(platform_cancel::platform_cancel(app_kernel(&state), &actor, &reservation_id, input).await?, id))
}

#[utoipa::path(
    get, path = "/api/admin/cases/assignees", operation_id = "admin_case_assignees_list", params(AssigneeQuery),
    responses((status = 200, body = Envelope<Page<CaseAssignee>>), (status = 403, body = ErrorBody))
)]
pub async fn cases_assignees<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Params(query): Params<AssigneeQuery>) -> Reply<Page<CaseAssignee>> {
    Ok(reply(assignees::list(app_kernel(&state), &actor, query).await?, id))
}
