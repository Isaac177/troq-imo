use axum::extract::rejection::JsonRejection;
use axum::extract::{Json, State};
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{codes, Error, ErrorBody, Result};
use kernel::KernelState;

use super::account::{update_profile, UpdateProfileInput};
use super::api_types::MessageResponse;
use super::session_view::{session_view, SessionView};
use super::sessions::logout_all_sessions;

fn body<T>(input: std::result::Result<Json<T>, JsonRejection>) -> Result<T> {
    input.map(|Json(value)| value).map_err(|_| Error::bad_request(codes::MALFORMED_REQUEST))
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProfileInput {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub preferred_language: Option<String>,
}

#[utoipa::path(
    get, path = "/api/guest/session", operation_id = "auth_session",
    params(("x-city" = Option<String>, Header)),
    responses((status = 200, body = Envelope<SessionView>), (status = 401, body = ErrorBody))
)]
pub async fn session<S>(State(state): State<S>, actor: Actor, RequestId(id): RequestId) -> Result<Json<Envelope<SessionView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(session_view(state.kernel().db.pool(), &actor).await?, id)))
}

#[utoipa::path(
    post, path = "/api/guest/session/logout", operation_id = "auth_logout",
    responses((status = 200, body = Envelope<MessageResponse>), (status = 401, body = ErrorBody))
)]
pub async fn logout<S>(State(state): State<S>, actor: Actor, RequestId(id): RequestId) -> Result<Json<Envelope<MessageResponse>>>
where S: KernelState + Clone + Send + Sync + 'static {
    logout_all_sessions(state.kernel().db.pool(), &actor.user).await?;
    Ok(Json(Envelope::ok(MessageResponse { message: "auth.logged_out".into() }, id)))
}

#[utoipa::path(
    patch, path = "/api/guest/profile", operation_id = "auth_profile_update",
    request_body = ProfileInput,
    responses((status = 200, body = Envelope<MessageResponse>), (status = 400, body = ErrorBody), (status = 401, body = ErrorBody))
)]
pub async fn update_me<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(id): RequestId,
    input: std::result::Result<Json<ProfileInput>, JsonRejection>,
) -> Result<Json<Envelope<MessageResponse>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let input = body(input)?;
    update_profile(
        state.kernel().db.pool(),
        actor.user_id().as_str(),
        UpdateProfileInput {
            first_name: input.first_name,
            last_name: input.last_name,
            preferred_language: input.preferred_language,
        },
    )
    .await?;
    Ok(Json(Envelope::ok(MessageResponse { message: "auth.profile_updated".into() }, id)))
}
