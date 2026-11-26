use kernel::ids::new_id;
use kernel::{Error, Result};
use serde_json::{json, Value};
use sqlx::PgPool;

use super::firebase;
use crate::notifications::codes::PUSH_TOKEN_INVALID;
use crate::notifications::config::NotificationsConfig;
use crate::notifications::types::PushTokenInput;

pub fn token_is_dead(error: &str) -> bool {
    error.contains("UNREGISTERED")
        || error.contains("NOT_FOUND")
        || error.contains("registration-token-not-registered")
        || error.contains("InvalidRegistration")
        || error.contains("NotRegistered")
}

pub async fn active_tokens(pool: &PgPool, user_id: &str) -> Result<Vec<(String, String)>> {
    let rows = sqlx::query_as::<_, (String, String)>("SELECT token, platform FROM fcm_tokens WHERE user_id = $1 AND is_active ORDER BY created_at")
        .bind(user_id)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

async fn record_attempt(pool: &PgPool, notification_id: &str, platform: &str, token: &str, error: Option<&str>, response: Option<Value>) -> Result<()> {
    sqlx::query(
        "INSERT INTO notification_delivery_attempts (id, notification_id, platform, device_token, status, error_message, fcm_response, succeeded_at) VALUES ($1, $2, $3, $4, $5, $6, $7, CASE WHEN $5 = 'SUCCESS' THEN now() ELSE NULL END)",
    )
    .bind(new_id("attempt"))
    .bind(notification_id)
    .bind(platform)
    .bind(token)
    .bind(if error.is_some() { "FAILED" } else { "SUCCESS" })
    .bind(error)
    .bind(response)
    .execute(pool)
    .await?;
    Ok(())
}

async fn deactivate(pool: &PgPool, token: &str) -> Result<()> {
    sqlx::query("UPDATE fcm_tokens SET is_active = false, invalidated_at = now(), updated_at = now() WHERE token = $1")
        .bind(token)
        .execute(pool)
        .await?;
    Ok(())
}

pub struct PushMessage<'a> {
    pub notification_id: &'a str,
    pub user_id: &'a str,
    pub title: &'a str,
    pub body: &'a str,
    pub event_key: Option<&'a str>,
    pub deep_link: Option<&'a str>,
    pub audience: &'a str,
}

pub async fn deliver(pool: &PgPool, config: &NotificationsConfig, message: &PushMessage<'_>) -> Result<usize> {
    let tokens = active_tokens(pool, message.user_id).await?;
    let data = json!({
        "notificationId": message.notification_id,
        "eventKey": message.event_key,
        "category": "stays",
        "audience": message.audience,
    });
    let mut sent = 0;
    for (token, platform) in tokens {
        match firebase::send_push(config, &token, message.title, message.body, &data, message.deep_link).await {
            Ok(response) => {
                record_attempt(pool, message.notification_id, &platform, &token, None, Some(response)).await?;
                sent += 1;
            }
            Err(error) => {
                let text = error.to_string();
                record_attempt(pool, message.notification_id, &platform, &token, Some(&text), None).await?;
                if token_is_dead(&text) {
                    deactivate(pool, &token).await?;
                }
                tracing::warn!(user_id = message.user_id, "notifications.push.delivery_failed");
            }
        }
    }
    Ok(sent)
}

pub fn validate_token(input: &PushTokenInput) -> Result<()> {
    let platform_valid = ["ANDROID", "IOS", "WEB", "DESKTOP"].contains(&input.platform.as_str());
    if input.token.trim().is_empty() || input.token.len() > 4096 || input.device_id.trim().is_empty() || !platform_valid {
        return Err(Error::bad_request(PUSH_TOKEN_INVALID));
    }
    Ok(())
}

pub async fn register(pool: &PgPool, user_id: &str, input: &PushTokenInput) -> Result<()> {
    validate_token(input)?;
    sqlx::query(
        "INSERT INTO fcm_tokens (id, user_id, device_id, token, platform, app_version, is_active, last_used_at) VALUES ($1, $2, $3, $4, $5, $6, true, now()) ON CONFLICT (token) DO UPDATE SET user_id = EXCLUDED.user_id, device_id = EXCLUDED.device_id, platform = EXCLUDED.platform, app_version = EXCLUDED.app_version, is_active = true, invalidated_at = NULL, last_used_at = now(), updated_at = now()",
    )
    .bind(new_id("fcm"))
    .bind(user_id)
    .bind(input.device_id.trim())
    .bind(input.token.trim())
    .bind(&input.platform)
    .bind(input.app_version.as_deref())
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn unregister(pool: &PgPool, user_id: &str, token: &str) -> Result<()> {
    sqlx::query("UPDATE fcm_tokens SET is_active = false, invalidated_at = now(), updated_at = now() WHERE token = $1 AND user_id = $2")
        .bind(token.trim())
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(token: &str, platform: &str) -> PushTokenInput {
        PushTokenInput { device_id: "d1".into(), token: token.into(), platform: platform.into(), app_version: None }
    }

    #[test]
    fn dead_token_errors_are_recognised() {
        assert!(token_is_dead("FCM v1 HTTP 404: UNREGISTERED"));
        assert!(token_is_dead("NotRegistered"));
        assert!(!token_is_dead("timeout"));
    }

    #[test]
    fn tokens_need_a_value_and_a_known_platform() {
        assert!(validate_token(&input("abc", "ANDROID")).is_ok());
        assert!(validate_token(&input(" ", "ANDROID")).is_err());
        assert!(validate_token(&input("abc", "SMS")).is_err());
    }
}
