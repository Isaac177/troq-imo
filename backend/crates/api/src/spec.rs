use axum::Json;
use utoipa::OpenApi;

use crate::health::{Health, HealthMetadata, HealthResponse};

#[derive(OpenApi)]
#[openapi(paths(crate::health::health), components(schemas(Health, HealthMetadata, HealthResponse)))]
struct ApiDoc;

pub fn document() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}

pub async fn serve() -> Json<utoipa::openapi::OpenApi> {
    Json(document())
}
