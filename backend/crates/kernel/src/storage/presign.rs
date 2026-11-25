use aws_sdk_s3::error::DisplayErrorContext;
use aws_sdk_s3::presigning::PresigningConfig;
use tokio::time::Duration;

use super::client::Storage;
use super::error::{StorageError, StorageResult};
use super::keys::validate_key;
use super::visibility::Visibility;

impl Storage {
    pub async fn presign_get(
        &self,
        visibility: Visibility,
        key: &str,
        ttl: Option<Duration>,
    ) -> StorageResult<String> {
        validate_key(key)?;
        let request = self
            .signer
            .get_object()
            .bucket(self.bucket(visibility))
            .key(key)
            .presigned(self.presign_config(ttl)?)
            .await
            .map_err(|error| StorageError::Request(DisplayErrorContext(&error).to_string()))?;
        Ok(request.uri().to_string())
    }

    pub async fn presign_put(
        &self,
        visibility: Visibility,
        key: &str,
        content_type: &str,
        size: u64,
        ttl: Option<Duration>,
    ) -> StorageResult<String> {
        validate_key(key)?;
        let request = self
            .signer
            .put_object()
            .bucket(self.bucket(visibility))
            .key(key)
            .content_type(content_type)
            .content_length(size as i64)
            .cache_control(visibility.cache_control())
            .presigned(self.presign_config(ttl)?)
            .await
            .map_err(|error| StorageError::Request(DisplayErrorContext(&error).to_string()))?;
        Ok(request.uri().to_string())
    }

    fn presign_config(&self, ttl: Option<Duration>) -> StorageResult<PresigningConfig> {
        PresigningConfig::expires_in(ttl.unwrap_or(self.config.signed_url_ttl))
            .map_err(|error| StorageError::Request(error.to_string()))
    }
}
