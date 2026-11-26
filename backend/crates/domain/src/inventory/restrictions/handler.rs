use axum::extract::{Path, State};
use axum::Json;
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{ErrorBody, Result};
use kernel::kernel::KernelState;
use kernel::paging::Page;
use kernel::permissions::names::{INVENTORY_WRITE, PROPERTIES_READ};

use super::service::list_page;
use super::types::{BulkRestrictionInput, BulkRestrictionResult, RestrictionListQuery, RestrictionView};
use super::bulk::apply_bulk_edit;
use crate::inventory::http::{JsonBody, Params};
use crate::inventory::query::paging;
use crate::inventory::scope::{admin_property, host_property};

#[utoipa::path(
    get, path = "/api/host/properties/{propertyId}/restrictions", operation_id = "inventory_restrictions_list",
    params(("propertyId" = String, Path), RestrictionListQuery),
    responses(
        (status = 200, body = Envelope<Page<RestrictionView>>),
        (status = 403, body = ErrorBody),
        (status = 404, body = ErrorBody)
    )
)]
pub async fn host_list<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Path(property_id): Path<String>,
    Params(query): Params<RestrictionListQuery>,
) -> Result<Json<Envelope<Page<RestrictionView>>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let pool = state.kernel().db.pool();
    let property = host_property(pool, &actor, &property_id, PROPERTIES_READ).await?;
    let page = paging(query.page, query.page_size)?;
    Ok(Json(Envelope::ok(list_page(pool, &property.id, page).await?, request_id)))
}

#[utoipa::path(
    post, path = "/api/host/restrictions/bulk", operation_id = "inventory_restrictions_bulk",
    request_body = BulkRestrictionInput,
    responses(
        (status = 200, body = Envelope<BulkRestrictionResult>),
        (status = 400, body = ErrorBody),
        (status = 403, body = ErrorBody),
        (status = 404, body = ErrorBody),
        (status = 409, body = ErrorBody)
    )
)]
pub async fn host_bulk<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    JsonBody(input): JsonBody<BulkRestrictionInput>,
) -> Result<Json<Envelope<BulkRestrictionResult>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let pool = state.kernel().db.pool();
    let property = host_property(pool, &actor, &input.property_id, INVENTORY_WRITE).await?;
    let result = apply_bulk_edit(pool, &property, &actor.audit(), input).await?;
    Ok(Json(Envelope::ok(result, request_id)))
}

#[utoipa::path(
    post, path = "/api/admin/restrictions/bulk", operation_id = "admin_inventory_restrictions_bulk",
    request_body = BulkRestrictionInput,
    responses(
        (status = 200, body = Envelope<BulkRestrictionResult>),
        (status = 400, body = ErrorBody),
        (status = 403, body = ErrorBody),
        (status = 404, body = ErrorBody),
        (status = 409, body = ErrorBody)
    )
)]
pub async fn admin_bulk<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    JsonBody(input): JsonBody<BulkRestrictionInput>,
) -> Result<Json<Envelope<BulkRestrictionResult>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let pool = state.kernel().db.pool();
    let property = admin_property(pool, &actor, &input.property_id).await?;
    let result = apply_bulk_edit(pool, &property, &actor.audit(), input).await?;
    Ok(Json(Envelope::ok(result, request_id)))
}
