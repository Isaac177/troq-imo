use std::time::Duration;

use crate::config::{optional, required};

use super::error::{StorageError, StorageResult};

#[derive(Clone, Debug)]
pub struct StorageConfig {
    pub endpoint: String,
    pub signing_endpoint: Option<String>,
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub bucket_public: String,
    pub bucket_private: String,
    pub public_base_url: String,
    pub force_path_style: bool,
    pub signed_url_ttl: Duration,
    pub upload_timeout: Duration,
    pub upload_retries: u32,
}

impl StorageConfig {
    pub fn from_env() -> StorageResult<Self> {
        Ok(Self {
            endpoint: need("STORAGE_S3_ENDPOINT")?,
            signing_endpoint: optional("STORAGE_SIGNING_ENDPOINT"),
            region: optional("STORAGE_S3_REGION").unwrap_or_else(|| "auto".to_string()),
            access_key_id: need("STORAGE_S3_ACCESS_KEY_ID")?,
            secret_access_key: need("STORAGE_S3_SECRET_ACCESS_KEY")?,
            bucket_public: need("STORAGE_BUCKET_PUBLIC")?,
            bucket_private: need("STORAGE_BUCKET_PRIVATE")?,
            public_base_url: need("STORAGE_PUBLIC_BASE_URL")?
                .trim_end_matches('/')
                .to_string(),
            force_path_style: parsed("STORAGE_S3_FORCE_PATH_STYLE", true)?,
            signed_url_ttl: Duration::from_secs(
                parsed::<u64>("STORAGE_SIGNED_URL_TTL_SECONDS", 300)?.clamp(30, 3600),
            ),
            upload_timeout: Duration::from_secs(
                parsed::<u64>("STORAGE_S3_UPLOAD_TIMEOUT_SECONDS", 20)?.clamp(5, 60),
            ),
            upload_retries: parsed::<u32>("STORAGE_S3_UPLOAD_RETRIES", 3)?.clamp(1, 5),
        })
    }
}

fn need(name: &'static str) -> StorageResult<String> {
    required(name).map_err(|_| StorageError::MissingEnv(name))
}

fn parsed<T: std::str::FromStr>(name: &'static str, default: T) -> StorageResult<T> {
    match optional(name) {
        Some(value) => value.parse::<T>().map_err(|_| StorageError::InvalidEnv(name)),
        None => Ok(default),
    }
}
