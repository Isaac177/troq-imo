use axum::extract::rejection::JsonRejection;
use axum::extract::{Json, State};
use kernel::auth::{RequestCity, RequestId};
use kernel::city::City;
use kernel::config::OtpChannel;
use kernel::envelope::Envelope;
use kernel::error::{codes, Error, ErrorBody, Result};
use kernel::{Kernel, KernelState};
use kernel::permissions::names;

use super::account::{self, ProfileLanguage, RegisterInput};
use super::api_types::{
    CatalogView, CityView, IssueOtpInput, LoginRequest, OtpChannelResponse, PermissionView, PresetView,
    ProofResponse, RefreshRequest, RegisterRequest, TokenResponse, VerifyOtpInput,
};
use super::password_login::password_login;
use super::presets;
use super::registration_otp::{self, RegistrationChannel};
use super::sessions::refresh_session;
use crate::notifications::NotificationsConfig;

fn body<T>(input: std::result::Result<Json<T>, JsonRejection>) -> Result<T> {
    input.map(|Json(value)| value).map_err(|_| Error::bad_request(codes::MALFORMED_REQUEST))
}

fn language(value: Option<&str>) -> String {
    ProfileLanguage::parse(value.unwrap_or("en")).map(|value| value.as_str().to_string()).unwrap_or_else(|_| "en".into())
}

fn channel(value: &str) -> Result<RegistrationChannel> {
    registration_otp::parse_channel(value).ok_or_else(|| Error::bad_request(codes::MALFORMED_REQUEST))
}

#[utoipa::path(
    get, path = "/api/public/auth/otp-channel", operation_id = "auth_otp_channel",
    responses((status = 200, body = Envelope<OtpChannelResponse>))
)]
pub async fn otp_channel<S>(State(state): State<S>, RequestId(id): RequestId) -> Result<Json<Envelope<OtpChannelResponse>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let channel = match state.kernel().config.otp_channel {
        OtpChannel::Email => "email",
        OtpChannel::Whatsapp => "whatsapp",
    };
    Ok(Json(Envelope::ok(OtpChannelResponse { channel: channel.into() }, id)))
}

#[utoipa::path(
    post, path = "/api/public/auth/register/otp", operation_id = "auth_register_otp",
    request_body = IssueOtpInput,
    responses((status = 200, body = Envelope<OtpChannelResponse>), (status = 400, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn issue_otp<S>(
    State(state): State<S>,
    RequestId(id): RequestId,
    input: std::result::Result<Json<IssueOtpInput>, JsonRejection>,
) -> Result<Json<Envelope<OtpChannelResponse>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let input = body(input)?;
    let channel = channel(&input.channel)?;
    let config = NotificationsConfig::from_env();
    registration_otp::issue_registration_otp(
        state.kernel().db.pool(),
        &config,
        state.kernel().config.otp_signing_key.as_bytes(),
        channel,
        &input.contact,
        &language(input.language.as_deref()),
    )
    .await?;
    Ok(Json(Envelope::ok(OtpChannelResponse { channel: channel_name(channel).into() }, id)))
}

fn channel_name(channel: RegistrationChannel) -> &'static str {
    match channel {
        RegistrationChannel::Email => "email",
        RegistrationChannel::Whatsapp => "whatsapp",
    }
}

#[utoipa::path(
    post, path = "/api/public/auth/register/verify", operation_id = "auth_register_verify",
    request_body = VerifyOtpInput,
    responses((status = 200, body = Envelope<ProofResponse>), (status = 400, body = ErrorBody))
)]
pub async fn verify_otp<S>(
    State(state): State<S>,
    RequestId(id): RequestId,
    input: std::result::Result<Json<VerifyOtpInput>, JsonRejection>,
) -> Result<Json<Envelope<ProofResponse>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let input = body(input)?;
    let channel = channel(&input.channel)?;
    let proof = registration_otp::verify_registration_otp(
        state.kernel().db.pool(),
        state.kernel().config.otp_signing_key.as_bytes(),
        channel,
        &input.contact,
        &input.code,
    )
    .await?;
    Ok(Json(Envelope::ok(ProofResponse { proof }, id)))
}

fn register_input(kernel: &Kernel, input: RegisterRequest) -> Result<RegisterInput> {
    let channel = channel(&input.channel)?;
    let city = City::from_code(&input.city_code)?;
    if !kernel.config.serves(city) {
        return Err(Error::bad_request(codes::CITY_UNSUPPORTED).with_params(serde_json::json!({ "city": city.code() })));
    }
    Ok(RegisterInput {
        channel,
        contact: input.contact,
        proof: input.proof,
        password: input.password,
        first_name: input.first_name,
        last_name: input.last_name,
        city,
        preferred_language: language(input.preferred_language.as_deref().or(input.language.as_deref())),
    })
}

