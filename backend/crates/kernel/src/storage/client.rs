use aws_sdk_s3::config::{
    BehaviorVersion, Credentials, Region, RequestChecksumCalculation, ResponseChecksumValidation,
};
use aws_sdk_s3::error::DisplayErrorContext;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use bytes::Bytes;
use tokio::time::{sleep, timeout, Duration};

use super::config::StorageConfig;
use super::error::{StorageError, StorageResult};
use super::keys::validate_key;
use super::stored::public_url;
use super::visibility::Visibility;

#[derive(Clone)]
pub struct Storage {
    pub(crate) client: Client,
    pub(crate) signer: Client,
    pub(crate) config: StorageConfig,
}

#[derive(Clone, Debug)]
pub struct ObjectMeta {
    pub size: u64,
    pub content_type: Option<String>,
}

impl Storage {
    pub fn new(config: StorageConfig) -> Self {
        let client = build_client(&config, &config.endpoint);
        let signer = match &config.signing_endpoint {
            Some(endpoint) => build_client(&config, endpoint),
            None => client.clone(),
        };
        Self {
            client,
            signer,
            config,
        }
    }

    pub fn public_url(&self, key: &str) -> String {
        public_url(&self.config.public_base_url, key)
    }

    pub fn public_base_url(&self) -> &str {
        &self.config.public_base_url
    }

    pub(crate) fn bucket(&self, visibility: Visibility) -> &str {
        match visibility {
            Visibility::Public => &self.config.bucket_public,
            Visibility::Private => &self.config.bucket_private,
        }
    }

    pub async fn put(
        &self,
        visibility: Visibility,
        key: &str,
        body: impl Into<Bytes>,
        content_type: &str,
    ) -> StorageResult<()> {
        validate_key(key)?;
        let body: Bytes = body.into();
        let retries = self.config.upload_retries;
        let mut last = StorageError::Timeout;
        for attempt in 1..=retries {
            let request = self
                .client
                .put_object()
                .bucket(self.bucket(visibility))
                .key(key)
                .content_type(content_type)
                .cache_control(visibility.cache_control())
                .body(ByteStream::from(body.clone()))
                .send();
            match timeout(self.config.upload_timeout, request).await {
                Ok(Ok(_)) => return Ok(()),
                Ok(Err(error)) => {
                    last = StorageError::Request(DisplayErrorContext(&error).to_string())
                }
                Err(_) => last = StorageError::Timeout,
            }
            if attempt < retries {
                sleep(Duration::from_millis(250 * u64::from(attempt))).await;
            }
        }
        Err(last)
    }

    pub async fn get_bytes(&self, visibility: Visibility, key: &str) -> StorageResult<Bytes> {
        validate_key(key)?;
        let output = self
            .client
            .get_object()
            .bucket(self.bucket(visibility))
            .key(key)
            .send()
            .await
            .map_err(|error| {
                if error.as_service_error().is_some_and(|e| e.is_no_such_key()) {
                    StorageError::NotFound
                } else {
                    StorageError::Request(DisplayErrorContext(&error).to_string())
                }
            })?;
        let data = output
            .body
            .collect()
            .await
            .map_err(|error| StorageError::Request(error.to_string()))?;
        Ok(data.into_bytes())
    }

    pub async fn head(
        &self,
        visibility: Visibility,
        key: &str,
    ) -> StorageResult<Option<ObjectMeta>> {
        validate_key(key)?;
        match self
            .client
            .head_object()
            .bucket(self.bucket(visibility))
            .key(key)
            .send()
            .await
        {
            Ok(output) => Ok(Some(ObjectMeta {
                size: output.content_length().unwrap_or(0).max(0) as u64,
                content_type: output.content_type().map(str::to_string),
            })),
            Err(error) if error.as_service_error().is_some_and(|e| e.is_not_found()) => Ok(None),
            Err(error) => Err(StorageError::Request(DisplayErrorContext(&error).to_string())),
        }
    }

    pub async fn delete(&self, visibility: Visibility, key: &str) -> StorageResult<()> {
        validate_key(key)?;
        self.client
            .delete_object()
            .bucket(self.bucket(visibility))
            .key(key)
            .send()
            .await
            .map_err(|error| StorageError::Request(DisplayErrorContext(&error).to_string()))?;
        Ok(())
    }
}

fn build_client(config: &StorageConfig, endpoint: &str) -> Client {
    let credentials = Credentials::new(
        config.access_key_id.clone(),
        config.secret_access_key.clone(),
        None,
        None,
        "mungini-storage",
    );
    let settings = aws_sdk_s3::config::Builder::new()
        .behavior_version(BehaviorVersion::latest())
        .endpoint_url(endpoint)
        .region(Region::new(config.region.clone()))
        .credentials_provider(credentials)
        .force_path_style(config.force_path_style)
        .request_checksum_calculation(RequestChecksumCalculation::WhenRequired)
        .response_checksum_validation(ResponseChecksumValidation::WhenRequired)
        .build();
    Client::from_conf(settings)
}
