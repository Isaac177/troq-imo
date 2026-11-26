use anyhow::{bail, Result};
use base64::Engine;
use serde::Serialize;

use crate::notifications::config::NotificationsConfig;

#[derive(Debug, Clone)]
pub struct Attachment {
    pub filename: String,
    pub content: Vec<u8>,
    pub content_type: Option<String>,
}

#[derive(Debug, Serialize)]
struct WireAttachment {
    filename: String,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<String>,
}

#[derive(Debug, Serialize)]
struct WireMessage {
    from: String,
    to: Vec<String>,
    subject: String,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<Vec<WireAttachment>>,
}

fn wire_attachments(attachments: &[Attachment]) -> Option<Vec<WireAttachment>> {
    if attachments.is_empty() {
        return None;
    }
    Some(
        attachments
            .iter()
            .map(|attachment| WireAttachment {
                filename: attachment.filename.clone(),
                content: base64::engine::general_purpose::STANDARD.encode(&attachment.content),
                content_type: attachment.content_type.clone(),
            })
            .collect(),
    )
}

pub async fn send(
    config: &NotificationsConfig,
    to: &str,
    subject: &str,
    text_body: &str,
    html_body: Option<&str>,
    attachments: &[Attachment],
) -> Result<()> {
    let payload = WireMessage {
        from: config.email_from.clone(),
        to: vec![to.to_string()],
        subject: subject.to_string(),
        text: text_body.to_string(),
        html: html_body.map(ToString::to_string),
        attachments: wire_attachments(attachments),
    };
    let response = reqwest::Client::new()
        .post(&config.shipmail_api_url)
        .bearer_auth(&config.shipmail_api_key)
        .json(&payload)
        .send()
        .await?;
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        tracing::warn!(status = %status, "notifications.email.request_rejected");
        bail!("shipmail send failed with status {status}: {body}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attachments_are_base64_encoded_and_omitted_when_empty() {
        assert!(wire_attachments(&[]).is_none());
        let wire = wire_attachments(&[Attachment {
            filename: "voucher.pdf".into(),
            content: b"pdf".to_vec(),
            content_type: Some("application/pdf".into()),
        }])
        .unwrap();
        assert_eq!(wire[0].content, "cGRm");
        let json = serde_json::to_value(&wire[0]).unwrap();
        assert_eq!(json["content_type"], "application/pdf");
    }
}