#[utoipa::path(
    post, path = "/api/public/auth/register", operation_id = "auth_register",
    request_body = RegisterRequest,
    responses((status = 200, body = Envelope<TokenResponse>), (status = 400, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn register<S>(
    State(state): State<S>,
    RequestId(id): RequestId,
    input: std::result::Result<Json<RegisterRequest>, JsonRejection>,
) -> Result<Json<Envelope<TokenResponse>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let kernel = state.kernel();
    let input = body(input)?;
    let tokens = account::register(
        kernel.db.pool(),
        &kernel.config,
        kernel.config.otp_signing_key.as_bytes(),
        register_input(kernel, input)?,
    )
    .await?;
    Ok(Json(Envelope::ok(TokenResponse { access_token: tokens.access_token, refresh_token: tokens.refresh_token }, id)))
}

#[utoipa::path(
    post, path = "/api/public/auth/login", operation_id = "auth_login",
    request_body = LoginRequest,
    responses((status = 200, body = Envelope<TokenResponse>), (status = 400, body = ErrorBody), (status = 401, body = ErrorBody))
)]
pub async fn login<S>(
    State(state): State<S>,
    RequestId(id): RequestId,
    input: std::result::Result<Json<LoginRequest>, JsonRejection>,
) -> Result<Json<Envelope<TokenResponse>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let input = body(input)?;
    let tokens = password_login(state.kernel().db.pool(), &state.kernel().config, &input.identifier, &input.password).await?;
    Ok(Json(Envelope::ok(TokenResponse { access_token: tokens.access_token, refresh_token: tokens.refresh_token }, id)))
}

#[utoipa::path(
    post, path = "/api/public/auth/refresh", operation_id = "auth_refresh",
    request_body = RefreshRequest,
    responses((status = 200, body = Envelope<TokenResponse>), (status = 401, body = ErrorBody))
)]
pub async fn refresh<S>(
    State(state): State<S>,
    RequestId(id): RequestId,
    input: std::result::Result<Json<RefreshRequest>, JsonRejection>,
) -> Result<Json<Envelope<TokenResponse>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let input = body(input)?;
    let tokens = refresh_session(state.kernel().db.pool(), &state.kernel().config, &input.refresh_token).await?;
    Ok(Json(Envelope::ok(TokenResponse { access_token: tokens.access_token, refresh_token: tokens.refresh_token }, id)))
}

#[utoipa::path(
    get, path = "/api/public/cities", operation_id = "cities_list",
    responses((status = 200, body = Envelope<Vec<CityView>>))
)]
pub async fn cities<S>(State(state): State<S>, RequestId(id): RequestId) -> Result<Json<Envelope<Vec<CityView>>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let items = state
        .kernel()
        .config
        .cities
        .iter()
        .map(|city| CityView { code: city.code().into(), name: city.name().into(), timezone: city.timezone().into() })
        .collect();
    Ok(Json(Envelope::ok(items, id)))
}

#[utoipa::path(
    get, path = "/api/public/permission-catalog", operation_id = "permission_catalog",
    responses((status = 200, body = Envelope<CatalogView>))
)]
pub async fn permission_catalog<S>(State(_state): State<S>, RequestId(id): RequestId) -> Result<Json<Envelope<CatalogView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let permissions = names::ALL
        .iter()
        .map(|name| PermissionView { name: (*name).into(), domain: presets::domain(name).into(), delegable: presets::is_delegable(name) })
        .collect();
    let presets = presets::PRESETS
        .iter()
        .map(|preset| PresetView {
            code: preset.code.into(),
            audience: preset.audience.as_str().into(),
            permissions: preset.permissions.iter().map(|value| (*value).to_string()).collect(),
        })
        .collect();
    Ok(Json(Envelope::ok(CatalogView { permissions, presets }, id)))
}

#[utoipa::path(
    get, path = "/api/public/host-reference", operation_id = "host_reference",
    responses((status = 200, body = Envelope<super::api_types::HostReferenceView>))
)]
pub async fn host_reference<S>(State(_state): State<S>, RequestId(id): RequestId) -> Result<Json<Envelope<super::api_types::HostReferenceView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    use crate::hosts::onboarding::types as reference;
    let strings = |values: &[&str]| values.iter().map(|value| (*value).to_string()).collect();
    Ok(Json(Envelope::ok(super::api_types::HostReferenceView {
        kinds: strings(reference::APPLICATION_KINDS),
        document_kinds: strings(reference::DOCUMENT_KINDS),
        approve_reasons: strings(reference::APPROVE_REASONS),
        reject_reasons: strings(reference::REJECT_REASONS),
        suspend_reasons: strings(reference::SUSPEND_REASONS),
        reinstate_reasons: strings(reference::REINSTATE_REASONS),
        document_reject_reasons: strings(reference::DOCUMENT_REJECT_REASONS),
    }, id)))
}

#[utoipa::path(
    get, path = "/api/public/city-context", operation_id = "city_context",
    params(("x-city" = Option<String>, Header)),
    responses((status = 200, body = Envelope<CityView>))
)]
pub async fn city_context<S>(State(_state): State<S>, RequestId(id): RequestId, RequestCity(city): RequestCity) -> Result<Json<Envelope<CityView>>>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(CityView { code: city.code().into(), name: city.name().into(), timezone: city.timezone().into() }, id)))
}
