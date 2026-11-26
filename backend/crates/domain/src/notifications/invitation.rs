use kernel::{Error, Kernel, Result};
use serde_json::json;
use sqlx::PgConnection;

use super::channels::email;
use super::channels::email_html::NotificationMail;
use super::config::NotificationsConfig;
use super::deliveries::{self, Channel, Delivery, NewDelivery};
use super::dispatch::EventContext;

pub async fn enqueue(conn: &mut PgConnection, ctx: &EventContext<'_>) -> Result<()> {
    let key = format!("{}:EMAIL:invitation", ctx.event_id);
    deliveries::enqueue(conn, NewDelivery {
        city: ctx.city,
        dedupe_key: &key,
        channel: Channel::Email,
        user_id: None,
        notification_id: None,
        payload: json!({ "kind": "identity_invitation", "invitationId": ctx.aggregate_id }),
    }).await
}

pub fn is_invitation(delivery: &Delivery) -> bool {
    delivery.payload.get("kind").and_then(|value| value.as_str()) == Some("identity_invitation")
}

pub async fn deliver(kernel: &Kernel, config: &NotificationsConfig, delivery: &Delivery) -> Result<&'static str> {
    let id = delivery.payload.get("invitationId").and_then(|value| value.as_str())
        .filter(|value| !value.is_empty()).ok_or_else(|| Error::internal("invitation delivery missing invitation id"))?;
    let row: Option<(String, String)> = sqlx::query_as(
        "SELECT i.email, h.display_name FROM identity_invitations i JOIN hosts h ON h.id = i.host_id WHERE i.id = $1 AND h.city_id = $2 AND h.status = 'ACTIVE' AND i.status = 'PENDING' AND i.expires_at > now()",
    ).bind(id).bind(&delivery.city).fetch_optional(kernel.db.pool()).await?;
    let Some((email_address, host)) = row else { return Ok("no active invitation"); };
    if config.shipmail_api_key.is_empty() { return Err(Error::internal("invitation email provider is not configured")); }
    let template = &super::copy::HOST_INVITATION;
    let city = kernel::City::from_code(&delivery.city)?;
    let body_en = template.body_en.replace("{host}", &host).replace("{city}", city.name());
    let body_fr = template.body_fr.replace("{host}", &host).replace("{city}", city.name());
    let url = format!("{}/account", config.guest_base_url.trim_end_matches('/'));
    let mail = NotificationMail {
        brand: &config.product_name,
        title_fr: template.title_fr,
        title_en: template.title_en,
        body_fr: &body_fr,
        body_en: &body_en,
        action_url: Some(&url),
    };
    let text = format!("{}\n\n{}", mail.text(), url);
    let subject = mail.subject();
    let html = mail.html();
    tokio::time::timeout(std::time::Duration::from_secs(8), email::send(config, &email_address, &subject, &text, Some(&html), &[]))
        .await.map_err(|_| Error::internal("invitation email timed out"))?
        .map_err(|_| Error::internal("invitation email delivery failed"))?;
    Ok("emailed")
}
