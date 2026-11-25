use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use kernel::auth::RequestId;
use kernel::envelope::Envelope;
use kernel::kernel::Kernel;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Health {
    database: bool,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HealthMetadata {
    request_id: String,
}

#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    success: bool,
    data: Health,
    metadata: HealthMetadata,
}

#[utoipa::path(
    get,
    path = "/api/public/health",
    operation_id = "health_get",
    params(("x-request-id" = Option<String>, Header)),
    responses(
        (status = 200, body = HealthResponse, headers(("x-request-id" = String))),
        (status = 503, body = HealthResponse, headers(("x-request-id" = String)))
    )
)]
pub async fn health(State(kernel): State<Kernel>, RequestId(request_id): RequestId) -> Response {
    let database = kernel.db.ping().await;
    let status = if database { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE };
    (status, Json(Envelope::ok(Health { database }, request_id))).into_response()
}
