mod adapters;

use std::sync::Arc;
use std::time::Duration;

use domain::notifications::NotificationsConfig;
use kernel::Kernel;
use tokio::task::JoinSet;

use crate::schedule::every;
use crate::stop::Stop;

const SWEEP_INTERVAL: Duration = Duration::from_secs(30);
const NOTIFICATION_INTERVAL: Duration = Duration::from_secs(5);
const NOTIFICATION_BATCH: usize = 50;
const HOLD_EXPIRY_BATCH: usize = 100;
const GUEST_RETENTION_BATCH: usize = 100;
const SEARCH_EVENT_PURGE_BATCH: usize = 1000;

pub fn spawn(tasks: &mut JoinSet<()>, kernel: &Kernel, stop: &Stop) {
    tasks.spawn(every(
        "hold_expiry",
        SWEEP_INTERVAL,
        HOLD_EXPIRY_BATCH,
        kernel.clone(),
        stop.clone(),
        |kernel| async move { adapters::expire_due_holds(&kernel, HOLD_EXPIRY_BATCH as i64).await },
    ));
    match kernel.config.guest_retention_days {
        Some(days) => {
            tracing::info!(days, "guest retention enabled");
            tasks.spawn(every(
                "guest_retention",
                SWEEP_INTERVAL,
                GUEST_RETENTION_BATCH,
                kernel.clone(),
                stop.clone(),
                |kernel| async move {
                    adapters::anonymize_expired_guests(&kernel, GUEST_RETENTION_BATCH as i64).await
                },
            ));
        }
        None => tracing::warn!(
            "GUEST_RETENTION_DAYS is not set: guest data is kept, nothing is anonymized"
        ),
    }
    tasks.spawn(every(
        "search_event_purge",
        SWEEP_INTERVAL,
        SEARCH_EVENT_PURGE_BATCH,
        kernel.clone(),
        stop.clone(),
        |kernel| async move {
            adapters::purge_search_events(&kernel, SEARCH_EVENT_PURGE_BATCH as i64).await
        },
    ));
    let config = Arc::new(NotificationsConfig::from_env());
    tasks.spawn(every(
        "notification_delivery",
        NOTIFICATION_INTERVAL,
        NOTIFICATION_BATCH,
        (kernel.clone(), config),
        stop.clone(),
        |(kernel, config)| async move {
            adapters::deliver_notifications(&kernel, &config, NOTIFICATION_BATCH).await
        },
    ));
}
