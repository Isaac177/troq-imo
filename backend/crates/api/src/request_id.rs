use axum::extract::Request;
use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::Response;
use kernel::auth::request::{request_id, REQUEST_ID_HEADER};

pub async fn propagate(request: Request, next: Next) -> Response {
    let (mut parts, body) = request.into_parts();
    let id = request_id(&parts);
    if let Ok(value) = HeaderValue::from_str(&id) {
        parts.headers.insert(REQUEST_ID_HEADER, value.clone());
        let mut response = next.run(Request::from_parts(parts, body)).await;
        response.headers_mut().insert(REQUEST_ID_HEADER, value);
        response
    } else {
        next.run(Request::from_parts(parts, body)).await
    }
}

#[cfg(test)]
mod tests {
    use axum::body::{to_bytes, Body};
    use axum::http::{header::HeaderValue, Request};
    use axum::{middleware, routing::get, Router};
    use kernel::auth::RequestId;
    use tower::ServiceExt;

    use super::propagate;

    async fn echo(RequestId(id): RequestId) -> String {
        id
    }

    fn app() -> Router {
        Router::new()
            .route("/", get(echo))
            .layer(middleware::from_fn(propagate))
    }

    #[tokio::test]
    async fn response_and_extractor_share_generated_id() {
        let response = app().oneshot(Request::new(Body::empty())).await.unwrap();
        let id = response.headers()["x-request-id"].clone();
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        assert!(!id.is_empty());
        assert_eq!(body.as_ref(), id.as_bytes());
    }

    #[tokio::test]
    async fn supplied_id_is_reused() {
        let request = Request::builder()
            .header("x-request-id", "example-request")
            .body(Body::empty())
            .unwrap();
        let response = app().oneshot(request).await.unwrap();
        assert_eq!(response.headers()["x-request-id"], HeaderValue::from_static("example-request"));
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        assert_eq!(body.as_ref(), b"example-request");
    }
}
