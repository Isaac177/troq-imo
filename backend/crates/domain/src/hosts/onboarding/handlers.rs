use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::extract::{Json, Path, Query, State};
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{codes, Error, ErrorBody, Result};
use kernel::paging::PageRequest;
use kernel::{KernelState, Page};

use super::admin::{self, DocumentReview};
use super::applications;
use super::documents;
use super::types::{
    AddDocumentInput, AdminApplicationView, AdminHostView, ApplicationFilter, ApplicationInput,
    ApplicationView, HostDecisionInput, HostDocumentView, HostStatusInput, HostStatusView, OperationNotice,
};

fn body<T>(input: std::result::Result<Json<T>, JsonRejection>) -> Result<T> {
    input.map(|Json(value)| value).map_err(|_| Error::bad_request(codes::MALFORMED_REQUEST))
}

fn filter(input: std::result::Result<Query<ApplicationFilter>, QueryRejection>) -> Result<ApplicationFilter> {
    input.map(|Query(value)| value).map_err(|_| Error::bad_request(codes::MALFORMED_REQUEST))
}

fn page(input: std::result::Result<Query<PageRequest>, QueryRejection>) -> Result<PageRequest> {
    input.map(|Query(value)| value).map_err(|_| Error::bad_request(codes::MALFORMED_REQUEST))
}

#[utoipa::path(
    get, path = "/api/host/applications", operation_id = "host_applications_mine",
    params(("page" = Option<u32>, Query), ("pageSize" = Option<u32>, Query)),
    responses((status = 200, body = Envelope<Page<ApplicationView>>), (status = 401, body = ErrorBody))
)]
pub async fn list_mine<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    input: std::result::Result<Query<PageRequest>, QueryRejection>,
) -> Result<Json<Envelope<Page<ApplicationView>>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let result = applications::list_mine(state.kernel().db.pool(), &actor, page(input)?).await?;
    Ok(Json(Envelope::ok(result, id)))
}

#[utoipa::path(
    post, path = "/api/host/applications", operation_id = "host_application_save",
    request_body = ApplicationInput,
    responses((status = 200, body = Envelope<ApplicationView>), (status = 400, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn save<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    input: std::result::Result<Json<ApplicationInput>, JsonRejection>,
) -> Result<Json<Envelope<ApplicationView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let result = applications::create_or_update_draft(state.kernel().db.pool(), &actor, body(input)?).await?;
    Ok(Json(Envelope::ok(result, id)))
}

#[utoipa::path(
    post, path = "/api/host/applications/{applicationId}/submit", operation_id = "host_application_submit",
    params(("applicationId" = String, Path)),
    responses((status = 200, body = Envelope<ApplicationView>), (status = 400, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn submit<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    Path(application_id): Path<String>,
) -> Result<Json<Envelope<ApplicationView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let result = applications::submit(state.kernel().db.pool(), &actor, &application_id).await?;
    Ok(Json(Envelope::ok(result, id)))
}

#[utoipa::path(
    post, path = "/api/host/applications/{applicationId}/withdraw", operation_id = "host_application_withdraw",
    params(("applicationId" = String, Path)),
    responses((status = 200, body = Envelope<ApplicationView>), (status = 409, body = ErrorBody))
)]
pub async fn withdraw<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    Path(application_id): Path<String>,
) -> Result<Json<Envelope<ApplicationView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let result = applications::withdraw(state.kernel().db.pool(), &actor, &application_id).await?;
    Ok(Json(Envelope::ok(result, id)))
}

#[utoipa::path(
    post, path = "/api/host/applications/{applicationId}/documents", operation_id = "host_application_document_add",
    params(("applicationId" = String, Path)), request_body = AddDocumentInput,
    responses((status = 200, body = Envelope<HostDocumentView>), (status = 400, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn add_document<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    Path(application_id): Path<String>, input: std::result::Result<Json<AddDocumentInput>, JsonRejection>,
) -> Result<Json<Envelope<HostDocumentView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let result = documents::add(state.kernel().db.pool(), &state.kernel().storage, &actor, &application_id, body(input)?).await?;
    Ok(Json(Envelope::ok(result, id)))
}

