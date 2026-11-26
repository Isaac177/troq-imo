use axum::extract::{Path, State};
use axum::Json;
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{ErrorBody, Result};
use kernel::kernel::KernelState;

use super::service;
use super::types::{AssignUnitInput, AssignmentView, MoveUnitInput, UnitOptionView};
use crate::inventory::http::JsonBody;

#[utoipa::path(
    post, path = "/api/host/unit-assignments", operation_id = "inventory_assignments_assign",
    request_body = AssignUnitInput,
    responses((status = 200, body = Envelope<AssignmentView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn host_assign<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    JsonBody(input): JsonBody<AssignUnitInput>,
) -> Result<Json<Envelope<AssignmentView>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let view = service::assign(state.kernel().db.pool(), &actor, input).await?;
    Ok(Json(Envelope::ok(view, request_id)))
}

#[utoipa::path(
    post, path = "/api/host/unit-assignments/{assignmentId}/move", operation_id = "inventory_assignments_move",
    params(("assignmentId" = String, Path)), request_body = MoveUnitInput,
    responses((status = 200, body = Envelope<AssignmentView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn host_move<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Path(assignment_id): Path<String>,
    JsonBody(input): JsonBody<MoveUnitInput>,
) -> Result<Json<Envelope<AssignmentView>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let view = service::move_unit(state.kernel().db.pool(), &actor, &assignment_id, input).await?;
    Ok(Json(Envelope::ok(view, request_id)))
}

#[utoipa::path(
    get, path = "/api/host/reservation-items/{itemId}/unit-assignments", operation_id = "inventory_assignments_history",
    params(("itemId" = String, Path)),
    responses((status = 200, body = Envelope<Vec<AssignmentView>>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn host_history<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Path(item_id): Path<String>,
) -> Result<Json<Envelope<Vec<AssignmentView>>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let rows = service::history(state.kernel().db.pool(), &actor, &item_id).await?;
    Ok(Json(Envelope::ok(rows, request_id)))
}

#[utoipa::path(
    get, path = "/api/host/reservation-items/{itemId}/unit-options", operation_id = "inventory_assignments_options",
    params(("itemId" = String, Path)),
    responses((status = 200, body = Envelope<Vec<UnitOptionView>>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn host_options<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Path(item_id): Path<String>,
) -> Result<Json<Envelope<Vec<UnitOptionView>>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let rows = service::unit_options(state.kernel().db.pool(), &actor, &item_id).await?;
    Ok(Json(Envelope::ok(rows, request_id)))
}
