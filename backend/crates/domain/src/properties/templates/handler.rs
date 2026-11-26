use axum::extract::{Path, State};
use axum::Json;
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{ErrorBody, Result};
use kernel::kernel::KernelState;
use kernel::paging::Page;

use super::service;
use super::types::{TemplateInput, TemplateListQuery, TemplateView};
use crate::properties::http::{paging, JsonBody, Params};

type Reply<T> = Result<Json<Envelope<T>>>;

#[utoipa::path(
    get, path = "/api/host/accommodation-templates", operation_id = "host_accommodation_templates_list",
    params(TemplateListQuery),
    responses((status = 200, body = Envelope<Page<TemplateView>>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn host_list<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Params(query): Params<TemplateListQuery>) -> Reply<Page<TemplateView>>
where S: KernelState + Clone + Send + Sync + 'static {
    let kernel = state.kernel();
    let page = paging(query.page, query.page_size)?;
    Ok(Json(Envelope::ok(service::list(kernel.db.pool(), &kernel.storage, &actor, &query, page, false).await?, rid)))
}

#[utoipa::path(
    post, path = "/api/host/accommodation-templates", operation_id = "host_accommodation_templates_create",
    request_body = TemplateInput,
    responses((status = 200, body = Envelope<TemplateView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn host_create<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, JsonBody(input): JsonBody<TemplateInput>) -> Reply<TemplateView>
where S: KernelState + Clone + Send + Sync + 'static {
    let kernel = state.kernel();
    Ok(Json(Envelope::ok(service::create(kernel.db.pool(), &kernel.storage, &actor, input, false).await?, rid)))
}

#[utoipa::path(
    patch, path = "/api/host/accommodation-templates/{templateId}", operation_id = "host_accommodation_templates_update",
    params(("templateId" = String, Path)), request_body = TemplateInput,
    responses((status = 200, body = Envelope<TemplateView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn host_update<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path(id): Path<String>, JsonBody(input): JsonBody<TemplateInput>) -> Reply<TemplateView>
where S: KernelState + Clone + Send + Sync + 'static {
    let kernel = state.kernel();
    Ok(Json(Envelope::ok(service::update(kernel.db.pool(), &kernel.storage, &actor, &id, input, false).await?, rid)))
}

#[utoipa::path(
    get, path = "/api/admin/accommodation-templates", operation_id = "admin_accommodation_templates_list",
    params(TemplateListQuery),
    responses((status = 200, body = Envelope<Page<TemplateView>>), (status = 403, body = ErrorBody))
)]
pub async fn admin_list<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Params(query): Params<TemplateListQuery>) -> Reply<Page<TemplateView>>
where S: KernelState + Clone + Send + Sync + 'static {
    let kernel = state.kernel();
    let page = paging(query.page, query.page_size)?;
    Ok(Json(Envelope::ok(service::list(kernel.db.pool(), &kernel.storage, &actor, &query, page, true).await?, rid)))
}

#[utoipa::path(
    post, path = "/api/admin/accommodation-templates", operation_id = "admin_accommodation_templates_create",
    request_body = TemplateInput,
    responses((status = 200, body = Envelope<TemplateView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn admin_create<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, JsonBody(input): JsonBody<TemplateInput>) -> Reply<TemplateView>
where S: KernelState + Clone + Send + Sync + 'static {
    let kernel = state.kernel();
    Ok(Json(Envelope::ok(service::create(kernel.db.pool(), &kernel.storage, &actor, input, true).await?, rid)))
}

#[utoipa::path(
    patch, path = "/api/admin/accommodation-templates/{templateId}", operation_id = "admin_accommodation_templates_update",
    params(("templateId" = String, Path)), request_body = TemplateInput,
    responses((status = 200, body = Envelope<TemplateView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn admin_update<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path(id): Path<String>, JsonBody(input): JsonBody<TemplateInput>) -> Reply<TemplateView>
where S: KernelState + Clone + Send + Sync + 'static {
    let kernel = state.kernel();
    Ok(Json(Envelope::ok(service::update(kernel.db.pool(), &kernel.storage, &actor, &id, input, true).await?, rid)))
}
