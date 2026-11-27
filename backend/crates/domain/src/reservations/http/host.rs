use axum::extract::{Path, State};
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::ErrorBody;
use kernel::Page;

use super::extract::{reply, AppState, Body, Params, Reply};
use super::guest::app_kernel;
use crate::reservations::amendments::propose::propose_operator_amendment;
use crate::reservations::amendments::types::{AmendmentProposal, ProposeInput};
use crate::reservations::lifecycle::decision::decide_request;
use crate::reservations::lifecycle::operator_cancel::operator_cancel;
use crate::reservations::lifecycle::operator_list::{export_operator_reservations, get_operator_reservation, list_operator_reservations};
use crate::reservations::lifecycle::special_request::decide_special_request;
use crate::reservations::lifecycle::stay_flow::{transition, StayAction};
use crate::reservations::lifecycle::types::{
    DecisionInput, HostLocaleQuery, OperatorCancelInput, OperatorReservationQuery, OperatorReservationView,
    ReservationView, SpecialRequestDecisionInput, VersionedAction,
};
use crate::reservations::messages::service as messages;
use crate::reservations::messages::types::{MessageView, MessagesQuery, SendMessageInput};

#[utoipa::path(
    get, path = "/api/host/reservations", operation_id = "host_reservations_list", params(OperatorReservationQuery),
    responses((status = 200, body = Envelope<Page<OperatorReservationView>>), (status = 404, body = ErrorBody))
)]
pub async fn list<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Params(query): Params<OperatorReservationQuery>) -> Reply<Page<OperatorReservationView>> {
    Ok(reply(list_operator_reservations(app_kernel(&state), &actor, query).await?, id))
}

#[utoipa::path(
    get, path = "/api/host/reservations/export", operation_id = "host_reservations_export", params(OperatorReservationQuery),
    responses((status = 200, body = Envelope<Page<OperatorReservationView>>), (status = 404, body = ErrorBody))
)]
pub async fn export<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Params(query): Params<OperatorReservationQuery>) -> Reply<Page<OperatorReservationView>> {
    Ok(reply(export_operator_reservations(app_kernel(&state), &actor, query).await?, id))
}

#[utoipa::path(
    get, path = "/api/host/reservations/{reservationId}", operation_id = "host_reservations_get",
    params(("reservationId" = String, Path), HostLocaleQuery),
    responses((status = 200, body = Envelope<OperatorReservationView>), (status = 404, body = ErrorBody))
)]
pub async fn get<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(reservation_id): Path<String>, Params(query): Params<HostLocaleQuery>) -> Reply<OperatorReservationView> {
    Ok(reply(get_operator_reservation(app_kernel(&state), &actor, &reservation_id, query.locale.as_deref()).await?, id))
}

#[utoipa::path(
    post, path = "/api/host/reservations/{reservationId}/decision", operation_id = "host_reservations_decide",
    params(("reservationId" = String, Path)), request_body = DecisionInput,
    responses((status = 200, body = Envelope<ReservationView>), (status = 409, body = ErrorBody))
)]
pub async fn decide<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(reservation_id): Path<String>, Body(input): Body<DecisionInput>) -> Reply<ReservationView> {
    Ok(reply(decide_request(app_kernel(&state), &actor, &reservation_id, input).await?, id))
}

#[utoipa::path(
    post, path = "/api/host/reservations/{reservationId}/special-request", operation_id = "host_reservations_special_request",
    params(("reservationId" = String, Path)), request_body = SpecialRequestDecisionInput,
    responses((status = 200, body = Envelope<ReservationView>), (status = 409, body = ErrorBody))
)]
pub async fn special_request<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(reservation_id): Path<String>, Body(input): Body<SpecialRequestDecisionInput>) -> Reply<ReservationView> {
    Ok(reply(decide_special_request(app_kernel(&state), &actor, &reservation_id, input).await?, id))
}

#[utoipa::path(
    post, path = "/api/host/reservations/{reservationId}/check-in", operation_id = "host_reservations_check_in",
    params(("reservationId" = String, Path)), request_body = VersionedAction,
    responses((status = 200, body = Envelope<ReservationView>), (status = 409, body = ErrorBody))
)]
pub async fn check_in<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(reservation_id): Path<String>, Body(input): Body<VersionedAction>) -> Reply<ReservationView> {
    Ok(reply(transition(app_kernel(&state), &actor, &reservation_id, input, StayAction::CheckIn).await?, id))
}

