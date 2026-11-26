use axum::extract::{Path, State};
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::ErrorBody;
use kernel::{Kernel, KernelState, Page};

use super::extract::{reply, AppState, Body, Params, Reply};
use crate::reservations::amendments::list::list_amendments;
use crate::reservations::amendments::propose::propose_amendment;
use crate::reservations::amendments::respond::{accept_amendment, decline_amendment};
use crate::reservations::amendments::types::{AmendmentListItem, AmendmentListQuery, AmendmentProposal, ProposeInput};
use crate::reservations::lifecycle::cancel_guest::cancel_stay;
use crate::reservations::lifecycle::cancellation::evaluate_cancellation;
use crate::reservations::lifecycle::mine::my_reservations;
use crate::reservations::lifecycle::types::{
    CancelStayInput, CancelledStayView, CancellationView, HoldRequest, MyReservationView, MyReservationsQuery,
    ReservationView, VersionedAction,
};
use crate::reservations::lifecycle::view::require_view;
use crate::reservations::lifecycle::withdraw::withdraw_request;
use crate::reservations::holds::hold::create_hold;
use crate::reservations::messages::service as messages;
use crate::reservations::messages::types::{MessageView, MessagesQuery, SendMessageInput};

pub fn app_kernel<S: KernelState>(state: &S) -> &Kernel {
    state.kernel()
}

#[utoipa::path(
    post, path = "/api/guest/reservations", operation_id = "reservations_hold", request_body = HoldRequest,
    responses((status = 200, body = Envelope<ReservationView>), (status = 409, body = ErrorBody))
)]
pub async fn hold<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Body(input): Body<HoldRequest>) -> Reply<ReservationView> {
    Ok(reply(create_hold(app_kernel(&state), &actor, input).await?, id))
}

#[utoipa::path(
    get, path = "/api/guest/reservations/mine", operation_id = "reservations_mine", params(MyReservationsQuery),
    responses((status = 200, body = Envelope<Page<MyReservationView>>), (status = 400, body = ErrorBody))
)]
pub async fn mine<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Params(query): Params<MyReservationsQuery>) -> Reply<Page<MyReservationView>> {
    Ok(reply(my_reservations(app_kernel(&state), &actor, query).await?, id))
}

#[utoipa::path(
    get, path = "/api/guest/reservations/{reservationId}", operation_id = "reservations_get",
    params(("reservationId" = String, Path)),
    responses((status = 200, body = Envelope<ReservationView>), (status = 404, body = ErrorBody))
)]
pub async fn get<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(reservation_id): Path<String>) -> Reply<ReservationView> {
    let pool = app_kernel(&state).db.pool();
    let owner = crate::reservations::lifecycle::queries::reservation_owner(pool, actor.city, &reservation_id).await?;
    if owner.as_deref() != Some(actor.user_id().as_str()) {
        return Err(kernel::Error::not_found(crate::reservations::codes::RESERVATION_NOT_FOUND));
    }
    Ok(reply(require_view(app_kernel(&state), actor.city, &reservation_id).await?, id))
}

#[utoipa::path(
    get, path = "/api/guest/reservations/{reservationId}/cancellation", operation_id = "reservations_cancellation",
    params(("reservationId" = String, Path)),
    responses((status = 200, body = Envelope<CancellationView>), (status = 409, body = ErrorBody))
)]
pub async fn cancellation<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(reservation_id): Path<String>) -> Reply<CancellationView> {
    Ok(reply(evaluate_cancellation(app_kernel(&state), &actor, &reservation_id).await?, id))
}

#[utoipa::path(
    post, path = "/api/guest/reservations/{reservationId}/cancel", operation_id = "reservations_cancel",
    params(("reservationId" = String, Path)), request_body = CancelStayInput,
    responses((status = 200, body = Envelope<CancelledStayView>), (status = 409, body = ErrorBody))
)]
pub async fn cancel<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(reservation_id): Path<String>, Body(input): Body<CancelStayInput>) -> Reply<CancelledStayView> {
    Ok(reply(cancel_stay(app_kernel(&state), &actor, &reservation_id, input).await?, id))
}

