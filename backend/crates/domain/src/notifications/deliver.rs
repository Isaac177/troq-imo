use kernel::{Kernel, Result};

use super::config::NotificationsConfig;
use super::contacts::ContactDirectory;
use super::deliveries::{self, Channel, Delivery};
use super::deliver_document;
use super::deliver_notice;

async fn process(
    kernel: &Kernel,
    config: &NotificationsConfig,
    contacts: Option<&dyn ContactDirectory>,
    delivery: &Delivery,
) -> Result<Option<&'static str>> {
    match Channel::parse(&delivery.channel) {
        Some(Channel::Push) => {
            deliver_notice::run_push(kernel, config, delivery, "fr").await?;
            Ok(None)
        }
        Some(Channel::Email) if super::invitation::is_invitation(delivery) => {
            super::invitation::deliver(kernel, config, delivery).await.map(Some)
        }
        Some(Channel::Email) => match contacts {
            Some(directory) => deliver_notice::run_email(config, delivery, directory).await.map(Some),
            None => Ok(Some("no contact directory")),
        },
        Some(Channel::Document) => deliver_document::run(kernel, config, contacts, delivery).await.map(Some),
        Some(Channel::Whatsapp) | None => Ok(Some("unsupported channel")),
    }
}

pub async fn deliver_pending(
    kernel: &Kernel,
    config: &NotificationsConfig,
    contacts: Option<&dyn ContactDirectory>,
    limit: usize,
) -> Result<usize> {
    let channels = vec![Channel::Push.as_str(), Channel::Document.as_str(), Channel::Email.as_str()];
    let mut handled = 0;
    while handled < limit {
        let mut tx = kernel.db.pool().begin().await?;
        let Some(delivery) = deliveries::claim_next(&mut tx, &channels).await? else {
            tx.commit().await?;
            break;
        };
        match process(kernel, config, contacts, &delivery).await {
            Ok(Some("no contact" | "no user" | "no contact directory")) if delivery.channel == Channel::Email.as_str() || delivery.channel == Channel::Document.as_str() => {
                deliveries::defer_contact(&mut tx, &delivery.id).await?
            }
            Ok(Some(note)) if note.starts_with("no ") || note.starts_with("unsupported") => {
                deliveries::mark_skipped(&mut tx, &delivery.id, note).await?
            }
            Ok(_) => deliveries::mark_done(&mut tx, &delivery.id).await?,
            Err(error) => {
                tracing::warn!(channel = %delivery.channel, error = %error, "notifications.delivery_failed");
                deliveries::mark_failed(&mut tx, &delivery, error.code().as_str()).await?;
            }
        }
        tx.commit().await?;
        handled += 1;
    }
    Ok(handled)
}
