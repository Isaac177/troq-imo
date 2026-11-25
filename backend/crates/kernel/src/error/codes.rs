use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct ErrorCode(&'static str);

impl ErrorCode {
    pub const fn new(code: &'static str) -> Self {
        Self(code)
    }

    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

pub const PAGE_INVALID: ErrorCode = ErrorCode::new("PAGE_INVALID");
pub const PAGE_SIZE_INVALID: ErrorCode = ErrorCode::new("PAGE_SIZE_INVALID");
pub const CITY_UNSUPPORTED: ErrorCode = ErrorCode::new("CITY_UNSUPPORTED");
pub const CITY_MISMATCH: ErrorCode = ErrorCode::new("CITY_MISMATCH");
pub const CITY_VIEW_ONLY: ErrorCode = ErrorCode::new("CITY_VIEW_ONLY");
pub const AUTH_REQUIRED: ErrorCode = ErrorCode::new("AUTH_REQUIRED");
pub const TOKEN_INVALID: ErrorCode = ErrorCode::new("TOKEN_INVALID");
pub const SESSION_REVOKED: ErrorCode = ErrorCode::new("SESSION_REVOKED");
pub const PERMISSION_MISSING: ErrorCode = ErrorCode::new("PERMISSION_MISSING");
pub const NOT_FOUND: ErrorCode = ErrorCode::new("NOT_FOUND");
pub const FILE_NAME_INVALID: ErrorCode = ErrorCode::new("FILE_NAME_INVALID");
pub const AMOUNT_INVALID: ErrorCode = ErrorCode::new("AMOUNT_INVALID");
pub const IDEMPOTENCY_KEY_INVALID: ErrorCode = ErrorCode::new("IDEMPOTENCY_KEY_INVALID");
pub const STORAGE_KEY_INVALID: ErrorCode = ErrorCode::new("STORAGE_KEY_INVALID");
pub const STORAGE_OBJECT_NOT_FOUND: ErrorCode = ErrorCode::new("STORAGE_OBJECT_NOT_FOUND");
pub const MALFORMED_REQUEST: ErrorCode = ErrorCode::new("MALFORMED_REQUEST");
pub const INTERNAL_ERROR: ErrorCode = ErrorCode::new("INTERNAL_ERROR");
