use axum::extract::{Path, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use kernel::auth::{Actor, RequestId};
use kernel::city::City;
use kernel::envelope::Envelope;
use kernel::error::{Error, ErrorBody, Result};
use kernel::kernel::KernelState;
use kernel::paging::Page;

use super::defaults::{update, InventoryDefaultsInput, InventoryDefaultsResult};
use super::feed_tokens::{
    create, feed_by_token, list, revoke, CreateFeedTokenInput, FeedTokenCreated, FeedTokenRevoked, FeedTokenView,
};
use super::service::{admin_calendar, host_calendar, host_feed};
use super::types::{CalendarFeedQuery, CalendarQuery, CalendarView};
use crate::inventory::http::{JsonBody, Params};
use crate::inventory::messages::FEED_NOT_FOUND;
use crate::inventory::query::paging;

fn calendar_response(body: String) -> Response {
    (StatusCode::OK, [(header::CONTENT_TYPE, "text/calendar; charset=utf-8")], body).into_response()
}

#[derive(serde::Deserialize, utoipa::IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct FeedTokenListQuery {
    pub property_id: String,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[utoipa::path(
    get, path = "/api/host/calendar", operation_id = "inventory_calendar_get",
    params(CalendarQuery),
    responses((status = 200, body = Envelope<CalendarView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn host_get<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Params(query): Params<CalendarQuery>,
) -> Result<Json<Envelope<CalendarView>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let view = host_calendar(state.kernel().db.pool(), &state.kernel().storage, &actor, query).await?;
    Ok(Json(Envelope::ok(view, request_id)))
}

#[utoipa::path(
    get, path = "/api/admin/calendar", operation_id = "admin_inventory_calendar_get",
    params(CalendarQuery),
    responses((status = 200, body = Envelope<CalendarView>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn admin_get<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Params(query): Params<CalendarQuery>,
) -> Result<Json<Envelope<CalendarView>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let view = admin_calendar(state.kernel().db.pool(), &state.kernel().storage, &actor, query).await?;
    Ok(Json(Envelope::ok(view, request_id)))
}

#[utoipa::path(
    post, path = "/api/host/calendar/inventory-default", operation_id = "inventory_defaults_update",
    request_body = InventoryDefaultsInput,
    responses((status = 200, body = Envelope<InventoryDefaultsResult>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn host_defaults<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    JsonBody(input): JsonBody<InventoryDefaultsInput>,
) -> Result<Json<Envelope<InventoryDefaultsResult>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let result = update(state.kernel().db.pool(), &actor, input, false).await?;
    Ok(Json(Envelope::ok(result, request_id)))
}

#[utoipa::path(
    post, path = "/api/admin/inventory-defaults", operation_id = "admin_inventory_defaults_update",
    request_body = InventoryDefaultsInput,
    responses((status = 200, body = Envelope<InventoryDefaultsResult>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn admin_defaults<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    JsonBody(input): JsonBody<InventoryDefaultsInput>,
) -> Result<Json<Envelope<InventoryDefaultsResult>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let result = update(state.kernel().db.pool(), &actor, input, true).await?;
    Ok(Json(Envelope::ok(result, request_id)))
}

#[utoipa::path(
    get, path = "/api/host/calendar/feed.ics", operation_id = "inventory_calendar_feed",
    params(CalendarFeedQuery),
    responses((status = 200, content_type = "text/calendar", body = String), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn host_feed_ics<S>(
    State(state): State<S>,
    actor: Actor,
    Params(query): Params<CalendarFeedQuery>,
) -> Result<Response>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    Ok(calendar_response(host_feed(state.kernel().db.pool(), &actor, &query.property_id).await?))
}

#[utoipa::path(
    get, path = "/api/host/calendar/feed-tokens", operation_id = "inventory_feed_tokens_list",
    params(FeedTokenListQuery),
    responses((status = 200, body = Envelope<Page<FeedTokenView>>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn host_tokens<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Params(query): Params<FeedTokenListQuery>,
) -> Result<Json<Envelope<Page<FeedTokenView>>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let page = paging(query.page, query.page_size)?;
    let result = list(state.kernel().db.pool(), &actor, &query.property_id, page).await?;
    Ok(Json(Envelope::ok(result, request_id)))
}

#[utoipa::path(
    post, path = "/api/host/calendar/feed-tokens", operation_id = "inventory_feed_tokens_create",
    request_body = CreateFeedTokenInput,
    responses((status = 200, body = Envelope<FeedTokenCreated>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn host_token_create<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    JsonBody(input): JsonBody<CreateFeedTokenInput>,
) -> Result<Json<Envelope<FeedTokenCreated>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let result = create(state.kernel().db.pool(), &actor, input).await?;
    Ok(Json(Envelope::ok(result, request_id)))
}

#[utoipa::path(
    delete, path = "/api/host/calendar/feed-tokens/{tokenId}", operation_id = "inventory_feed_tokens_revoke",
    params(("tokenId" = String, Path)),
    responses((status = 200, body = Envelope<FeedTokenRevoked>), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn host_token_revoke<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Path(token_id): Path<String>,
) -> Result<Json<Envelope<FeedTokenRevoked>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let result = revoke(state.kernel().db.pool(), &actor, &token_id).await?;
    Ok(Json(Envelope::ok(result, request_id)))
}

#[utoipa::path(
    get, path = "/api/public/calendar/feeds/{city}/{token}", operation_id = "inventory_feed_by_token",
    params(("city" = String, Path), ("token" = String, Path)),
    responses((status = 200, content_type = "text/calendar", body = String), (status = 404, body = ErrorBody))
)]
pub async fn public_feed<S>(
    State(state): State<S>,
    Path((city, token)): Path<(String, String)>,
) -> Result<Response>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let city = City::from_code(&city).map_err(|_| Error::not_found(FEED_NOT_FOUND))?;
    let token = token.strip_suffix(".ics").unwrap_or(&token);
    Ok(calendar_response(feed_by_token(state.kernel().db.pool(), city, token).await?))
}
