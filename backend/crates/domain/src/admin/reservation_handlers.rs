use axum::extract::rejection::QueryRejection;
use axum::extract::{Query, State};
use axum::Json;
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{codes, Error, ErrorBody, Result};
use kernel::{KernelState, Page};

use super::reservation_list;
use super::reservation_types::{ReservationFilter, ReservationRow, ReservationSummary};

#[utoipa::path(
    get, path = "/api/admin/reservations", operation_id = "admin_reservations_list",
    params(ReservationFilter),
    responses((status = 200, body = Envelope<Page<ReservationRow>>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody))
)]
pub async fn list<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    query: std::result::Result<Query<ReservationFilter>, QueryRejection>,
) -> Result<Json<Envelope<Page<ReservationRow>>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let Query(query) = query.map_err(|_| Error::bad_request(codes::MALFORMED_REQUEST))?;
    Ok(Json(Envelope::ok(reservation_list::list(state.kernel(), &actor, query).await?, id)))
}

#[utoipa::path(
    get, path = "/api/admin/reservations/summary", operation_id = "admin_reservations_summary",
    params(ReservationFilter),
    responses((status = 200, body = Envelope<ReservationSummary>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody))
)]
pub async fn summary<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    query: std::result::Result<Query<ReservationFilter>, QueryRejection>,
) -> Result<Json<Envelope<ReservationSummary>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let Query(query) = query.map_err(|_| Error::bad_request(codes::MALFORMED_REQUEST))?;
    Ok(Json(Envelope::ok(reservation_list::summary(state.kernel(), &actor, query).await?, id)))
}

#[utoipa::path(
    get, path = "/api/admin/reservations/export", operation_id = "admin_reservations_export",
    params(ReservationFilter),
    responses((status = 200, body = Envelope<Page<ReservationRow>>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody))
)]
pub async fn export<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    query: std::result::Result<Query<ReservationFilter>, QueryRejection>,
) -> Result<Json<Envelope<Page<ReservationRow>>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let Query(query) = query.map_err(|_| Error::bad_request(codes::MALFORMED_REQUEST))?;
    Ok(Json(Envelope::ok(reservation_list::export(state.kernel(), &actor, query).await?, id)))
}