#[utoipa::path(
    post, path = "/api/host/reservations/{reservationId}/check-out", operation_id = "host_reservations_check_out",
    params(("reservationId" = String, Path)), request_body = VersionedAction,
    responses((status = 200, body = Envelope<ReservationView>), (status = 409, body = ErrorBody))
)]
pub async fn check_out<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(reservation_id): Path<String>, Body(input): Body<VersionedAction>) -> Reply<ReservationView> {
    Ok(reply(transition(app_kernel(&state), &actor, &reservation_id, input, StayAction::CheckOut).await?, id))
}

#[utoipa::path(
    post, path = "/api/host/reservations/{reservationId}/no-show", operation_id = "host_reservations_no_show",
    params(("reservationId" = String, Path)), request_body = VersionedAction,
    responses((status = 200, body = Envelope<ReservationView>), (status = 409, body = ErrorBody))
)]
pub async fn no_show<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(reservation_id): Path<String>, Body(input): Body<VersionedAction>) -> Reply<ReservationView> {
    Ok(reply(transition(app_kernel(&state), &actor, &reservation_id, input, StayAction::NoShow).await?, id))
}

#[utoipa::path(
    post, path = "/api/host/reservations/{reservationId}/cancel", operation_id = "host_reservations_cancel",
    params(("reservationId" = String, Path)), request_body = OperatorCancelInput,
    responses((status = 200, body = Envelope<ReservationView>), (status = 409, body = ErrorBody))
)]
pub async fn cancel<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(reservation_id): Path<String>, Body(input): Body<OperatorCancelInput>) -> Reply<ReservationView> {
    Ok(reply(operator_cancel(app_kernel(&state), &actor, &reservation_id, input).await?, id))
}

#[utoipa::path(
    post, path = "/api/host/reservations/{reservationId}/amendments", operation_id = "host_reservations_amend",
    params(("reservationId" = String, Path)), request_body = ProposeInput,
    responses((status = 200, body = Envelope<AmendmentProposal>), (status = 409, body = ErrorBody))
)]
pub async fn amend<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(reservation_id): Path<String>, Body(input): Body<ProposeInput>) -> Reply<AmendmentProposal> {
    Ok(reply(propose_operator_amendment(app_kernel(&state), &actor, &reservation_id, input).await?, id))
}

#[utoipa::path(
    get, path = "/api/host/reservations/{reservationId}/messages", operation_id = "reservation_messages_list_host",
    params(("reservationId" = String, Path), MessagesQuery),
    responses((status = 200, body = Envelope<Page<MessageView>>), (status = 404, body = ErrorBody))
)]
pub async fn messages_list<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(reservation_id): Path<String>, Params(query): Params<MessagesQuery>) -> Reply<Page<MessageView>> {
    Ok(reply(messages::list(app_kernel(&state), &actor, &reservation_id, messages::MessageAudience::Host, query).await?, id))
}

#[utoipa::path(
    post, path = "/api/host/reservations/{reservationId}/messages", operation_id = "reservation_messages_send_host",
    params(("reservationId" = String, Path)), request_body = SendMessageInput,
    responses((status = 200, body = Envelope<MessageView>), (status = 400, body = ErrorBody))
)]
pub async fn messages_send<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(reservation_id): Path<String>, Body(input): Body<SendMessageInput>) -> Reply<MessageView> {
    Ok(reply(messages::send(app_kernel(&state), &actor, &reservation_id, messages::MessageAudience::Host, input).await?, id))
}

#[utoipa::path(
    get, path = "/api/host/message-conversations", operation_id = "host_message_conversations", params(crate::reservations::messages::inbox::InboxQuery),
    responses((status = 200, body = Envelope<Page<crate::reservations::messages::inbox::ConversationView>>), (status = 403, body = ErrorBody))
)]
pub async fn message_conversations<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Params(query): Params<crate::reservations::messages::inbox::InboxQuery>) -> Reply<Page<crate::reservations::messages::inbox::ConversationView>> {
    Ok(reply(crate::reservations::messages::inbox::list(app_kernel(&state), &actor, query).await?, id))
}

#[utoipa::path(
    post, path = "/api/host/reservations/{reservationId}/messages/read", operation_id = "host_messages_mark_read",
    params(("reservationId" = String, Path)), request_body = crate::reservations::messages::read::ReadMessagesInput,
    responses((status = 200, body = Envelope<crate::reservations::messages::read::ReadMessagesResult>), (status = 404, body = ErrorBody))
)]
pub async fn messages_mark_read<S: AppState>(State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(reservation_id): Path<String>, Body(input): Body<crate::reservations::messages::read::ReadMessagesInput>) -> Reply<crate::reservations::messages::read::ReadMessagesResult> {
    Ok(reply(crate::reservations::messages::read::mark(app_kernel(&state), &actor, &reservation_id, input).await?, id))
}
