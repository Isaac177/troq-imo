mod backoff;
pub mod delivery;
pub mod event;
mod notifications;
pub mod pattern;
pub mod registry;
mod schedule;
mod stop;
pub mod subscribers;
mod tasks;
pub mod worker;

use std::future::Future;

use kernel::KernelState;
use tokio::sync::watch;
use tokio::task::JoinSet;

pub use delivery::TransactionalSubscriber;
pub use event::Event;
pub use registry::Registry;

pub async fn run<S, F>(state: S, shutdown: F)
where
    S: KernelState + Send + Sync + 'static,
    F: Future<Output = ()> + Send + 'static,
{
    let kernel = state.kernel().clone();
    let (stop_sender, stop) = watch::channel(false);
    let mut registry = Registry::new();
    subscribers::register(&mut registry);

    let mut scheduled = JoinSet::new();
    tasks::spawn(&mut scheduled, &kernel, &stop);

    tokio::spawn(async move {
        shutdown.await;
        let _ = stop_sender.send(true);
    });

    if registry.is_empty() {
        tracing::warn!("no outbox subscriber is registered: events stay pending");
        stop::wait(&mut stop.clone()).await;
    } else {
        let subscribers = registry.as_refs();
        if let Err(error) = worker::run(kernel.db.pool(), &subscribers, stop.clone()).await {
            tracing::error!(error = %error, "outbox worker stopped");
        }
    }
    while scheduled.join_next().await.is_some() {}
}

#[cfg(test)]
mod delivery_tests;
