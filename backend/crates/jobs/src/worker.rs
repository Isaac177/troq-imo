use std::time::Duration;

use kernel::{Error, Result};
use sqlx::PgPool;
use tokio::sync::watch;

use crate::delivery::{deliver_one, TransactionalSubscriber};

const IDLE_INTERVAL: Duration = Duration::from_secs(1);
const ERROR_INTERVAL: Duration = Duration::from_secs(5);

pub async fn run(
    pool: &PgPool,
    subscribers: &[&dyn TransactionalSubscriber],
    mut stop: watch::Receiver<bool>,
) -> Result<()> {
    if subscribers.is_empty() {
        return Err(Error::internal("outbox worker needs at least one subscriber"));
    }
    loop {
        if *stop.borrow() {
            return Ok(());
        }
        let delay = match deliver_one(pool, subscribers).await {
            Ok(true) => continue,
            Ok(false) => IDLE_INTERVAL,
            Err(error) => {
                tracing::error!(code = %error.code().as_str(), "outbox delivery failed");
                ERROR_INTERVAL
            }
        };
        tokio::select! {
            _ = tokio::time::sleep(delay) => {},
            changed = stop.changed() => {
                if changed.is_err() || *stop.borrow() {
                    return Ok(());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn rejects_start_without_subscribers() {
        let pool = PgPoolOptions::new()
            .connect_lazy("postgres://localhost/mungini")
            .unwrap();
        let (_sender, stop) = watch::channel(false);
        assert!(run(&pool, &[], stop).await.is_err());
    }
}
