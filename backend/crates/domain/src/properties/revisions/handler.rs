use axum::extract::{Path, State};
use axum::Json;
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{ErrorBody, Result};
use kernel::kernel::KernelState;
use kernel::paging::Page;

use super::service;
use super::types::{CreateRevision, RevisionAction, RevisionListQuery, RevisionSummary, RevisionView, UpdateRevision};
use crate::properties::http::{paging, JsonBody, Params};

#[utoipa::path(get, path = "/api/host/properties/{propertyId}/revisions", operation_id = "host_revisions_list",
    params(("propertyId" = String, Path), RevisionListQuery),
    responses((status = 200, body = Envelope<Page<RevisionSummary>>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody)))]
pub async fn list<S>(State(state): State<S>, actor: Actor, RequestId(request_id): RequestId, Path(property_id): Path<String>, Params(query): Params<RevisionListQuery>) -> Result<Json<Envelope<Page<RevisionSummary>>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let page = paging(query.page, query.page_size)?;
    Ok(Json(Envelope::ok(service::list(state.kernel().db.pool(), &actor, &property_id, page).await?, request_id)))
}

#[utoipa::path(get, path = "/api/host/properties/{propertyId}/revisions/{revisionId}", operation_id = "host_revisions_get",
    params(("propertyId" = String, Path), ("revisionId" = String, Path)),
    responses((status = 200, body = Envelope<RevisionView>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody)))]
pub async fn get<S>(State(state): State<S>, actor: Actor, RequestId(request_id): RequestId, Path((property_id, revision_id)): Path<(String, String)>) -> Result<Json<Envelope<RevisionView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(service::get(state.kernel().db.pool(), &actor, &property_id, &revision_id).await?, request_id)))
}

#[utoipa::path(post, path = "/api/host/properties/{propertyId}/revisions", operation_id = "host_revisions_create",
    params(("propertyId" = String, Path)), request_body = CreateRevision,
    responses((status = 200, body = Envelope<RevisionView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 409, body = ErrorBody)))]
pub async fn create<S>(State(state): State<S>, actor: Actor, RequestId(request_id): RequestId, Path(property_id): Path<String>, JsonBody(input): JsonBody<CreateRevision>) -> Result<Json<Envelope<RevisionView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(service::create(state.kernel().db.pool(), &actor, &property_id, input).await?, request_id)))
}

#[utoipa::path(patch, path = "/api/host/properties/{propertyId}/revisions/{revisionId}", operation_id = "host_revisions_update",
    params(("propertyId" = String, Path), ("revisionId" = String, Path)), request_body = UpdateRevision,
    responses((status = 200, body = Envelope<RevisionView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 409, body = ErrorBody)))]
pub async fn update<S>(State(state): State<S>, actor: Actor, RequestId(request_id): RequestId, Path((property_id, revision_id)): Path<(String, String)>, JsonBody(input): JsonBody<UpdateRevision>) -> Result<Json<Envelope<RevisionView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(service::update(state.kernel().db.pool(), &actor, &property_id, &revision_id, input).await?, request_id)))
}

#[utoipa::path(post, path = "/api/host/properties/{propertyId}/revisions/{revisionId}/withdraw", operation_id = "host_revisions_withdraw",
    params(("propertyId" = String, Path), ("revisionId" = String, Path)), request_body = RevisionAction,
    responses((status = 200, body = Envelope<RevisionView>), (status = 403, body = ErrorBody), (status = 409, body = ErrorBody)))]
pub async fn withdraw<S>(State(state): State<S>, actor: Actor, RequestId(request_id): RequestId, Path((property_id, revision_id)): Path<(String, String)>, JsonBody(input): JsonBody<RevisionAction>) -> Result<Json<Envelope<RevisionView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(service::withdraw(state.kernel().db.pool(), &actor, &property_id, &revision_id, input).await?, request_id)))
}

#[utoipa::path(get, path = "/api/host/properties/{propertyId}/revisions/{revisionId}/readiness", operation_id = "host_revisions_readiness",
    params(("propertyId" = String, Path), ("revisionId" = String, Path)),
    responses((status = 200, body = Envelope<crate::properties::listings::types::ReadinessView>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody)))]
pub async fn readiness<S>(State(state): State<S>, actor: Actor, RequestId(request_id): RequestId, Path((property_id, revision_id)): Path<(String, String)>) -> Result<Json<Envelope<crate::properties::listings::types::ReadinessView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(service::readiness(state.kernel().db.pool(), &actor, &property_id, &revision_id).await?, request_id)))
}

#[utoipa::path(post, path = "/api/host/properties/{propertyId}/revisions/{revisionId}/submit", operation_id = "host_revisions_submit",
    params(("propertyId" = String, Path), ("revisionId" = String, Path)), request_body = RevisionAction,
    responses((status = 200, body = Envelope<RevisionView>), (status = 403, body = ErrorBody), (status = 409, body = ErrorBody)))]
pub async fn submit<S>(State(state): State<S>, actor: Actor, RequestId(request_id): RequestId, Path((property_id, revision_id)): Path<(String, String)>, JsonBody(input): JsonBody<RevisionAction>) -> Result<Json<Envelope<RevisionView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(service::submit(state.kernel().db.pool(), &actor, &property_id, &revision_id, input).await?, request_id)))
}

#[utoipa::path(post, path = "/api/host/properties/{propertyId}/lifecycle", operation_id = "host_property_lifecycle",
    params(("propertyId" = String, Path)), request_body = crate::properties::listings::types::LifecycleInput,
    responses((status = 200, body = Envelope<crate::properties::property::types::PropertyView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 409, body = ErrorBody)))]
pub async fn lifecycle<S>(State(state): State<S>, actor: Actor, RequestId(request_id): RequestId, Path(property_id): Path<String>, JsonBody(input): JsonBody<crate::properties::listings::types::LifecycleInput>) -> Result<Json<Envelope<crate::properties::property::types::PropertyView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(crate::properties::listings::service::host_lifecycle(state.kernel().db.pool(), &actor, &property_id, input).await?, request_id)))
}
