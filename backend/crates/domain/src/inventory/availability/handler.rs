use axum::extract::State;
use axum::Json;
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{ErrorBody, Result};
use kernel::kernel::KernelState;

use super::service::get_availability;
use super::types::{AvailabilityQuery, AvailabilityView};
use crate::inventory::http::Params;

#[utoipa::path(
    get, path = "/api/host/availability", operation_id = "inventory_availability_get",
    params(AvailabilityQuery),
    responses((status = 200, body = Envelope<AvailabilityView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn host_get<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Params(query): Params<AvailabilityQuery>,
) -> Result<Json<Envelope<AvailabilityView>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let view = get_availability(state.kernel().db.pool(), &actor, query).await?;
    Ok(Json(Envelope::ok(view, request_id)))
}
