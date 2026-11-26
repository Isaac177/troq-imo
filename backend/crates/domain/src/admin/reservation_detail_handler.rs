use axum::extract::rejection::QueryRejection;
use axum::extract::{Path, Query, State};
use axum::Json;
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{codes, Error, ErrorBody, Result};
use kernel::KernelState;

use super::reservation_detail;
use super::reservation_detail_types::{Detail, DetailPage};

#[utoipa::path(
    get, path = "/api/admin/reservations/{reservationId}", operation_id = "admin_reservations_detail",
    params(("reservationId" = String, Path), DetailPage),
    responses((status = 200, body = Envelope<Detail>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn get<S>(
    State(state): State<S>, actor: Actor, RequestId(request_id): RequestId, Path(reservation_id): Path<String>,
    query: std::result::Result<Query<DetailPage>, QueryRejection>,
) -> Result<Json<Envelope<Detail>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let Query(query) = query.map_err(|_| Error::bad_request(codes::MALFORMED_REQUEST))?;
    Ok(Json(Envelope::ok(reservation_detail::get(state.kernel(), &actor, &reservation_id, query).await?, request_id)))
}