#[utoipa::path(
    delete, path = "/api/host/applications/{applicationId}/documents/{documentId}", operation_id = "host_application_document_remove",
    params(("applicationId" = String, Path), ("documentId" = String, Path)),
    responses((status = 200, body = Envelope<OperationNotice>), (status = 404, body = ErrorBody))
)]
pub async fn remove_document<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    Path((application_id, document_id)): Path<(String, String)>,
) -> Result<Json<Envelope<OperationNotice>>>
where S: KernelState + Clone + Send + Sync + 'static {
    documents::remove(state.kernel().db.pool(), &state.kernel().storage, &actor, &application_id, &document_id).await?;
    Ok(Json(Envelope::ok(OperationNotice { message: "host.document.removed".into() }, id)))
}

#[utoipa::path(
    get, path = "/api/admin/host-applications", operation_id = "admin_host_applications_list",
    params(("status" = Option<String>, Query), ("search" = Option<String>, Query), ("page" = Option<u32>, Query), ("pageSize" = Option<u32>, Query)),
    responses((status = 200, body = Envelope<Page<AdminApplicationView>>), (status = 403, body = ErrorBody))
)]
pub async fn admin_list<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    input: std::result::Result<Query<ApplicationFilter>, QueryRejection>,
) -> Result<Json<Envelope<Page<AdminApplicationView>>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let result = admin::list_applications(state.kernel().db.pool(), &actor, filter(input)?).await?;
    Ok(Json(Envelope::ok(result, id)))
}

#[utoipa::path(
    get, path = "/api/admin/host-applications/{applicationId}", operation_id = "admin_host_application_detail",
    params(("applicationId" = String, Path)),
    responses((status = 200, body = Envelope<ApplicationView>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn admin_detail<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    Path(application_id): Path<String>,
) -> Result<Json<Envelope<ApplicationView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let result = admin::application_detail(state.kernel().db.pool(), &actor, &application_id).await?;
    Ok(Json(Envelope::ok(result, id)))
}

#[utoipa::path(
    post, path = "/api/admin/host-applications/{applicationId}/decision", operation_id = "admin_host_application_decide",
    params(("applicationId" = String, Path)), request_body = HostDecisionInput,
    responses((status = 200, body = Envelope<ApplicationView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn admin_decide<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    Path(application_id): Path<String>, input: std::result::Result<Json<HostDecisionInput>, JsonRejection>,
) -> Result<Json<Envelope<ApplicationView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let result = admin::decide(state.kernel().db.pool(), &actor, &application_id, body(input)?).await?;
    Ok(Json(Envelope::ok(result, id)))
}

#[utoipa::path(
    post, path = "/api/admin/host-applications/{applicationId}/documents/{documentId}/review", operation_id = "admin_host_document_review",
    params(("applicationId" = String, Path), ("documentId" = String, Path)), request_body = DocumentReview,
    responses((status = 200, body = Envelope<HostDocumentView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn admin_review_document<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    Path((application_id, document_id)): Path<(String, String)>,
    input: std::result::Result<Json<DocumentReview>, JsonRejection>,
) -> Result<Json<Envelope<HostDocumentView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let result = admin::review_document(state.kernel().db.pool(), &actor, &application_id, &document_id, body(input)?).await?;
    Ok(Json(Envelope::ok(result, id)))
}

#[utoipa::path(
    get, path = "/api/admin/hosts", operation_id = "admin_hosts_list",
    params(("status" = Option<String>, Query), ("search" = Option<String>, Query), ("page" = Option<u32>, Query), ("pageSize" = Option<u32>, Query)),
    responses((status = 200, body = Envelope<Page<AdminHostView>>), (status = 403, body = ErrorBody))
)]
pub async fn admin_hosts<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    input: std::result::Result<Query<ApplicationFilter>, QueryRejection>,
) -> Result<Json<Envelope<Page<AdminHostView>>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let result = admin::list_hosts(state.kernel().db.pool(), &actor, filter(input)?).await?;
    Ok(Json(Envelope::ok(result, id)))
}

#[utoipa::path(
    post, path = "/api/admin/hosts/{hostId}/status", operation_id = "admin_host_status_change",
    params(("hostId" = String, Path)), request_body = HostStatusInput,
    responses((status = 200, body = Envelope<HostStatusView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn admin_host_status<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    Path(host_id): Path<String>, input: std::result::Result<Json<HostStatusInput>, JsonRejection>,
) -> Result<Json<Envelope<HostStatusView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let status = admin::change_host_status(state.kernel().db.pool(), &actor, &host_id, body(input)?).await?;
    Ok(Json(Envelope::ok(HostStatusView { host_id, status }, id)))
}
