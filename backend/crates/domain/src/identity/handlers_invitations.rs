use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::extract::{Json, Path, Query, State};
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{codes, Error, ErrorBody, Result};
use kernel::paging::PageRequest;
use kernel::{KernelState, Page};

use super::api_types::MessageResponse;
use super::invitations::{
    accept_invitation, create_invitation, list_invitations, list_my_invitations, revoke_invitation,
    CreateInvitation, InvitationView, MyInvitationView,
};

fn body<T>(input: std::result::Result<Json<T>, JsonRejection>) -> Result<T> {
    input.map(|Json(value)| value).map_err(|_| Error::bad_request(codes::MALFORMED_REQUEST))
}

fn query(input: std::result::Result<Query<PageRequest>, QueryRejection>) -> Result<PageRequest> {
    input.map(|Query(value)| value).map_err(|_| Error::bad_request(codes::MALFORMED_REQUEST))
}

#[utoipa::path(
    post, path = "/api/host/hosts/{hostId}/invitations", operation_id = "host_invitation_create",
    params(("hostId" = String, Path)), request_body = CreateInvitation,
    responses((status = 200, body = Envelope<InvitationView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn create<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    Path(host_id): Path<String>, input: std::result::Result<Json<CreateInvitation>, JsonRejection>,
) -> Result<Json<Envelope<InvitationView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let invitation = create_invitation(state.kernel().db.pool(), &actor, &host_id, body(input)?).await?;
    Ok(Json(Envelope::ok(invitation, id)))
}

#[utoipa::path(
    get, path = "/api/host/hosts/{hostId}/invitations", operation_id = "host_invitations_list",
    params(("hostId" = String, Path), ("page" = Option<u32>, Query), ("pageSize" = Option<u32>, Query)),
    responses((status = 200, body = Envelope<Page<InvitationView>>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn list<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    Path(host_id): Path<String>, input: std::result::Result<Query<PageRequest>, QueryRejection>,
) -> Result<Json<Envelope<Page<InvitationView>>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let page = list_invitations(state.kernel().db.pool(), &actor, &host_id, query(input)?).await?;
    Ok(Json(Envelope::ok(page, id)))
}

#[utoipa::path(
    delete, path = "/api/host/hosts/{hostId}/invitations/{invitationId}", operation_id = "host_invitation_revoke",
    params(("hostId" = String, Path), ("invitationId" = String, Path)),
    responses((status = 200, body = Envelope<MessageResponse>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn revoke<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    Path((host_id, invitation_id)): Path<(String, String)>,
) -> Result<Json<Envelope<MessageResponse>>>
where S: KernelState + Clone + Send + Sync + 'static {
    revoke_invitation(state.kernel().db.pool(), &actor, &host_id, &invitation_id).await?;
    Ok(Json(Envelope::ok(MessageResponse { message: "host.invitation.revoked".into() }, id)))
}

#[utoipa::path(
    get, path = "/api/guest/invitations", operation_id = "guest_invitations_list",
    params(("page" = Option<u32>, Query), ("pageSize" = Option<u32>, Query)),
    responses((status = 200, body = Envelope<Page<MyInvitationView>>), (status = 401, body = ErrorBody))
)]
pub async fn mine<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    input: std::result::Result<Query<PageRequest>, QueryRejection>,
) -> Result<Json<Envelope<Page<MyInvitationView>>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let page = list_my_invitations(state.kernel().db.pool(), &actor, query(input)?).await?;
    Ok(Json(Envelope::ok(page, id)))
}

#[utoipa::path(
    post, path = "/api/guest/invitations/{invitationId}/accept", operation_id = "guest_invitation_accept",
    params(("invitationId" = String, Path)),
    responses((status = 200, body = Envelope<InvitationView>), (status = 401, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn accept<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    Path(invitation_id): Path<String>,
) -> Result<Json<Envelope<InvitationView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let invitation = accept_invitation(state.kernel().db.pool(), &actor, &invitation_id).await?;
    Ok(Json(Envelope::ok(invitation, id)))
}
