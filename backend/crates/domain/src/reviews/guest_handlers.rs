use axum::extract::rejection::JsonRejection;
use axum::extract::{Json, Path, State};
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{ErrorBody, Result};
use kernel::KernelState;

use super::extract;
use super::submit::submit_review;
use super::types::{ReviewView, SubmitReviewInput};

#[utoipa::path(
    post, path = "/api/guest/reservations/{reservationId}/review", operation_id = "reviews_submit", tag = "reviews",
    params(("reservationId" = String, Path), ("x-city" = Option<String>, Header), ("x-request-id" = Option<String>, Header)),
    request_body = SubmitReviewInput,
    responses(
        (status = 200, body = Envelope<ReviewView>, headers(("x-request-id" = String))),
        (status = 400, body = ErrorBody),
        (status = 404, body = ErrorBody),
        (status = 409, body = ErrorBody)
    )
)]
pub async fn submit<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Path(reservation_id): Path<String>,
    input: std::result::Result<Json<SubmitReviewInput>, JsonRejection>,
) -> Result<Json<Envelope<ReviewView>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let input = extract::body(input)?;
    let view = submit_review(state.kernel(), &actor, &reservation_id, input).await?;
    Ok(Json(Envelope::ok(view, request_id)))
}
