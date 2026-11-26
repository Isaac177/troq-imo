use axum::extract::rejection::QueryRejection;
use axum::extract::{Path, Query, State};
use axum::Json;
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{codes, Error, ErrorBody, Result};
use kernel::{KernelState, Page};

use super::guests::{self, GuestFilter, GuestItem};

#[utoipa::path(
    get, path = "/api/admin/guests", operation_id = "admin_guests_list",
    params(("search" = Option<String>, Query), ("flag" = Option<String>, Query), ("page" = Option<u32>, Query), ("pageSize" = Option<u32>, Query)),
    responses((status = 200, body = Envelope<Page<GuestItem>>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody))
)]
pub async fn list<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(id): RequestId,
    query: std::result::Result<Query<GuestFilter>, QueryRejection>,
) -> Result<Json<Envelope<Page<GuestItem>>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let Query(filter) = query.map_err(|_| Error::bad_request(codes::MALFORMED_REQUEST))?;
    Ok(Json(Envelope::ok(guests::list(state.kernel().db.pool(), &actor, filter).await?, id)))
}

#[utoipa::path(
    get, path = "/api/admin/guests/{userId}", operation_id = "admin_guest_detail",
    params(("userId" = String, Path)),
    responses((status = 200, body = Envelope<GuestItem>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn detail<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(id): RequestId,
    Path(user_id): Path<String>,
) -> Result<Json<Envelope<GuestItem>>>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(guests::detail(state.kernel().db.pool(), &actor, &user_id).await?, id)))
}
