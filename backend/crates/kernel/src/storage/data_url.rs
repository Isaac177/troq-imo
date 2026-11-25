use base64::Engine;
use bytes::Bytes;

use super::error::{StorageError, StorageResult};

#[derive(Clone, Debug)]
pub struct DataUrl {
    pub mime: String,
    pub bytes: Bytes,
}

pub fn is_data_url(value: &str) -> bool {
    value.trim_start().starts_with("data:") && value.contains(";base64,")
}

pub fn parse_data_url(value: &str) -> StorageResult<DataUrl> {
    let rest = value
        .trim()
        .strip_prefix("data:")
        .ok_or(StorageError::InvalidKey)?;
    let (mime, payload) = rest.split_once(";base64,").ok_or(StorageError::InvalidKey)?;
    let mime = mime.trim();
    if mime.is_empty() {
        return Err(StorageError::InvalidKey);
    }
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(payload.trim())
        .map_err(|_| StorageError::InvalidKey)?;
    Ok(DataUrl {
        mime: mime.to_string(),
        bytes: Bytes::from(bytes),
    })
}

pub fn extension_for_mime(mime: &str) -> &'static str {
    match mime {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/webp" => "webp",
        "image/gif" => "gif",
        "image/svg+xml" => "svg",
        "application/pdf" => "pdf",
        _ => "bin",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_base64_data_url() {
        let parsed = parse_data_url("data:image/png;base64,aGVsbG8=").unwrap();
        assert_eq!(parsed.mime, "image/png");
        assert_eq!(parsed.bytes.as_ref(), b"hello");
        assert!(parse_data_url("data:;base64,aGk=").is_err());
        assert!(parse_data_url("image/png;base64,aGk=").is_err());
    }
}
