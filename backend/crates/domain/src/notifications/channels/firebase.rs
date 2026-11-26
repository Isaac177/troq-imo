use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::notifications::config::NotificationsConfig;

const FIREBASE_SCOPE: &str = "https://www.googleapis.com/auth/firebase.messaging";
const DEFAULT_TOKEN_URI: &str = "https://oauth2.googleapis.com/token";
const DEFAULT_V1_PREFIX: &str = "https://fcm.googleapis.com/v1/projects";

#[derive(Debug, Deserialize)]
struct ServiceAccount {
    project_id: String,
    client_email: String,
    private_key: String,
    token_uri: Option<String>,
}

#[derive(Debug, Serialize)]
struct JwtClaims<'a> {
    iss: &'a str,
    scope: &'a str,
    aud: &'a str,
    iat: i64,
    exp: i64,
}

#[derive(Debug, Deserialize)]
struct OAuthTokenResponse {
    access_token: String,
    expires_in: i64,
}

#[derive(Clone)]
struct CachedToken {
    access_token: String,
    expires_at_epoch: i64,
}

static ACCESS_TOKEN_CACHE: Mutex<Option<CachedToken>> = Mutex::new(None);

pub async fn send_push(
    config: &NotificationsConfig,
    device_token: &str,
    title: &str,
    body: &str,
    data: &Value,
    deep_link: Option<&str>,
) -> Result<Value> {
    if !config.fcm_server_key.trim().is_empty() {
        return send_legacy(config, device_token, title, body, data, deep_link).await;
    }
    let account = load_service_account(config)?;
    let access_token = get_access_token(&account).await?;
    send_v1(config, &account, &access_token, device_token, title, body, data, deep_link).await
}

async fn send_legacy(
    config: &NotificationsConfig,
    device_token: &str,
    title: &str,
    body: &str,
    data: &Value,
    deep_link: Option<&str>,
) -> Result<Value> {
    let mut data_payload = data.as_object().cloned().unwrap_or_default();
    if let Some(link) = deep_link.filter(|value| !value.trim().is_empty()) {
        data_payload.insert("deepLink".to_string(), Value::String(link.to_string()));
    }
    let payload = json!({
        "to": device_token,
        "priority": "high",
        "notification": { "title": title, "body": body, "sound": "default" },
        "data": data_payload,
    });
    let response = reqwest::Client::new()
        .post(&config.fcm_send_endpoint)
        .header("Authorization", format!("key={}", config.fcm_server_key))
        .json(&payload)
        .send()
        .await?;
    let status = response.status();
    let body: Value = response.json().await.unwrap_or_else(|_| json!({}));
    if !status.is_success() {
        return Err(anyhow!("FCM legacy HTTP {}: {}", status, body));
    }
    if body.get("success").and_then(Value::as_i64).unwrap_or(0) <= 0 {
        return Err(anyhow!("FCM legacy delivery failed: {}", body));
    }
    Ok(body)
}

fn v1_endpoint(config: &NotificationsConfig, account: &ServiceAccount) -> String {
    if config.fcm_send_endpoint.trim().starts_with("https://fcm.googleapis.com/v1/") {
        config.fcm_send_endpoint.clone()
    } else {
        format!("{}/{}/messages:send", DEFAULT_V1_PREFIX, account.project_id)
    }
}

async fn send_v1(
    config: &NotificationsConfig,
    account: &ServiceAccount,
    access_token: &str,
    device_token: &str,
    title: &str,
    body: &str,
    data: &Value,
    deep_link: Option<&str>,
) -> Result<Value> {
    let mut data_payload = stringify_data_map(data);
    if let Some(link) = deep_link.filter(|value| !value.trim().is_empty()) {
        data_payload.insert("deepLink".to_string(), link.to_string());
    }
    let payload = json!({
        "message": {
            "token": device_token,
            "notification": { "title": title, "body": body },
            "android": { "priority": "HIGH", "notification": { "sound": "default" } },
            "apns": { "payload": { "aps": { "sound": "default" } } },
            "data": data_payload,
        }
    });
    let response = reqwest::Client::new()
        .post(v1_endpoint(config, account))
        .bearer_auth(access_token)
        .json(&payload)
        .send()
        .await?;
    let status = response.status();
    let body: Value = response.json().await.unwrap_or_else(|_| json!({}));
    if !status.is_success() {
        return Err(anyhow!("FCM v1 HTTP {}: {}", status, body));
    }
    Ok(body)
}

