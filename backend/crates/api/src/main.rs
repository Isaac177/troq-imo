mod directory;
mod health;
mod spec;
mod request_id;

use std::sync::Arc;

use axum::{routing::get, Router};
use kernel::config::Config;
use kernel::kernel::Kernel;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::args().nth(1).as_deref() == Some("openapi") {
        println!("{}", serde_json::to_string_pretty(&spec::document())?);
        return Ok(());
    }

    let config = Config::from_env()?;
    let address = config.bind_addr();
    let kernel = Kernel::connect(config, Arc::new(directory::ClosedDirectory)).await?;
    let app = Router::new()
        .route("/api/public/health", get(health::health))
        .route("/api/openapi.json", get(spec::serve))
        .with_state(kernel)
        .layer(axum::middleware::from_fn(request_id::propagate));
    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