#[utoipa::path(
    post, path = "/api/guest/reservations/{reservationId}/withdraw", operation_id = "reservations_withdraw",
    params(("reservationId" = String, Path)), request_body = VersionedAction,
    responses((status = 200, body = Envelope<ReservationView>), (status = 409, body = ErrorBody))
)]
pub async fn withdraw<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(reservation_id): Path<String>, Body(input): Body<VersionedAction>) -> Reply<ReservationView> {
    Ok(reply(withdraw_request(app_kernel(&state), &actor, &reservation_id, input).await?, id))
}

#[utoipa::path(
    get, path = "/api/guest/reservations/{reservationId}/amendments", operation_id = "reservation_amendments_list",
    params(("reservationId" = String, Path), AmendmentListQuery),
    responses((status = 200, body = Envelope<Page<AmendmentListItem>>), (status = 404, body = ErrorBody))
)]
pub async fn amendments<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(reservation_id): Path<String>, Params(query): Params<AmendmentListQuery>) -> Reply<Page<AmendmentListItem>> {
    Ok(reply(list_amendments(app_kernel(&state), &actor, &reservation_id, query).await?, id))
}

#[utoipa::path(
    post, path = "/api/guest/reservations/{reservationId}/amendments", operation_id = "reservation_amendments_propose",
    params(("reservationId" = String, Path)), request_body = ProposeInput,
    responses((status = 200, body = Envelope<AmendmentProposal>), (status = 409, body = ErrorBody))
)]
pub async fn propose<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(reservation_id): Path<String>, Body(input): Body<ProposeInput>) -> Reply<AmendmentProposal> {
    Ok(reply(propose_amendment(app_kernel(&state), &actor, &reservation_id, input).await?, id))
}

#[utoipa::path(
    post, path = "/api/guest/amendments/{amendmentId}/accept", operation_id = "amendments_accept",
    params(("amendmentId" = String, Path)),
    responses((status = 200, body = Envelope<ReservationView>), (status = 409, body = ErrorBody))
)]
pub async fn accept<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(amendment_id): Path<String>) -> Reply<ReservationView> {
    Ok(reply(accept_amendment(app_kernel(&state), &actor, &amendment_id).await?, id))
}

#[utoipa::path(
    post, path = "/api/guest/amendments/{amendmentId}/decline", operation_id = "amendments_decline",
    params(("amendmentId" = String, Path)),
    responses((status = 200, body = Envelope<ReservationView>), (status = 409, body = ErrorBody))
)]
pub async fn decline<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(amendment_id): Path<String>) -> Reply<ReservationView> {
    Ok(reply(decline_amendment(app_kernel(&state), &actor, &amendment_id).await?, id))
}

#[utoipa::path(
    get, path = "/api/guest/reservations/{reservationId}/messages", operation_id = "reservation_messages_list_guest",
    params(("reservationId" = String, Path), MessagesQuery),
    responses((status = 200, body = Envelope<Page<MessageView>>), (status = 404, body = ErrorBody))
)]
pub async fn messages_list<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(reservation_id): Path<String>, Params(query): Params<MessagesQuery>) -> Reply<Page<MessageView>> {
    Ok(reply(messages::list(app_kernel(&state), &actor, &reservation_id, query).await?, id))
}

#[utoipa::path(
    post, path = "/api/guest/reservations/{reservationId}/messages", operation_id = "reservation_messages_send_guest",
    params(("reservationId" = String, Path)), request_body = SendMessageInput,
    responses((status = 200, body = Envelope<MessageView>), (status = 400, body = ErrorBody))
)]
pub async fn messages_send<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(reservation_id): Path<String>, Body(input): Body<SendMessageInput>) -> Reply<MessageView> {
    Ok(reply(messages::send(app_kernel(&state), &actor, &reservation_id, input).await?, id))
}
