use std::env;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct NotificationsConfig {
    pub product_name: String,
    pub email_from: String,
    pub shipmail_api_key: String,
    pub shipmail_api_url: String,
    pub whatsapp_access_token: String,
    pub fcm_server_key: String,
    pub fcm_send_endpoint: String,
    pub google_application_credentials: String,
    pub firebase_service_account_path: String,
    pub guest_base_url: String,
    pub host_base_url: String,
    pub admin_base_url: String,
    pub chrome_path: Option<String>,
    pub recycle_after: u32,
    pub render_timeout_secs: u64,
    pub assets_dir: PathBuf,
}

fn text(name: &str, default: &str) -> String {
    env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| default.to_string())
}

fn number<T: std::str::FromStr>(name: &str, default: T) -> T {
    env::var(name).ok().and_then(|value| value.trim().parse().ok()).unwrap_or(default)
}

impl NotificationsConfig {
    pub fn from_env() -> Self {
        let chrome = text("DOCUMENT_CHROME_PATH", "");
        Self {
            product_name: text("PRODUCT_NAME", "Mungini"),
            email_from: text("EMAIL_FROM", "Mungini <no-reply@mungini.local>"),
            shipmail_api_key: text("SHIPMAIL_API_KEY", ""),
            shipmail_api_url: text("SHIPMAIL_API_URL", "https://shipmail.to/api/v1/messages"),
            whatsapp_access_token: text("WHATSAPP_ACCESS_TOKEN", ""),
            fcm_server_key: text("FCM_SERVER_KEY", ""),
            fcm_send_endpoint: text("FCM_SEND_ENDPOINT", "https://fcm.googleapis.com/fcm/send"),
            google_application_credentials: text("GOOGLE_APPLICATION_CREDENTIALS", ""),
            firebase_service_account_path: text("FIREBASE_SERVICE_ACCOUNT_PATH", ""),
            guest_base_url: text("GUEST_BASE_URL", "http://localhost:3000"),
            host_base_url: text("HOST_BASE_URL", "http://localhost:5174"),
            admin_base_url: text("ADMIN_BASE_URL", "http://localhost:5175"),
            chrome_path: if chrome.is_empty() { None } else { Some(chrome) },
            recycle_after: number("DOCUMENT_RECYCLE_AFTER", 200),
            render_timeout_secs: number("DOCUMENT_RENDER_TIMEOUT_SECS", 30),
            assets_dir: assets_dir(),
        }
    }

    pub fn base_url(&self, audience: &str) -> &str {
        match audience {
            "admin" => &self.admin_base_url,
            "host" => &self.host_base_url,
            _ => &self.guest_base_url,
        }
    }
}

fn assets_dir() -> PathBuf {
    match env::var("APP_BASE_DIR") {
        Ok(dir) if !dir.trim().is_empty() => PathBuf::from(dir),
        _ => PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/notifications"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_url_follows_the_audience() {
        let config = NotificationsConfig::from_env();
        assert_eq!(config.base_url("admin"), config.admin_base_url);
        assert_eq!(config.base_url("host"), config.host_base_url);
        assert_eq!(config.base_url("guest"), config.guest_base_url);
    }

    #[test]
    fn assets_dir_points_at_the_module_templates() {
        assert!(NotificationsConfig::from_env().assets_dir.join("templates").exists());
    }
}
