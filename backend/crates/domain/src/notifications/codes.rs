use kernel::ErrorCode;

pub const NOTIFICATION_NOT_FOUND: ErrorCode = ErrorCode::new("NOTIFICATION_NOT_FOUND");
pub const DOCUMENT_NOT_FOUND: ErrorCode = ErrorCode::new("DOCUMENT_NOT_FOUND");
pub const DOCUMENT_KIND_INVALID: ErrorCode = ErrorCode::new("DOCUMENT_KIND_INVALID");
pub const PREFERENCE_KEY_INVALID: ErrorCode = ErrorCode::new("PREFERENCE_KEY_INVALID");
pub const PUSH_TOKEN_INVALID: ErrorCode = ErrorCode::new("PUSH_TOKEN_INVALID");
pub const LOCALE_INVALID: ErrorCode = ErrorCode::new("LOCALE_INVALID");
