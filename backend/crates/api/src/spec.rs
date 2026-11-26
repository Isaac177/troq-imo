use axum::Json;
use kernel::error::{Error, Result};
use serde_json::Value;
use utoipa::OpenApi;

use crate::contract;
use crate::health::{Health, HealthMetadata, HealthResponse};
use crate::registry;

#[derive(OpenApi)]
#[openapi(
    info(title = "Mungini API", version = "0.1.0"),
    paths(crate::health::health, crate::amenities::groups, crate::amenities::amenities),
    components(schemas(Health, HealthMetadata, HealthResponse))
)]
struct ApiDoc;

pub fn document() -> anyhow::Result<Value> {
    let mut merged = ApiDoc::openapi();
    for module in registry::documents() {
        merged.merge(module);
    }
    let mut value = serde_json::to_value(&merged)?;
    contract::finish(&mut value)?;
    Ok(value)
}

pub async fn serve() -> Result<Json<Value>> {
    document()
        .map(Json)
        .map_err(|error| Error::internal(error.to_string()))
}
