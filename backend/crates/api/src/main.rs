mod amenities;
mod app;
mod audience;
mod contract;
mod cors;
mod directory;
mod health;
mod logging;
mod registry;
mod quote_checks;
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

    logging::init();
    if let Err(error) = run().await {
        tracing::error!(error = %format!("{error:#}"), "server stopped with an error");
        return Err(error);
    }
    Ok(())
}

async fn run() -> anyhow::Result<()> {
    tracing::info!("starting server");
    let config = Config::from_env()?;
    let address = config.bind_addr();
    let cors = cors::layer(&config.cors_allowed_origins)?;
    let kernel = Kernel::connect(config, Arc::new(directory::ClosedDirectory)).await?;
    let directory = Arc::new(domain::identity::IdentityDirectory::new(kernel.db.pool().clone()));
    let kernel = kernel.with_directory(directory);
    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!(address = %listener.local_addr()?, jobs_enabled = kernel.config.jobs_enabled, "server listening");
    let (stop, mut stopped) = tokio::sync::watch::channel(false);
    let worker = if kernel.config.jobs_enabled {
        let state = kernel.clone();
        Some(tokio::spawn(async move {
            jobs::run(state, async move {
                while !*stopped.borrow() {
                    if stopped.changed().await.is_err() { break; }
                }
            }).await;
        }))
    } else {
        None
    };
    let app = app::router(kernel)
        .layer(cors)
        .layer(axum::middleware::from_fn(logging::request))
        .layer(axum::middleware::from_fn(request_id::propagate));
    let result = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await;
    let _ = stop.send(true);
    if let Some(worker) = worker { worker.await?; }
    result?;
    tracing::info!("server stopped");
    Ok(())
}

async fn shutdown_signal() {
    #[cfg(unix)]
    if let Ok(mut terminate) = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {},
            _ = terminate.recv() => {},
        }
        return;
    }
    let _ = tokio::signal::ctrl_c().await;
}
