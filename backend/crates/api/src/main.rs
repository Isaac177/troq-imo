mod amenities;
mod app;
mod audience;
mod contract;
mod cors;
mod directory;
mod health;
mod registry;
mod request_id;
mod spec;

use std::sync::Arc;

use kernel::config::Config;
use kernel::kernel::Kernel;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::args().nth(1).as_deref() == Some("openapi") {
        println!("{}", serde_json::to_string_pretty(&spec::document()?)?);
        return Ok(());
    }

    let config = Config::from_env()?;
    let address = config.bind_addr();
    let cors = cors::layer(&config.cors_allowed_origins)?;
    let kernel = Kernel::connect(config, Arc::new(directory::ClosedDirectory)).await?;
    let app = app::router(kernel)
        .layer(cors)
        .layer(axum::middleware::from_fn(request_id::propagate));
    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
