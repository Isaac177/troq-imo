use axum::extract::{Path, State};
use axum::Json;
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{ErrorBody, Result};
use kernel::kernel::{Kernel, KernelState};

use super::access::{scoped, scoped_media, scoped_media_by_id};
use super::kind::MediaKind;
use super::service;
use super::types::{
    AddMediaInput, PropertyMediaContent, PropertyMediaRemoved, PropertyMediaView, RemoveMediaInput, UpdateMediaInput,
};
use crate::properties::http::JsonBody;

type Reply<T> = Result<Json<Envelope<T>>>;

async fn list_for(kernel: &Kernel, actor: &Actor, id: &str, admin: bool) -> Result<Vec<PropertyMediaView>> {
    let scope = scoped(kernel.db.pool(), actor, id, false, admin).await?;
    service::list(kernel.db.pool(), scope.owner()).await
}

async fn add_for(kernel: &Kernel, actor: &Actor, id: &str, input: AddMediaInput, admin: bool) -> Result<PropertyMediaView> {
    let scope = scoped(kernel.db.pool(), actor, id, true, admin).await?;
    service::add(kernel.db.pool(), &kernel.storage, actor, scope.owner(), input).await
}

async fn update_for(
    kernel: &Kernel,
    actor: &Actor,
    ids: (&str, &str),
    input: UpdateMediaInput,
    admin: bool,
) -> Result<PropertyMediaView> {
    let (scope, media) = scoped_media(kernel.db.pool(), actor, ids.0, ids.1, true, admin).await?;
    service::update(kernel.db.pool(), actor, scope.owner(), &media, input).await
}

async fn remove_for(
    kernel: &Kernel,
    actor: &Actor,
    ids: (&str, &str),
    input: RemoveMediaInput,
    admin: bool,
) -> Result<PropertyMediaRemoved> {
    let (scope, media) = scoped_media(kernel.db.pool(), actor, ids.0, ids.1, true, admin).await?;
    service::remove(kernel.db.pool(), &kernel.storage, actor, scope.owner(), &media, input).await
}

async fn content_for(kernel: &Kernel, actor: &Actor, media_id: &str, admin: bool) -> Result<PropertyMediaContent> {
    let media = scoped_media_by_id(kernel.db.pool(), actor, media_id, admin).await?;
    service::content(kernel.db.pool(), &kernel.storage, MediaKind::Property, media).await
}

#[utoipa::path(
    get, path = "/api/host/properties/{propertyId}/media", operation_id = "host_property_media_list",
    params(("propertyId" = String, Path)),
    responses((status = 200, body = Envelope<Vec<PropertyMediaView>>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn host_list<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path(id): Path<String>) -> Reply<Vec<PropertyMediaView>>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(list_for(state.kernel(), &actor, &id, false).await?, rid)))
}

#[utoipa::path(
    post, path = "/api/host/properties/{propertyId}/media", operation_id = "host_property_media_add",
    params(("propertyId" = String, Path)), request_body = AddMediaInput,
    responses((status = 200, body = Envelope<PropertyMediaView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn host_add<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path(id): Path<String>, JsonBody(input): JsonBody<AddMediaInput>) -> Reply<PropertyMediaView>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(add_for(state.kernel(), &actor, &id, input, false).await?, rid)))
}

#[utoipa::path(
    patch, path = "/api/host/properties/{propertyId}/media/{mediaId}", operation_id = "host_property_media_update",
    params(("propertyId" = String, Path), ("mediaId" = String, Path)), request_body = UpdateMediaInput,
    responses((status = 200, body = Envelope<PropertyMediaView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn host_update<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path((id, media_id)): Path<(String, String)>, JsonBody(input): JsonBody<UpdateMediaInput>) -> Reply<PropertyMediaView>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(update_for(state.kernel(), &actor, (&id, &media_id), input, false).await?, rid)))
}

#[utoipa::path(
    delete, path = "/api/host/properties/{propertyId}/media/{mediaId}", operation_id = "host_property_media_remove",
    params(("propertyId" = String, Path), ("mediaId" = String, Path)), request_body = RemoveMediaInput,
    responses((status = 200, body = Envelope<PropertyMediaRemoved>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn host_remove<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path((id, media_id)): Path<(String, String)>, JsonBody(input): JsonBody<RemoveMediaInput>) -> Reply<PropertyMediaRemoved>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(remove_for(state.kernel(), &actor, (&id, &media_id), input, false).await?, rid)))
}

#[utoipa::path(
    get, path = "/api/host/media/{mediaId}/content", operation_id = "host_property_media_content",
    params(("mediaId" = String, Path)),
    responses((status = 200, body = Envelope<PropertyMediaContent>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn host_content<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path(media_id): Path<String>) -> Reply<PropertyMediaContent>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(content_for(state.kernel(), &actor, &media_id, false).await?, rid)))
}

#[utoipa::path(
    get, path = "/api/admin/properties/{propertyId}/media", operation_id = "admin_property_media_list",
    params(("propertyId" = String, Path)),
    responses((status = 200, body = Envelope<Vec<PropertyMediaView>>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn admin_list<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path(id): Path<String>) -> Reply<Vec<PropertyMediaView>>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(list_for(state.kernel(), &actor, &id, true).await?, rid)))
}

#[utoipa::path(
    post, path = "/api/admin/properties/{propertyId}/media", operation_id = "admin_property_media_add",
    params(("propertyId" = String, Path)), request_body = AddMediaInput,
    responses((status = 200, body = Envelope<PropertyMediaView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn admin_add<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path(id): Path<String>, JsonBody(input): JsonBody<AddMediaInput>) -> Reply<PropertyMediaView>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(add_for(state.kernel(), &actor, &id, input, true).await?, rid)))
}

#[utoipa::path(
    patch, path = "/api/admin/properties/{propertyId}/media/{mediaId}", operation_id = "admin_property_media_update",
    params(("propertyId" = String, Path), ("mediaId" = String, Path)), request_body = UpdateMediaInput,
    responses((status = 200, body = Envelope<PropertyMediaView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn admin_update<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path((id, media_id)): Path<(String, String)>, JsonBody(input): JsonBody<UpdateMediaInput>) -> Reply<PropertyMediaView>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(update_for(state.kernel(), &actor, (&id, &media_id), input, true).await?, rid)))
}

#[utoipa::path(
    delete, path = "/api/admin/properties/{propertyId}/media/{mediaId}", operation_id = "admin_property_media_remove",
    params(("propertyId" = String, Path), ("mediaId" = String, Path)), request_body = RemoveMediaInput,
    responses((status = 200, body = Envelope<PropertyMediaRemoved>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn admin_remove<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path((id, media_id)): Path<(String, String)>, JsonBody(input): JsonBody<RemoveMediaInput>) -> Reply<PropertyMediaRemoved>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(remove_for(state.kernel(), &actor, (&id, &media_id), input, true).await?, rid)))
}

#[utoipa::path(
    get, path = "/api/admin/media/{mediaId}/content", operation_id = "admin_property_media_content",
    params(("mediaId" = String, Path)),
    responses((status = 200, body = Envelope<PropertyMediaContent>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn admin_content<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path(media_id): Path<String>) -> Reply<PropertyMediaContent>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(content_for(state.kernel(), &actor, &media_id, true).await?, rid)))
}
