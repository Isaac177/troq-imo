use kernel::{Error, Kernel, Result};
use serde_json::Value;

use super::channels::email_html::NotificationMail;
use super::channels::{email, push, whatsapp};
use super::config::NotificationsConfig;
use super::contacts::{Contact, ContactDirectory};
use super::deliveries::Delivery;
use super::render::pick;

fn text<'a>(payload: &'a Value, key: &str) -> &'a str {
    payload.get(key).and_then(Value::as_str).unwrap_or_default()
}

fn failure(error: impl std::fmt::Display) -> Error {
    Error::internal(error.to_string())
}

pub fn action_url(config: &NotificationsConfig, audience: &str, deep_link: &str, language: &str) -> Option<String> {
    if deep_link.is_empty() {
        return None;
    }
    let lang = if language.to_ascii_lowercase().starts_with("en") { "en" } else { "fr" };
    Some(format!("{}/{}{}", config.base_url(audience).trim_end_matches('/'), lang, deep_link))
}

pub async fn run_push(kernel: &Kernel, config: &NotificationsConfig, delivery: &Delivery, language: &str) -> Result<()> {
    let (Some(user_id), Some(notification_id)) = (delivery.user_id.as_deref(), delivery.notification_id.as_deref()) else {
        return Ok(());
    };
    let payload = &delivery.payload;
    let event_key = sqlx::query_scalar::<_, Option<String>>("SELECT event_key FROM notifications WHERE id = $1")
        .bind(notification_id)
        .fetch_optional(kernel.db.pool())
        .await?
        .flatten();
    let message = push::PushMessage {
        notification_id,
        user_id,
        title: pick(language, text(payload, "titleEn"), text(payload, "titleFr")),
        body: pick(language, text(payload, "bodyEn"), text(payload, "bodyFr")),
        event_key: event_key.as_deref(),
        deep_link: Some(text(payload, "deepLink")).filter(|value| !value.is_empty()),
        audience: text(payload, "audience"),
    };
    push::deliver(kernel.db.pool(), config, &message).await?;
    Ok(())
}

pub async fn run_email(
    config: &NotificationsConfig,
    delivery: &Delivery,
    directory: &dyn ContactDirectory,
) -> Result<&'static str> {
    let Some(user_id) = delivery.user_id.clone() else {
        return Ok("no user");
    };
    let contact: Contact = directory
        .contacts(std::slice::from_ref(&user_id))
        .await?
        .into_iter()
        .find(|found| found.user_id == user_id)
        .unwrap_or_default();
    let language = if contact.language.is_empty() { "fr" } else { contact.language.as_str() };
    let payload = &delivery.payload;
    let url = action_url(config, text(payload, "audience"), text(payload, "deepLink"), language);
    if whatsapp::has_deliverable_email(&contact.email) {
        let mail = NotificationMail {
            brand: &config.product_name,
            title_fr: text(payload, "titleFr"),
            title_en: text(payload, "titleEn"),
            body_fr: text(payload, "bodyFr"),
            body_en: text(payload, "bodyEn"),
            action_url: url.as_deref(),
        };
        email::send(config, &contact.email, &mail.subject(), &mail.text(), Some(&mail.html()), &[]).await.map_err(failure)?;
        return Ok("emailed");
    }
    if whatsapp::has_phone(&contact.phone) {
        let title = pick(language, text(payload, "titleEn"), text(payload, "titleFr"));
        let body = pick(language, text(payload, "bodyEn"), text(payload, "bodyFr"));
        let message = whatsapp::compact(title, body, url.as_deref());
        whatsapp::send_message(config, &contact.phone, &message).await.map_err(failure)?;
        return Ok("whatsapp");
    }
    Ok("no contact")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_url_prefixes_the_language_and_trims_the_base() {
        let mut config = NotificationsConfig::from_env();
        config.host_base_url = "https://host.example/".into();
        assert_eq!(
            action_url(&config, "host", "/reservations?reservation=r1", "en-GB").as_deref(),
            Some("https://host.example/en/reservations?reservation=r1")
        );
        assert_eq!(action_url(&config, "host", "/x", "sw").as_deref(), Some("https://host.example/fr/x"));
        assert!(action_url(&config, "host", "", "en").is_none());
    }
}
