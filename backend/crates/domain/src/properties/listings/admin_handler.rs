use axum::extract::{Path, State};
use axum::Json;
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{ErrorBody, Result};
use kernel::kernel::KernelState;
use kernel::paging::Page;

use super::types::{
    AdminPropertyDetail, AdminPropertyItem, AdminPropertyQuery, AdminRevisionView, AdminUpdateInput, DecisionInput,
    LifecycleInput, ModerationInput, QueueItem, QueueQuery,
};
use super::{admin_list, admin_property, review, service};
use crate::properties::http::{paging, JsonBody, Params};
use crate::properties::property::types::{CreatePropertyInput, PropertyView};
use crate::properties::revisions::types::{RevisionListQuery, RevisionSummary, RevisionView};
use crate::properties::scope::ensure_moderator;

type Reply<T> = Result<Json<Envelope<T>>>;

#[utoipa::path(
    get, path = "/api/admin/listings/queue", operation_id = "admin_listing_queue", params(QueueQuery),
    responses((status = 200, body = Envelope<Page<QueueItem>>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody))
)]
pub async fn queue<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Params(query): Params<QueueQuery>) -> Reply<Page<QueueItem>>
where S: KernelState + Clone + Send + Sync + 'static {
    let page = paging(query.page, query.page_size)?;
    Ok(Json(Envelope::ok(service::admin_queue(state.kernel().db.pool(), &actor, query.status, page).await?, rid)))
}

#[utoipa::path(
    get, path = "/api/admin/listings/{revisionId}", operation_id = "admin_listing_revision_get",
    params(("revisionId" = String, Path)),
    responses((status = 200, body = Envelope<AdminRevisionView>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn revision<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path(revision_id): Path<String>) -> Reply<AdminRevisionView>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(service::admin_revision(state.kernel().db.pool(), &actor, &revision_id).await?, rid)))
}

#[utoipa::path(
    post, path = "/api/admin/listings/{revisionId}/decision", operation_id = "admin_listing_decide",
    params(("revisionId" = String, Path)), request_body = DecisionInput,
    responses((status = 200, body = Envelope<RevisionView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn decide<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path(revision_id): Path<String>, JsonBody(input): JsonBody<DecisionInput>) -> Reply<RevisionView>
where S: KernelState + Clone + Send + Sync + 'static {
    ensure_moderator(&actor)?;
    Ok(Json(Envelope::ok(review::decide(state.kernel().db.pool(), &actor, &revision_id, input).await?, rid)))
}

#[utoipa::path(
    get, path = "/api/admin/properties", operation_id = "admin_properties_list", params(AdminPropertyQuery),
    responses((status = 200, body = Envelope<Page<AdminPropertyItem>>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody))
)]
pub async fn list<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Params(query): Params<AdminPropertyQuery>) -> Reply<Page<AdminPropertyItem>>
where S: KernelState + Clone + Send + Sync + 'static {
    ensure_moderator(&actor)?;
    let page = paging(query.page, query.page_size)?;
    Ok(Json(Envelope::ok(admin_list::list(state.kernel().db.pool(), actor.city, &query, page).await?, rid)))
}

#[utoipa::path(
    get, path = "/api/admin/properties/{propertyId}", operation_id = "admin_properties_get",
    params(("propertyId" = String, Path)),
    responses((status = 200, body = Envelope<AdminPropertyDetail>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn get<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path(property_id): Path<String>) -> Reply<AdminPropertyDetail>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(admin_property::detail(state.kernel().db.pool(), &actor, &property_id).await?, rid)))
}

#[utoipa::path(
    get, path = "/api/admin/properties/{propertyId}/revisions", operation_id = "admin_properties_revisions",
    params(("propertyId" = String, Path), RevisionListQuery),
    responses((status = 200, body = Envelope<Page<RevisionSummary>>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn revisions<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path(property_id): Path<String>, Params(query): Params<RevisionListQuery>) -> Reply<Page<RevisionSummary>>
where S: KernelState + Clone + Send + Sync + 'static {
    let page = paging(query.page, query.page_size)?;
    Ok(Json(Envelope::ok(admin_property::revisions(state.kernel().db.pool(), &actor, &property_id, page).await?, rid)))
}

#[utoipa::path(
    post, path = "/api/admin/properties", operation_id = "admin_properties_create", request_body = CreatePropertyInput,
    responses((status = 200, body = Envelope<PropertyView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn create<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, JsonBody(input): JsonBody<CreatePropertyInput>) -> Reply<PropertyView>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(admin_property::create(state.kernel().db.pool(), &actor, input).await?, rid)))
}

#[utoipa::path(
    patch, path = "/api/admin/properties/{propertyId}", operation_id = "admin_properties_update",
    params(("propertyId" = String, Path)), request_body = AdminUpdateInput,
    responses((status = 200, body = Envelope<PropertyView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn update<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path(property_id): Path<String>, JsonBody(input): JsonBody<AdminUpdateInput>) -> Reply<PropertyView>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(admin_property::update(state.kernel().db.pool(), &actor, &property_id, input).await?, rid)))
}

#[utoipa::path(
    post, path = "/api/admin/properties/{propertyId}/suspend", operation_id = "admin_properties_suspend",
    params(("propertyId" = String, Path)), request_body = ModerationInput,
    responses((status = 200, body = Envelope<PropertyView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn suspend<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path(property_id): Path<String>, JsonBody(input): JsonBody<ModerationInput>) -> Reply<PropertyView>
where S: KernelState + Clone + Send + Sync + 'static {
    ensure_moderator(&actor)?;
    Ok(Json(Envelope::ok(review::suspend(state.kernel().db.pool(), &actor, &property_id, input).await?, rid)))
}

#[utoipa::path(
    post, path = "/api/admin/properties/{propertyId}/reinstate", operation_id = "admin_properties_reinstate",
    params(("propertyId" = String, Path)), request_body = ModerationInput,
    responses((status = 200, body = Envelope<PropertyView>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn reinstate<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path(property_id): Path<String>, JsonBody(input): JsonBody<ModerationInput>) -> Reply<PropertyView>
where S: KernelState + Clone + Send + Sync + 'static {
    ensure_moderator(&actor)?;
    Ok(Json(Envelope::ok(review::reinstate(state.kernel().db.pool(), &actor, &property_id, input).await?, rid)))
}

#[utoipa::path(
    post, path = "/api/admin/properties/{propertyId}/lifecycle", operation_id = "admin_properties_lifecycle",
    params(("propertyId" = String, Path)), request_body = LifecycleInput,
    responses((status = 200, body = Envelope<PropertyView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn lifecycle<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path(property_id): Path<String>, JsonBody(input): JsonBody<LifecycleInput>) -> Reply<PropertyView>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(service::admin_lifecycle(state.kernel().db.pool(), &actor, &property_id, input).await?, rid)))
}
