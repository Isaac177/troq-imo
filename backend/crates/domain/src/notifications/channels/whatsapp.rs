use anyhow::{bail, Result};

use crate::notifications::config::NotificationsConfig;

const MAX_BODY_CHARS: usize = 220;
const PLACEHOLDER_DOMAIN: &str = "@mungini.local";
const ENDPOINT: &str = "https://www.wasenderapi.com/api/send-message";

pub fn has_deliverable_email(email: &str) -> bool {
    let email = email.trim();
    email.contains('@') && !email.to_ascii_lowercase().ends_with(PLACEHOLDER_DOMAIN)
}

pub fn has_phone(phone: &str) -> bool {
    phone.chars().filter(|c| c.is_ascii_digit()).count() >= 8
}

fn shorten(text: &str) -> String {
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if text.chars().count() <= MAX_BODY_CHARS {
        return text;
    }
    let cut: String = text.chars().take(MAX_BODY_CHARS - 1).collect();
    format!("{}…", cut.trim_end())
}

pub fn compact(title: &str, body: &str, link: Option<&str>) -> String {
    let mut lines = vec![format!("*{}*", title.trim())];
    let body = shorten(body);
    if !body.is_empty() && body != title.trim() {
        lines.push(body);
    }
    if let Some(link) = link.map(str::trim).filter(|value| !value.is_empty()) {
        lines.push(link.to_string());
    }
    lines.join("\n")
}

fn mask_phone(value: &str) -> String {
    let digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() <= 4 {
        return "***".to_string();
    }
    format!("***{}", &digits[digits.len() - 4..])
}

pub async fn send_message(config: &NotificationsConfig, to: &str, message: &str) -> Result<()> {
    if config.whatsapp_access_token.is_empty() {
        bail!("WHATSAPP_ACCESS_TOKEN is empty");
    }
    let response = reqwest::Client::new()
        .post(ENDPOINT)
        .bearer_auth(&config.whatsapp_access_token)
        .json(&serde_json::json!({ "to": to, "text": message }))
        .send()
        .await?;
    let status = response.status();
    if status.is_success() {
        return Ok(());
    }
    let body = response.text().await.unwrap_or_default();
    let preview: String = body.chars().take(300).collect();
    tracing::warn!(to = %mask_phone(to), status = %status, "notifications.whatsapp.request_failed");
    bail!("whatsapp provider error: status={status} body={preview}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_and_malformed_emails_are_not_deliverable() {
        assert!(has_deliverable_email("ada@example.com"));
        assert!(!has_deliverable_email("ada@mungini.local"));
        assert!(!has_deliverable_email("not-an-email"));
    }

    #[test]
    fn phones_need_eight_digits() {
        assert!(has_phone("+243 811 111 111"));
        assert!(!has_phone("1234567"));
    }

    #[test]
    fn compact_message_has_title_body_and_link() {
        let text = compact("Title", "Body text", Some(" https://x.example "));
        assert_eq!(text, "*Title*\nBody text\nhttps://x.example");
        assert_eq!(compact("Same", "Same", None), "*Same*");
    }

    #[test]
    fn long_bodies_are_cut_with_an_ellipsis() {
        let text = compact("T", &"word ".repeat(100), None);
        assert!(text.ends_with('…'));
        assert!(text.chars().count() < MAX_BODY_CHARS + 10);
    }

    #[test]
    fn phone_numbers_are_masked_in_logs() {
        assert_eq!(mask_phone("+243811111111"), "***1111");
        assert_eq!(mask_phone("12"), "***");
    }
}
