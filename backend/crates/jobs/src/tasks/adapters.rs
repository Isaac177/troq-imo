use domain::notifications::{self, NotificationsConfig};
use kernel::{Kernel, Result};

pub async fn expire_due_holds(kernel: &Kernel, batch: i64) -> Result<usize> {
    domain::reservations::expire_due_holds(kernel, batch).await
}

pub async fn anonymize_expired_guests(kernel: &Kernel, batch: i64) -> Result<usize> {
    domain::guests::anonymize_expired_guests(kernel, batch).await
}

pub async fn purge_search_events(kernel: &Kernel, batch: i64) -> Result<usize> {
    domain::guests::purge_search_events(kernel, batch).await
}

pub async fn deliver_notifications(
    kernel: &Kernel,
    config: &NotificationsConfig,
    limit: usize,
) -> Result<usize> {
    let contacts = domain::identity::IdentityContacts::new(kernel.db.pool().clone());
    notifications::deliver_pending(kernel, config, Some(&contacts), limit).await
}
