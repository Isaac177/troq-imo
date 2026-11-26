use axum::routing::{delete, get, patch, post, put};
use axum::Router;
use kernel::KernelState;
use utoipa::OpenApi;

use super::api_types::{
    CatalogView, CityView, HostReferenceView, IssueOtpInput, LoginRequest, MessageResponse,
    OtpChannelResponse, PermissionView, PresetView, ProofResponse, RefreshRequest, RegisterRequest,
    TokenResponse, VerifyOtpInput,
};
use super::handlers_host as host;
use super::handlers_invitations as invitations;
use super::handlers_public as public;
use super::handlers_session as session;
use super::invitations::{CreateInvitation, InvitationView, MyInvitationView};
use super::manage_overrides::{HostOverride, SetHostOverride};
use super::overrides::OverrideEffect;
use super::session_view::{PermissionGrantView, SessionHost, SessionUser, SessionView};
use super::team::{MemberOverride, MemberView, SetMemberPermissions};
use crate::hosts::{HostContact, HostMembership, UpdateHostContact};

pub fn public_routes<S>() -> Router<S>
where S: KernelState + Clone + Send + Sync + 'static {
    Router::new()
        .route("/auth/otp-channel", get(public::otp_channel::<S>))
        .route("/auth/register/otp", post(public::issue_otp::<S>))
        .route("/auth/register/verify", post(public::verify_otp::<S>))
        .route("/auth/register", post(public::register::<S>))
        .route("/auth/login", post(public::login::<S>))
        .route("/auth/refresh", post(public::refresh::<S>))
        .route("/cities", get(public::cities::<S>))
        .route("/city-context", get(public::city_context::<S>))
        .route("/permission-catalog", get(public::permission_catalog::<S>))
        .route("/host-reference", get(public::host_reference::<S>))
}

pub fn guest_routes<S>() -> Router<S>
where S: KernelState + Clone + Send + Sync + 'static {
    Router::new()
        .route("/session", get(session::session::<S>))
        .route("/session/logout", post(session::logout::<S>))
        .route("/profile", patch(session::update_me::<S>))
        .route("/invitations", get(invitations::mine::<S>))
        .route("/invitations/:invitation_id/accept", post(invitations::accept::<S>))
}

pub fn host_routes<S>() -> Router<S>
where S: KernelState + Clone + Send + Sync + 'static {
    Router::new()
        .route("/hosts", get(host::my_hosts::<S>))
        .route("/hosts/:host_id/contact", patch(host::update_contact::<S>))
        .route("/hosts/:host_id/members", get(host::list_host_members::<S>))
        .route("/hosts/:host_id/members/:user_id", delete(host::remove::<S>))
        .route("/hosts/:host_id/members/:user_id/permissions", put(host::set_permissions::<S>))
        .route("/hosts/:host_id/members/:user_id/overrides", put(host::set_override::<S>))
        .route("/hosts/:host_id/members/:user_id/overrides/:override_id", delete(host::clear_override::<S>))
        .route("/hosts/:host_id/invitations", get(invitations::list::<S>).post(invitations::create::<S>))
        .route("/hosts/:host_id/invitations/:invitation_id", delete(invitations::revoke::<S>))
}

pub fn admin_routes<S>() -> Router<S>
where S: KernelState + Clone + Send + Sync + 'static {
    Router::new()
}

#[derive(OpenApi)]
#[openapi(
    paths(
        public::otp_channel, public::issue_otp, public::verify_otp, public::register, public::login,
        public::refresh, public::cities, public::city_context, public::permission_catalog, public::host_reference,
        session::session, session::logout, session::update_me,
        invitations::mine, invitations::accept, invitations::create, invitations::list, invitations::revoke,
        host::my_hosts, host::update_contact, host::list_host_members, host::set_permissions,
        host::remove, host::set_override, host::clear_override
    ),
    components(schemas(
        TokenResponse, ProofResponse, OtpChannelResponse, IssueOtpInput, VerifyOtpInput, RegisterRequest,
        LoginRequest, RefreshRequest, CityView, PermissionView, PresetView, CatalogView, HostReferenceView,
        MessageResponse, SessionView, SessionUser, PermissionGrantView, SessionHost, HostMembership,
        HostContact, UpdateHostContact, MemberView, MemberOverride, SetMemberPermissions, HostOverride,
        SetHostOverride, OverrideEffect, CreateInvitation, InvitationView, MyInvitationView,
        session::ProfileInput
    ))
)]
struct Doc;

pub fn openapi() -> utoipa::openapi::OpenApi {
    Doc::openapi()
}
