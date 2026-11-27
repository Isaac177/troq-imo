use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::extract::{Path, Query, State};
use axum::response::Response;
use axum::routing::{get, post, put};
use axum::{Json, Router};
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::ErrorBody;
use kernel::{KernelState, Page, Result};

use super::api_queries::{DocumentsQuery, ListQuery};
use super::api_shared::{self as shared, Audience};
use super::types::{DocumentView, MarkedCount, NotificationView, PreferenceList, PreferenceView, PushTokenInput, PushTokenRemoval, UnreadCount};

const AUDIENCE: Audience = Audience::Host;

type Reply<T> = Result<Json<Envelope<T>>>;
type JsonBody<T> = std::result::Result<Json<T>, JsonRejection>;

fn ok<T: serde::Serialize>(data: T, request_id: String) -> Reply<T> {
    Ok(Json(Envelope::ok(data, request_id)))
}

fn body<T>(input: JsonBody<T>) -> Result<T> {
    input.map(|Json(value)| value).map_err(|_| shared::malformed())
}

fn query<T>(input: std::result::Result<Query<T>, QueryRejection>) -> Result<T> {
    input.map(|Query(value)| value).map_err(|_| shared::malformed())
}

#[utoipa::path(get, path = "/api/host/notifications", operation_id = "notifications_host_list", tag = "notifications",
    params(ListQuery, ("x-request-id" = Option<String>, Header)),
    responses((status = 200, body = Envelope<Page<NotificationView>>), (status = 400, body = ErrorBody)))]
pub async fn list<S: KernelState + Clone + Send + Sync + 'static>(State(state): State<S>, actor: Actor, RequestId(request_id): RequestId, input: std::result::Result<Query<ListQuery>, QueryRejection>) -> Reply<Page<NotificationView>> {
    ok(shared::list(state.kernel(), &actor, AUDIENCE, query(input)?).await?, request_id)
}

#[utoipa::path(get, path = "/api/host/notifications/unread-count", operation_id = "notifications_host_unread_count", tag = "notifications",
    params(("x-request-id" = Option<String>, Header)),
    responses((status = 200, body = Envelope<UnreadCount>)))]
pub async fn unread_count<S: KernelState + Clone + Send + Sync + 'static>(State(state): State<S>, actor: Actor, RequestId(request_id): RequestId) -> Reply<UnreadCount> {
    ok(shared::unread(state.kernel(), &actor, AUDIENCE).await?, request_id)
}

#[utoipa::path(post, path = "/api/host/notifications/{id}/read", operation_id = "notifications_host_mark_read", tag = "notifications",
    params(("id" = String, Path), ("x-request-id" = Option<String>, Header)),
    responses((status = 200, body = Envelope<MarkedCount>), (status = 404, body = ErrorBody)))]
pub async fn mark_read<S: KernelState + Clone + Send + Sync + 'static>(State(state): State<S>, actor: Actor, RequestId(request_id): RequestId, Path(id): Path<String>) -> Reply<MarkedCount> {
    ok(shared::read_one(state.kernel(), &actor, AUDIENCE, &id).await?, request_id)
}

#[utoipa::path(post, path = "/api/host/notifications/read-all", operation_id = "notifications_host_mark_all_read", tag = "notifications",
    params(("x-request-id" = Option<String>, Header)),
    responses((status = 200, body = Envelope<MarkedCount>)))]
pub async fn mark_all_read<S: KernelState + Clone + Send + Sync + 'static>(State(state): State<S>, actor: Actor, RequestId(request_id): RequestId) -> Reply<MarkedCount> {
    ok(shared::read_all(state.kernel(), &actor, AUDIENCE).await?, request_id)
}

#[utoipa::path(get, path = "/api/host/notification-preferences", operation_id = "notifications_host_preferences", tag = "notifications",
    params(("x-request-id" = Option<String>, Header)),
    responses((status = 200, body = Envelope<PreferenceList>)))]
