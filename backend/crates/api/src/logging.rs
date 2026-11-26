use std::fmt;
use std::time::Instant;

use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use chrono::{Duration, Utc};
use tracing::Instrument;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;

struct GmtPlusThree;

impl FormatTime for GmtPlusThree {
    fn format_time(&self, writer: &mut Writer<'_>) -> fmt::Result {
        write!(writer, "{} +03:00", (Utc::now() + Duration::hours(3)).format("%d/%m/%Y %H:%M:%S%.3f"))
    }
}

pub fn init() {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "trace".to_string());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_timer(GmtPlusThree)
        .with_ansi(true)
        .with_writer(std::io::stderr)
        .init();
}

pub async fn request(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let path = request.uri().path().to_owned();
    let request_id = request.headers().get("x-request-id").and_then(|value| value.to_str().ok()).unwrap_or("").to_owned();
    let span = tracing::info_span!("request", %request_id, %method, %path);
    async move {
        let started = Instant::now();
        tracing::info!("request started");
        let response = next.run(request).await;
        let status = response.status();
        let elapsed_ms = started.elapsed().as_millis() as u64;
        if status.is_server_error() {
            tracing::error!(%status, elapsed_ms, "request completed");
        } else if status.is_client_error() {
            tracing::warn!(%status, elapsed_ms, "request completed");
        } else {
            tracing::info!(%status, elapsed_ms, "request completed");
        }
        response
    }.instrument(span).await
}
