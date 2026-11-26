use kernel::error::ErrorCode;

pub const CONTACT_PHONE_INVALID: ErrorCode = ErrorCode::new("HOST_CONTACT_PHONE_INVALID");
pub const CONTACT_EMAIL_INVALID: ErrorCode = ErrorCode::new("HOST_CONTACT_EMAIL_INVALID");
pub const APPLICATION_EXISTS: ErrorCode = ErrorCode::new("HOST_APPLICATION_EXISTS");
pub const APPLICATION_STATE: ErrorCode = ErrorCode::new("HOST_APPLICATION_STATE_INVALID");
pub const KIND_INVALID: ErrorCode = ErrorCode::new("HOST_KIND_INVALID");
pub const REASON_INVALID: ErrorCode = ErrorCode::new("HOST_REASON_INVALID");
pub const DOCUMENT_KIND_INVALID: ErrorCode = ErrorCode::new("HOST_DOCUMENT_KIND_INVALID");
pub const DOCUMENT_KEY_INVALID: ErrorCode = ErrorCode::new("HOST_DOCUMENT_KEY_INVALID");
pub const DOCUMENT_REQUIRED: ErrorCode = ErrorCode::new("HOST_DOCUMENT_REQUIRED");
pub const HOST_STATUS_INVALID: ErrorCode = ErrorCode::new("HOST_STATUS_INVALID");