pub fn stringify_data_map(data: &Value) -> HashMap<String, String> {
    let Some(object) = data.as_object() else {
        return HashMap::new();
    };
    object
        .iter()
        .map(|(key, value)| {
            let text = match value {
                Value::Null => String::new(),
                Value::String(text) => text.clone(),
                Value::Bool(flag) => flag.to_string(),
                Value::Number(number) => number.to_string(),
                other => serde_json::to_string(other).unwrap_or_default(),
            };
            (key.clone(), text)
        })
        .collect()
}

async fn get_access_token(account: &ServiceAccount) -> Result<String> {
    let now = Utc::now().timestamp();
    if let Some(cached) = ACCESS_TOKEN_CACHE.lock().ok().and_then(|guard| guard.clone()) {
        if cached.expires_at_epoch > now + 60 {
            return Ok(cached.access_token);
        }
    }
    let token_uri = account
        .token_uri
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(DEFAULT_TOKEN_URI);
    let claims = JwtClaims { iss: &account.client_email, scope: FIREBASE_SCOPE, aud: token_uri, iat: now, exp: now + 3600 };
    let jwt = encode(
        &Header::new(Algorithm::RS256),
        &claims,
        &EncodingKey::from_rsa_pem(account.private_key.as_bytes()).context("invalid service account private_key PEM")?,
    )?;
    let response = reqwest::Client::new()
        .post(token_uri)
        .form(&[("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"), ("assertion", jwt.as_str())])
        .send()
        .await?;
    let status = response.status();
    let text = response.text().await.unwrap_or_default();
    if !status.is_success() {
        return Err(anyhow!("google oauth HTTP {}: {}", status, text));
    }
    let parsed: OAuthTokenResponse = serde_json::from_str(&text).context("invalid oauth token response json")?;
    if let Ok(mut guard) = ACCESS_TOKEN_CACHE.lock() {
        *guard = Some(CachedToken {
            access_token: parsed.access_token.clone(),
            expires_at_epoch: now + parsed.expires_in.max(300),
        });
    }
    Ok(parsed.access_token)
}

fn load_service_account(config: &NotificationsConfig) -> Result<ServiceAccount> {
    let path = [&config.google_application_credentials, &config.firebase_service_account_path]
        .into_iter()
        .map(|value| value.trim())
        .find(|value| !value.is_empty())
        .ok_or_else(|| anyhow!("FCM auth is not configured: set FCM_SERVER_KEY or GOOGLE_APPLICATION_CREDENTIALS/FIREBASE_SERVICE_ACCOUNT_PATH"))?;
    let resolved = PathBuf::from(path);
    let content = std::fs::read_to_string(&resolved)
        .with_context(|| format!("failed reading service account file {}", resolved.display()))?;
    let account: ServiceAccount = serde_json::from_str(&content).context("invalid service account json")?;
    if account.project_id.trim().is_empty() || account.client_email.trim().is_empty() || account.private_key.trim().is_empty() {
        return Err(anyhow!("service account json missing project_id/client_email/private_key"));
    }
    Ok(account)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_values_are_flattened_to_strings() {
        let map = stringify_data_map(&json!({"a": 1, "b": true, "c": null, "d": "x", "e": {"k": 1}}));
        assert_eq!(map["a"], "1");
        assert_eq!(map["b"], "true");
        assert_eq!(map["c"], "");
        assert_eq!(map["d"], "x");
        assert_eq!(map["e"], "{\"k\":1}");
    }

    #[test]
    fn v1_endpoint_defaults_to_the_project_path() {
        let config = NotificationsConfig::from_env();
        let account = ServiceAccount { project_id: "p1".into(), client_email: String::new(), private_key: String::new(), token_uri: None };
        assert_eq!(v1_endpoint(&config, &account), "https://fcm.googleapis.com/v1/projects/p1/messages:send");
    }
}
