use kernel::{City, Kernel, Result};
use serde_json::Value;

use super::channels::email::{self, Attachment};
use super::channels::whatsapp;
use super::config::NotificationsConfig;
use super::deliveries::Delivery;
use super::documents::{self, DocumentKind, Locale};
use super::links::deep_link;

fn contact<'a>(snapshot: &'a Value, field: &str) -> Option<&'a str> {
    snapshot
        .get("guest")
        .and_then(|guest| guest.get(field))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn failure(error: impl std::fmt::Display) -> kernel::Error {
    kernel::Error::internal(error.to_string())
}

pub async fn run(kernel: &Kernel, config: &NotificationsConfig, delivery: &Delivery) -> Result<&'static str> {
    let kind = delivery
        .payload
        .get("document")
        .and_then(Value::as_str)
        .and_then(DocumentKind::parse)
        .ok_or_else(|| failure("document delivery without a known kind"))?;
    let snapshot = delivery.payload.get("snapshot").cloned().unwrap_or(Value::Null);
    let locale = Locale::parse(snapshot.get("language").and_then(Value::as_str));
    let city = City::from_code(&delivery.city)?;
    let Some(issued) = documents::issue(kernel, config, kind, city, &snapshot, locale).await? else {
        return Ok("already issued or not applicable");
    };
    if let Some(to) = contact(&snapshot, "contactEmail").filter(|value| whatsapp::has_deliverable_email(value)) {
        let attachment = Attachment {
            filename: issued.generated.file_name.clone(),
            content: issued.generated.bytes.clone(),
            content_type: Some("application/pdf".to_string()),
        };
        email::send(config, to, &issued.subject, &issued.body, None, &[attachment]).await.map_err(failure)?;
        return Ok("emailed");
    }
    if let Some(phone) = contact(&snapshot, "contactPhone") {
        let reservation_id = snapshot.get("reservationId").and_then(Value::as_str);
        let link = format!(
            "{}/{}{}",
            config.base_url("guest").trim_end_matches('/'),
            locale.code(),
            deep_link("guest", Some("reservation"), reservation_id)
        );
        let text = whatsapp::compact(&issued.subject, "", Some(&link));
        whatsapp::send_message(config, phone, &text).await.map_err(failure)?;
        return Ok("whatsapp");
    }
    Ok("no contact")
}
