use axum::extract::{Path, State};
use axum::Json;
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{ErrorBody, Result};
use kernel::kernel::KernelState;
use kernel::paging::Page;

use super::service;
use super::types::{CategoryInput, CategoryListQuery, CategoryView};
use crate::properties::http::{paging, JsonBody, Params};

#[utoipa::path(
    get, path = "/api/public/accommodation-categories", operation_id = "public_accommodation_categories",
    responses((status = 200, body = Envelope<Vec<CategoryView>>))
)]
pub async fn public_catalog<S>(State(state): State<S>, RequestId(request_id): RequestId) -> Result<Json<Envelope<Vec<CategoryView>>>>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(service::catalog(state.kernel().db.pool()).await?, request_id)))
}

#[utoipa::path(
    get, path = "/api/admin/accommodation-categories", operation_id = "admin_accommodation_categories_list",
    params(CategoryListQuery),
    responses((status = 200, body = Envelope<Page<CategoryView>>), (status = 403, body = ErrorBody))
)]
pub async fn admin_list<S>(
    State(state): State<S>, actor: Actor, RequestId(request_id): RequestId, Params(query): Params<CategoryListQuery>,
) -> Result<Json<Envelope<Page<CategoryView>>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let page = paging(query.page, query.page_size)?;
    Ok(Json(Envelope::ok(service::list(state.kernel().db.pool(), &actor, &query, page).await?, request_id)))
}

#[utoipa::path(
    post, path = "/api/admin/accommodation-categories", operation_id = "admin_accommodation_categories_create",
    request_body = CategoryInput,
    responses((status = 200, body = Envelope<CategoryView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn admin_create<S>(
    State(state): State<S>, actor: Actor, RequestId(request_id): RequestId, JsonBody(input): JsonBody<CategoryInput>,
) -> Result<Json<Envelope<CategoryView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(service::create(state.kernel().db.pool(), &actor, input).await?, request_id)))
}

#[utoipa::path(
    patch, path = "/api/admin/accommodation-categories/{code}", operation_id = "admin_accommodation_categories_update",
    params(("code" = String, Path)), request_body = CategoryInput,
    responses((status = 200, body = Envelope<CategoryView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn admin_update<S>(
    State(state): State<S>, actor: Actor, RequestId(request_id): RequestId,
    Path(code): Path<String>, JsonBody(input): JsonBody<CategoryInput>,
) -> Result<Json<Envelope<CategoryView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(service::update(state.kernel().db.pool(), &actor, &code, input).await?, request_id)))
}
