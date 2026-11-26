use axum::extract::rejection::JsonRejection;
use axum::extract::{Json, Path, State};
use kernel::auth::Actor;
use kernel::envelope::Envelope;
use kernel::error::ErrorBody;
use kernel::KernelState;

use super::extract::{body, reply, Reply};
use crate::pricing::settings::service;
use crate::pricing::settings::types::{SettingsView, UpdateSettingInput, UpdatedSetting};

#[utoipa::path(
    get, path = "/api/admin/settings", operation_id = "admin_settings_list",
    responses((status = 200, body = Envelope<SettingsView>), (status = 403, body = ErrorBody))
)]
pub async fn list<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
) -> Reply<SettingsView> {
    let view = service::list(state.kernel().db.pool(), &actor).await?;
    reply(view, &actor)
}

#[utoipa::path(
    patch, path = "/api/admin/settings/{key}", operation_id = "admin_settings_update",
    params(("key" = String, Path)), request_body = UpdateSettingInput,
    responses((status = 200, body = Envelope<UpdatedSetting>), (status = 400, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn update<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
    Path(key): Path<String>,
    input: std::result::Result<Json<UpdateSettingInput>, JsonRejection>,
) -> Reply<UpdatedSetting> {
    let input = body(input)?;
    let updated = service::update(state.kernel().db.pool(), &actor, &key, input).await?;
    reply(updated, &actor)
}
