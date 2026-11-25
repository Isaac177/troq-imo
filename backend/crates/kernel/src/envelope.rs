use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub request_id: String,
}

#[derive(Debug, Serialize)]
pub struct Envelope<T: Serialize> {
    pub success: bool,
    pub data: T,
    pub metadata: Metadata,
}

impl<T: Serialize> Envelope<T> {
    pub fn ok(data: T, request_id: impl Into<String>) -> Self {
        Self {
            success: true,
            data,
            metadata: Metadata {
                request_id: request_id.into(),
            },
        }
    }
}

impl<T: Serialize> IntoResponse for Envelope<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_envelope_carries_data_and_request_id() {
        let value = serde_json::to_value(Envelope::ok(vec![1], "req-1")).unwrap();
        assert_eq!(value["success"], true);
        assert_eq!(value["data"][0], 1);
        assert_eq!(value["metadata"]["requestId"], "req-1");
    }
}
