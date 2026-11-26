use axum::extract::{Path, State};
use axum::Json;
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{ErrorBody, Result};
use kernel::kernel::{Kernel, KernelState};

use super::access::{scoped, scoped_media};
use crate::properties::http::JsonBody;
use crate::properties::media::kind::MediaKind;
use crate::properties::media::service;
use crate::properties::media::types::{
    AddMediaInput, MediaContent, MediaRemoved, MediaView, RemoveMediaInput, UpdateMediaInput,
};

type Reply<T> = Result<Json<Envelope<T>>>;

async fn list_for(kernel: &Kernel, actor: &Actor, id: &str, admin: bool) -> Result<Vec<MediaView>> {
    let scope = scoped(kernel.db.pool(), actor, id, false, admin).await?;
    service::list(kernel.db.pool(), scope.owner()).await
}

async fn add_for(kernel: &Kernel, actor: &Actor, id: &str, input: AddMediaInput, admin: bool) -> Result<MediaView> {
    let scope = scoped(kernel.db.pool(), actor, id, true, admin).await?;
    service::add(kernel.db.pool(), &kernel.storage, actor, scope.owner(), input).await
}

async fn update_for(kernel: &Kernel, actor: &Actor, ids: (&str, &str), input: UpdateMediaInput, admin: bool) -> Result<MediaView> {
    let (scope, media) = scoped_media(kernel.db.pool(), actor, ids.0, ids.1, true, admin).await?;
    service::update(kernel.db.pool(), actor, scope.owner(), &media, input).await
}

async fn remove_for(kernel: &Kernel, actor: &Actor, ids: (&str, &str), input: RemoveMediaInput, admin: bool) -> Result<MediaRemoved> {
    let (scope, media) = scoped_media(kernel.db.pool(), actor, ids.0, ids.1, true, admin).await?;
    service::remove(kernel.db.pool(), &kernel.storage, actor, scope.owner(), &media, input).await
}

async fn content_for(kernel: &Kernel, actor: &Actor, ids: (&str, &str), admin: bool) -> Result<MediaContent> {
    let (_, media) = scoped_media(kernel.db.pool(), actor, ids.0, ids.1, false, admin).await?;
    service::content(kernel.db.pool(), &kernel.storage, MediaKind::Template, media).await
}

#[utoipa::path(
    get, path = "/api/host/accommodation-templates/{templateId}/media", operation_id = "host_template_media_list",
    params(("templateId" = String, Path)),
    responses((status = 200, body = Envelope<Vec<MediaView>>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn host_list<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path(id): Path<String>) -> Reply<Vec<MediaView>>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(list_for(state.kernel(), &actor, &id, false).await?, rid)))
}

#[utoipa::path(
    post, path = "/api/host/accommodation-templates/{templateId}/media", operation_id = "host_template_media_add",
    params(("templateId" = String, Path)), request_body = AddMediaInput,
    responses((status = 200, body = Envelope<MediaView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn host_add<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path(id): Path<String>, JsonBody(input): JsonBody<AddMediaInput>) -> Reply<MediaView>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(add_for(state.kernel(), &actor, &id, input, false).await?, rid)))
}

#[utoipa::path(
    patch, path = "/api/host/accommodation-templates/{templateId}/media/{mediaId}", operation_id = "host_template_media_update",
    params(("templateId" = String, Path), ("mediaId" = String, Path)), request_body = UpdateMediaInput,
    responses((status = 200, body = Envelope<MediaView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn host_update<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path((id, media_id)): Path<(String, String)>, JsonBody(input): JsonBody<UpdateMediaInput>) -> Reply<MediaView>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(update_for(state.kernel(), &actor, (&id, &media_id), input, false).await?, rid)))
}

#[utoipa::path(
    delete, path = "/api/host/accommodation-templates/{templateId}/media/{mediaId}", operation_id = "host_template_media_remove",
    params(("templateId" = String, Path), ("mediaId" = String, Path)), request_body = RemoveMediaInput,
    responses((status = 200, body = Envelope<MediaRemoved>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn host_remove<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path((id, media_id)): Path<(String, String)>, JsonBody(input): JsonBody<RemoveMediaInput>) -> Reply<MediaRemoved>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(remove_for(state.kernel(), &actor, (&id, &media_id), input, false).await?, rid)))
}

#[utoipa::path(
    get, path = "/api/host/accommodation-templates/{templateId}/media/{mediaId}/content", operation_id = "host_template_media_content",
    params(("templateId" = String, Path), ("mediaId" = String, Path)),
    responses((status = 200, body = Envelope<MediaContent>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn host_content<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path((id, media_id)): Path<(String, String)>) -> Reply<MediaContent>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(content_for(state.kernel(), &actor, (&id, &media_id), false).await?, rid)))
}

#[utoipa::path(
    get, path = "/api/admin/accommodation-templates/{templateId}/media", operation_id = "admin_template_media_list",
    params(("templateId" = String, Path)),
    responses((status = 200, body = Envelope<Vec<MediaView>>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn admin_list<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path(id): Path<String>) -> Reply<Vec<MediaView>>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(list_for(state.kernel(), &actor, &id, true).await?, rid)))
}

#[utoipa::path(
    post, path = "/api/admin/accommodation-templates/{templateId}/media", operation_id = "admin_template_media_add",
    params(("templateId" = String, Path)), request_body = AddMediaInput,
    responses((status = 200, body = Envelope<MediaView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn admin_add<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path(id): Path<String>, JsonBody(input): JsonBody<AddMediaInput>) -> Reply<MediaView>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(add_for(state.kernel(), &actor, &id, input, true).await?, rid)))
}

#[utoipa::path(
    patch, path = "/api/admin/accommodation-templates/{templateId}/media/{mediaId}", operation_id = "admin_template_media_update",
    params(("templateId" = String, Path), ("mediaId" = String, Path)), request_body = UpdateMediaInput,
    responses((status = 200, body = Envelope<MediaView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn admin_update<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path((id, media_id)): Path<(String, String)>, JsonBody(input): JsonBody<UpdateMediaInput>) -> Reply<MediaView>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(update_for(state.kernel(), &actor, (&id, &media_id), input, true).await?, rid)))
}

#[utoipa::path(
    delete, path = "/api/admin/accommodation-templates/{templateId}/media/{mediaId}", operation_id = "admin_template_media_remove",
    params(("templateId" = String, Path), ("mediaId" = String, Path)), request_body = RemoveMediaInput,
    responses((status = 200, body = Envelope<MediaRemoved>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn admin_remove<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path((id, media_id)): Path<(String, String)>, JsonBody(input): JsonBody<RemoveMediaInput>) -> Reply<MediaRemoved>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(remove_for(state.kernel(), &actor, (&id, &media_id), input, true).await?, rid)))
}

#[utoipa::path(
    get, path = "/api/admin/accommodation-templates/{templateId}/media/{mediaId}/content", operation_id = "admin_template_media_content",
    params(("templateId" = String, Path), ("mediaId" = String, Path)),
    responses((status = 200, body = Envelope<MediaContent>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn admin_content<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path((id, media_id)): Path<(String, String)>) -> Reply<MediaContent>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(content_for(state.kernel(), &actor, (&id, &media_id), true).await?, rid)))
}