pub async fn preferences<S: KernelState + Clone + Send + Sync + 'static>(State(state): State<S>, actor: Actor, RequestId(request_id): RequestId) -> Reply<PreferenceList> {
    ok(shared::preferences(state.kernel(), &actor).await?, request_id)
}

#[utoipa::path(put, path = "/api/host/notification-preferences", operation_id = "notifications_host_set_preference", tag = "notifications",
    params(("x-request-id" = Option<String>, Header)), request_body = PreferenceView,
    responses((status = 200, body = Envelope<PreferenceView>), (status = 400, body = ErrorBody)))]
pub async fn set_preference<S: KernelState + Clone + Send + Sync + 'static>(State(state): State<S>, actor: Actor, RequestId(request_id): RequestId, input: JsonBody<PreferenceView>) -> Reply<PreferenceView> {
    ok(shared::set_preference(state.kernel(), &actor, body(input)?).await?, request_id)
}

#[utoipa::path(put, path = "/api/host/push-tokens", operation_id = "notifications_host_register_token", tag = "notifications",
    params(("x-request-id" = Option<String>, Header)), request_body = PushTokenInput,
    responses((status = 200, body = Envelope<MarkedCount>), (status = 400, body = ErrorBody)))]
pub async fn register_token<S: KernelState + Clone + Send + Sync + 'static>(State(state): State<S>, actor: Actor, RequestId(request_id): RequestId, input: JsonBody<PushTokenInput>) -> Reply<MarkedCount> {
    ok(shared::register_token(state.kernel(), &actor, body(input)?).await?, request_id)
}

#[utoipa::path(delete, path = "/api/host/push-tokens", operation_id = "notifications_host_remove_token", tag = "notifications",
    params(("x-request-id" = Option<String>, Header)), request_body = PushTokenRemoval,
    responses((status = 200, body = Envelope<MarkedCount>)))]
pub async fn remove_token<S: KernelState + Clone + Send + Sync + 'static>(State(state): State<S>, actor: Actor, RequestId(request_id): RequestId, input: JsonBody<PushTokenRemoval>) -> Reply<MarkedCount> {
    ok(shared::remove_token(state.kernel(), &actor, body(input)?).await?, request_id)
}

#[utoipa::path(get, path = "/api/host/documents", operation_id = "notifications_host_documents", tag = "notifications",
    params(DocumentsQuery, ("x-request-id" = Option<String>, Header)),
    responses((status = 200, body = Envelope<Page<DocumentView>>), (status = 400, body = ErrorBody)))]
pub async fn documents<S: KernelState + Clone + Send + Sync + 'static>(State(state): State<S>, actor: Actor, RequestId(request_id): RequestId, input: std::result::Result<Query<DocumentsQuery>, QueryRejection>) -> Reply<Page<DocumentView>> {
    ok(shared::documents(state.kernel(), &actor, AUDIENCE, query(input)?).await?, request_id)
}

#[utoipa::path(get, path = "/api/host/documents/{id}/download", operation_id = "notifications_host_document_download", tag = "notifications",
    params(("id" = String, Path)),
    responses((status = 200, content_type = "application/pdf", body = Vec<u8>), (status = 404, body = ErrorBody)))]
pub async fn document_download<S: KernelState + Clone + Send + Sync + 'static>(State(state): State<S>, actor: Actor, Path(id): Path<String>) -> Result<Response> {
    shared::download(state.kernel(), &actor, AUDIENCE, &id).await
}

pub fn routes<S: KernelState + Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/notifications", get(list::<S>))
        .route("/notifications/unread-count", get(unread_count::<S>))
        .route("/notifications/read-all", post(mark_all_read::<S>))
        .route("/notifications/:id/read", post(mark_read::<S>))
        .route("/notification-preferences", get(preferences::<S>).put(set_preference::<S>))
        .route("/push-tokens", put(register_token::<S>).delete(remove_token::<S>))
        .route("/documents", get(documents::<S>))
        .route("/documents/:id/download", get(document_download::<S>))
}
