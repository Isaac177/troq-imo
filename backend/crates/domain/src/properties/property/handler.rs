use axum::extract::{Path, State};
use axum::Json;
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{ErrorBody, Result};
use kernel::kernel::KernelState;
use kernel::paging::Page;

use super::service;
use super::types::{CreatePropertyInput, PropertyListQuery, PropertyView};
use crate::properties::http::{paging, JsonBody, Params};

#[utoipa::path(
    get, path = "/api/host/properties", operation_id = "host_properties_list",
    params(PropertyListQuery),
    responses((status = 200, body = Envelope<Page<PropertyView>>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody))
)]
pub async fn list<S>(
    State(state): State<S>, actor: Actor, RequestId(request_id): RequestId,
    Params(query): Params<PropertyListQuery>,
) -> Result<Json<Envelope<Page<PropertyView>>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let page = paging(query.page, query.page_size)?;
    let result = service::list(state.kernel().db.pool(), &actor, &query, page).await?;
    Ok(Json(Envelope::ok(result, request_id)))
}

#[utoipa::path(
    get, path = "/api/host/properties/{propertyId}", operation_id = "host_properties_get",
    params(("propertyId" = String, Path)),
    responses((status = 200, body = Envelope<PropertyView>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn get<S>(
    State(state): State<S>, actor: Actor, RequestId(request_id): RequestId,
    Path(property_id): Path<String>,
) -> Result<Json<Envelope<PropertyView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let result = service::get(state.kernel().db.pool(), &actor, &property_id).await?;
    Ok(Json(Envelope::ok(result, request_id)))
}

#[utoipa::path(
    post, path = "/api/host/properties", operation_id = "host_properties_create",
    request_body = CreatePropertyInput,
    responses((status = 200, body = Envelope<PropertyView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn create<S>(
    State(state): State<S>, actor: Actor, RequestId(request_id): RequestId,
    JsonBody(input): JsonBody<CreatePropertyInput>,
) -> Result<Json<Envelope<PropertyView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let result = service::create(state.kernel().db.pool(), &actor, input).await?;
    Ok(Json(Envelope::ok(result, request_id)))
}
