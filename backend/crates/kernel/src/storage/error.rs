use std::fmt;

use crate::error::codes::{STORAGE_KEY_INVALID, STORAGE_OBJECT_NOT_FOUND};
use crate::error::Error;

pub type StorageResult<T> = Result<T, StorageError>;

#[derive(Debug)]
pub enum StorageError {
    MissingEnv(&'static str),
    InvalidEnv(&'static str),
    Timeout,
    NotFound,
    InvalidKey,
    Request(String),
}

impl fmt::Display for StorageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingEnv(name) => write!(formatter, "missing storage env var {name}"),
            Self::InvalidEnv(name) => write!(formatter, "invalid storage env var {name}"),
            Self::Timeout => formatter.write_str("storage request timed out"),
            Self::NotFound => formatter.write_str("storage object not found"),
            Self::InvalidKey => formatter.write_str("invalid storage key"),
            Self::Request(detail) => write!(formatter, "storage request failed: {detail}"),
        }
    }
}

impl std::error::Error for StorageError {}

impl From<StorageError> for Error {
    fn from(error: StorageError) -> Self {
        match error {
            StorageError::NotFound => Error::not_found(STORAGE_OBJECT_NOT_FOUND),
            StorageError::InvalidKey => Error::bad_request(STORAGE_KEY_INVALID),
            other => Error::internal(other.to_string()),
        }
    }
}
