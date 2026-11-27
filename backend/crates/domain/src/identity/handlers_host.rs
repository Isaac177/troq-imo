use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::extract::{Json, Path, Query, State};
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{codes, Error, ErrorBody, Result};
use kernel::paging::PageRequest;
use kernel::{KernelState, Page};

use super::api_types::MessageResponse;
use super::clear_override::clear_host_override;
use super::manage_overrides::{set_host_override, HostOverride, SetHostOverride};
use super::team::{list_members, remove_member, set_member_permissions, MemberView, SetMemberPermissions};
use crate::hosts::{self, HostContact, HostMembership, UpdateHostContact};

fn body<T>(input: std::result::Result<Json<T>, JsonRejection>) -> Result<T> {
    input.map(|Json(value)| value).map_err(|_| Error::bad_request(codes::MALFORMED_REQUEST))
}

fn query(input: std::result::Result<Query<PageRequest>, QueryRejection>) -> Result<PageRequest> {
    input.map(|Query(value)| value).map_err(|_| Error::bad_request(codes::MALFORMED_REQUEST))
}

#[utoipa::path(
    get, path = "/api/host/hosts", operation_id = "host_hosts_list",
    params(("page" = Option<u32>, Query), ("pageSize" = Option<u32>, Query), ("x-city" = Option<String>, Header)),
    responses((status = 200, body = Envelope<Page<HostMembership>>), (status = 401, body = ErrorBody))
)]
pub async fn my_hosts<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    input: std::result::Result<Query<PageRequest>, QueryRejection>,
) -> Result<Json<Envelope<Page<HostMembership>>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let page = hosts::list_my_hosts(state.kernel().db.pool(), &actor.user, actor.city, query(input)?).await?;
    Ok(Json(Envelope::ok(page, id)))
}

#[utoipa::path(
    get, path = "/api/host/hosts/{hostId}/contact", operation_id = "host_contact_get",
    params(("hostId" = String, Path)),
    responses((status = 200, body = Envelope<HostContact>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn get_contact<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId, Path(host_id): Path<String>,
) -> Result<Json<Envelope<HostContact>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let contact = hosts::get_contact(state.kernel().db.pool(), &actor, &host_id).await?;
    Ok(Json(Envelope::ok(contact, id)))
}

#[utoipa::path(
    patch, path = "/api/host/hosts/{hostId}/contact", operation_id = "host_contact_update",
    params(("hostId" = String, Path)), request_body = UpdateHostContact,
    responses((status = 200, body = Envelope<HostContact>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn update_contact<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    Path(host_id): Path<String>, input: std::result::Result<Json<UpdateHostContact>, JsonRejection>,
) -> Result<Json<Envelope<HostContact>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let contact = hosts::update_contact(state.kernel().db.pool(), &actor, &host_id, body(input)?).await?;
    Ok(Json(Envelope::ok(contact, id)))
}

#[utoipa::path(
    get, path = "/api/host/hosts/{hostId}/members", operation_id = "host_members_list",
    params(("hostId" = String, Path), ("page" = Option<u32>, Query), ("pageSize" = Option<u32>, Query)),
    responses((status = 200, body = Envelope<Page<MemberView>>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn list_host_members<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    Path(host_id): Path<String>, input: std::result::Result<Query<PageRequest>, QueryRejection>,
) -> Result<Json<Envelope<Page<MemberView>>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let page = list_members(state.kernel().db.pool(), &actor, &host_id, query(input)?).await?;
    Ok(Json(Envelope::ok(page, id)))
}

#[utoipa::path(
    put, path = "/api/host/hosts/{hostId}/members/{userId}/permissions", operation_id = "host_member_permissions_set",
    params(("hostId" = String, Path), ("userId" = String, Path)), request_body = SetMemberPermissions,
    responses((status = 200, body = Envelope<MemberView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn set_permissions<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    Path((host_id, user_id)): Path<(String, String)>,
    input: std::result::Result<Json<SetMemberPermissions>, JsonRejection>,
) -> Result<Json<Envelope<MemberView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let member = set_member_permissions(state.kernel().db.pool(), &actor, &host_id, &user_id, body(input)?).await?;
    Ok(Json(Envelope::ok(member, id)))
}

#[utoipa::path(
    delete, path = "/api/host/hosts/{hostId}/members/{userId}", operation_id = "host_member_remove",
    params(("hostId" = String, Path), ("userId" = String, Path)),
    responses((status = 200, body = Envelope<MessageResponse>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn remove<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    Path((host_id, user_id)): Path<(String, String)>,
) -> Result<Json<Envelope<MessageResponse>>>
where S: KernelState + Clone + Send + Sync + 'static {
    remove_member(state.kernel().db.pool(), &actor, &host_id, &user_id).await?;
    Ok(Json(Envelope::ok(MessageResponse { message: "host.member.removed".into() }, id)))
}

#[utoipa::path(
    put, path = "/api/host/hosts/{hostId}/members/{userId}/overrides", operation_id = "host_member_override_set",
    params(("hostId" = String, Path), ("userId" = String, Path)), request_body = SetHostOverride,
    responses((status = 200, body = Envelope<HostOverride>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn set_override<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    Path((host_id, user_id)): Path<(String, String)>,
    input: std::result::Result<Json<SetHostOverride>, JsonRejection>,
) -> Result<Json<Envelope<HostOverride>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let value = set_host_override(state.kernel().db.pool(), &actor, &host_id, &user_id, body(input)?).await?;
    Ok(Json(Envelope::ok(value, id)))
}

#[utoipa::path(
    delete, path = "/api/host/hosts/{hostId}/members/{userId}/overrides/{overrideId}", operation_id = "host_member_override_clear",
    params(("hostId" = String, Path), ("userId" = String, Path), ("overrideId" = String, Path)),
    responses((status = 200, body = Envelope<MessageResponse>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn clear_override<S>(
    State(state): State<S>, actor: Actor, RequestId(id): RequestId,
    Path((host_id, _user_id, override_id)): Path<(String, String, String)>,
) -> Result<Json<Envelope<MessageResponse>>>
where S: KernelState + Clone + Send + Sync + 'static {
    clear_host_override(state.kernel().db.pool(), &actor, &host_id, &override_id).await?;
    Ok(Json(Envelope::ok(MessageResponse { message: "host.member.override_cleared".into() }, id)))
}
