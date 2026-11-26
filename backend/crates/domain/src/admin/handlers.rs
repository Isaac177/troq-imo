use axum::extract::rejection::QueryRejection;
use axum::extract::{Path, Query, State};
use axum::Json;
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{codes, Error, ErrorBody, Result};
use kernel::{KernelState, Page};

use super::guest_bookings::{self, GuestBooking, GuestBookingsQuery};

#[utoipa::path(
    get,
    path = "/api/admin/guests/{userId}/bookings",
    operation_id = "admin_guest_bookings",
    params(("userId" = String, Path), GuestBookingsQuery),
    responses(
        (status = 200, body = Envelope<Page<GuestBooking>>),
        (status = 400, body = ErrorBody),
        (status = 403, body = ErrorBody)
    )
)]
pub async fn guest_bookings<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Path(user_id): Path<String>,
    query: std::result::Result<Query<GuestBookingsQuery>, QueryRejection>,
) -> Result<Json<Envelope<Page<GuestBooking>>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let Query(query) = query.map_err(|_| Error::bad_request(codes::MALFORMED_REQUEST))?;
    let page = guest_bookings::list(state.kernel(), &actor, &user_id, query).await?;
    Ok(Json(Envelope::ok(page, request_id)))
}
