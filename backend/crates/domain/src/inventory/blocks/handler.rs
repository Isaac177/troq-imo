use axum::extract::{Path, State};
use axum::Json;
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{Error, ErrorBody, Result};
use kernel::kernel::KernelState;
use kernel::paging::Page;

use super::service;
use super::types::{BlockListQuery, BlockView, CreateBlockInput, RemoveBlockInput};
use crate::inventory::http::{JsonBody, Params};
use crate::inventory::messages::PROPERTY_NOT_FOUND;
use crate::inventory::query::paging;

fn required_property(query: &BlockListQuery) -> Result<&str> {
    query
        .property_id
        .as_deref()
        .ok_or_else(|| Error::not_found(PROPERTY_NOT_FOUND))
}

#[utoipa::path(
    post, path = "/api/host/blocks", operation_id = "inventory_blocks_create",
    request_body = CreateBlockInput,
    responses(
        (status = 200, body = Envelope<BlockView>),
        (status = 400, body = ErrorBody),
        (status = 403, body = ErrorBody),
        (status = 404, body = ErrorBody),
        (status = 409, body = ErrorBody)
    )
)]
pub async fn host_create<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    JsonBody(input): JsonBody<CreateBlockInput>,
) -> Result<Json<Envelope<BlockView>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let view = service::create(state.kernel().db.pool(), &actor, input, false).await?;
    Ok(Json(Envelope::ok(view, request_id)))
}

#[utoipa::path(
    get, path = "/api/host/properties/{propertyId}/blocks", operation_id = "inventory_blocks_list",
    params(("propertyId" = String, Path), BlockListQuery),
    responses((status = 200, body = Envelope<Page<BlockView>>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn host_list<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Path(property_id): Path<String>,
    Params(query): Params<BlockListQuery>,
) -> Result<Json<Envelope<Page<BlockView>>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let page = paging(query.page, query.page_size)?;
    let result = service::list(state.kernel().db.pool(), &actor, &property_id, page, false).await?;
    Ok(Json(Envelope::ok(result, request_id)))
}

#[utoipa::path(
    get, path = "/api/host/blocks/{blockId}", operation_id = "inventory_blocks_get",
    params(("blockId" = String, Path)),
    responses((status = 200, body = Envelope<BlockView>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn host_get<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Path(block_id): Path<String>,
) -> Result<Json<Envelope<BlockView>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let view = service::get(state.kernel().db.pool(), &actor, &block_id).await?;
    Ok(Json(Envelope::ok(view, request_id)))
}

#[utoipa::path(
    delete, path = "/api/host/blocks/{blockId}", operation_id = "inventory_blocks_remove",
    params(("blockId" = String, Path)), request_body = RemoveBlockInput,
    responses((status = 200, body = Envelope<BlockView>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn host_remove<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Path(block_id): Path<String>,
    JsonBody(input): JsonBody<RemoveBlockInput>,
) -> Result<Json<Envelope<BlockView>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let view = service::remove(state.kernel().db.pool(), &actor, &block_id, input, false).await?;
    Ok(Json(Envelope::ok(view, request_id)))
}

#[utoipa::path(
    get, path = "/api/admin/blocks", operation_id = "admin_inventory_blocks_list",
    params(BlockListQuery),
    responses((status = 200, body = Envelope<Page<BlockView>>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn admin_list<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Params(query): Params<BlockListQuery>,
) -> Result<Json<Envelope<Page<BlockView>>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let property_id = required_property(&query)?;
    let page = paging(query.page, query.page_size)?;
    let result = service::list(state.kernel().db.pool(), &actor, property_id, page, true).await?;
    Ok(Json(Envelope::ok(result, request_id)))
}

#[utoipa::path(
    post, path = "/api/admin/blocks", operation_id = "admin_inventory_blocks_create",
    request_body = CreateBlockInput,
    responses((status = 200, body = Envelope<BlockView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn admin_create<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    JsonBody(input): JsonBody<CreateBlockInput>,
) -> Result<Json<Envelope<BlockView>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let view = service::create(state.kernel().db.pool(), &actor, input, true).await?;
    Ok(Json(Envelope::ok(view, request_id)))
}

#[utoipa::path(
    post, path = "/api/admin/blocks/{blockId}/remove", operation_id = "admin_inventory_blocks_remove",
    params(("blockId" = String, Path)), request_body = RemoveBlockInput,
    responses((status = 200, body = Envelope<BlockView>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn admin_remove<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Path(block_id): Path<String>,
    JsonBody(input): JsonBody<RemoveBlockInput>,
) -> Result<Json<Envelope<BlockView>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let view = service::remove(state.kernel().db.pool(), &actor, &block_id, input, true).await?;
    Ok(Json(Envelope::ok(view, request_id)))
}
