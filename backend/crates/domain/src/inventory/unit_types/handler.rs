use axum::extract::{Path, State};
use axum::Json;
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{ErrorBody, Result};
use kernel::kernel::KernelState;
use kernel::paging::Page;

use super::types::{
    ArchiveUnitTypeQuery, CreateUnitTypeInput, UnitTypeListQuery, UnitTypeView, UpdateUnitTypeInput,
};
use super::{archive, create, list, update};
use crate::inventory::http::{JsonBody, Params};
use crate::inventory::query::paging;

#[utoipa::path(
    get, path = "/api/host/properties/{propertyId}/unit-types", operation_id = "inventory_unit_types_list",
    params(("propertyId" = String, Path), UnitTypeListQuery),
    responses((status = 200, body = Envelope<Page<UnitTypeView>>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn host_list<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Path(property_id): Path<String>,
    Params(query): Params<UnitTypeListQuery>,
) -> Result<Json<Envelope<Page<UnitTypeView>>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let kernel = state.kernel();
    let page = paging(query.page, query.page_size)?;
    let result = list(kernel.db.pool(), &kernel.storage, &actor, &property_id, page, false).await?;
    Ok(Json(Envelope::ok(result, request_id)))
}

#[utoipa::path(
    post, path = "/api/host/properties/{propertyId}/unit-types", operation_id = "inventory_unit_types_create",
    params(("propertyId" = String, Path)), request_body = CreateUnitTypeInput,
    responses((status = 200, body = Envelope<UnitTypeView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn host_create<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Path(property_id): Path<String>,
    JsonBody(input): JsonBody<CreateUnitTypeInput>,
) -> Result<Json<Envelope<UnitTypeView>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let view = create(state.kernel().db.pool(), &actor, &property_id, input, false).await?;
    Ok(Json(Envelope::ok(view, request_id)))
}

#[utoipa::path(
    patch, path = "/api/host/properties/{propertyId}/unit-types/{unitTypeId}", operation_id = "inventory_unit_types_update",
    params(("propertyId" = String, Path), ("unitTypeId" = String, Path)), request_body = UpdateUnitTypeInput,
    responses((status = 200, body = Envelope<UnitTypeView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn host_update<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Path((property_id, unit_type_id)): Path<(String, String)>,
    JsonBody(input): JsonBody<UpdateUnitTypeInput>,
) -> Result<Json<Envelope<UnitTypeView>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let view = update(state.kernel().db.pool(), &actor, &property_id, &unit_type_id, input, false).await?;
    Ok(Json(Envelope::ok(view, request_id)))
}

#[utoipa::path(
    delete, path = "/api/host/properties/{propertyId}/unit-types/{unitTypeId}", operation_id = "inventory_unit_types_archive",
    params(("propertyId" = String, Path), ("unitTypeId" = String, Path), ArchiveUnitTypeQuery),
    responses((status = 200, body = Envelope<UnitTypeView>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn host_archive<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Path((property_id, unit_type_id)): Path<(String, String)>,
    Params(query): Params<ArchiveUnitTypeQuery>,
) -> Result<Json<Envelope<UnitTypeView>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let view = archive(state.kernel().db.pool(), &actor, &property_id, &unit_type_id, query.expected_version, false).await?;
    Ok(Json(Envelope::ok(view, request_id)))
}

#[utoipa::path(
    get, path = "/api/admin/properties/{propertyId}/unit-types", operation_id = "admin_inventory_unit_types_list",
    params(("propertyId" = String, Path), UnitTypeListQuery),
    responses((status = 200, body = Envelope<Page<UnitTypeView>>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn admin_list<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Path(property_id): Path<String>,
    Params(query): Params<UnitTypeListQuery>,
) -> Result<Json<Envelope<Page<UnitTypeView>>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let kernel = state.kernel();
    let page = paging(query.page, query.page_size)?;
    let result = list(kernel.db.pool(), &kernel.storage, &actor, &property_id, page, true).await?;
    Ok(Json(Envelope::ok(result, request_id)))
}

#[utoipa::path(
    post, path = "/api/admin/properties/{propertyId}/unit-types", operation_id = "admin_inventory_unit_types_create",
    params(("propertyId" = String, Path)), request_body = CreateUnitTypeInput,
    responses((status = 200, body = Envelope<UnitTypeView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn admin_create<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Path(property_id): Path<String>,
    JsonBody(input): JsonBody<CreateUnitTypeInput>,
) -> Result<Json<Envelope<UnitTypeView>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let view = create(state.kernel().db.pool(), &actor, &property_id, input, true).await?;
    Ok(Json(Envelope::ok(view, request_id)))
}

#[utoipa::path(
    patch, path = "/api/admin/properties/{propertyId}/unit-types/{unitTypeId}", operation_id = "admin_inventory_unit_types_update",
    params(("propertyId" = String, Path), ("unitTypeId" = String, Path)), request_body = UpdateUnitTypeInput,
    responses((status = 200, body = Envelope<UnitTypeView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn admin_update<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Path((property_id, unit_type_id)): Path<(String, String)>,
    JsonBody(input): JsonBody<UpdateUnitTypeInput>,
) -> Result<Json<Envelope<UnitTypeView>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let view = update(state.kernel().db.pool(), &actor, &property_id, &unit_type_id, input, true).await?;
    Ok(Json(Envelope::ok(view, request_id)))
}

#[utoipa::path(
    delete, path = "/api/admin/properties/{propertyId}/unit-types/{unitTypeId}", operation_id = "admin_inventory_unit_types_archive",
    params(("propertyId" = String, Path), ("unitTypeId" = String, Path), ArchiveUnitTypeQuery),
    responses((status = 200, body = Envelope<UnitTypeView>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn admin_archive<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Path((property_id, unit_type_id)): Path<(String, String)>,
    Params(query): Params<ArchiveUnitTypeQuery>,
) -> Result<Json<Envelope<UnitTypeView>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let view = archive(state.kernel().db.pool(), &actor, &property_id, &unit_type_id, query.expected_version, true).await?;
    Ok(Json(Envelope::ok(view, request_id)))
}
