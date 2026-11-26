use kernel::error::ErrorCode;

pub const PERMISSION_INVALID: ErrorCode = ErrorCode::new("IDENTITY_PERMISSION_INVALID");
pub const INVALID_CREDENTIALS: ErrorCode = ErrorCode::new("INVALID_CREDENTIALS");
pub const ACCOUNT_BLOCKED: ErrorCode = ErrorCode::new("ACCOUNT_BLOCKED");
pub const OTP_INVALID: ErrorCode = ErrorCode::new("REGISTRATION_OTP_INVALID");
pub const OTP_RATE_LIMITED: ErrorCode = ErrorCode::new("REGISTRATION_OTP_RATE_LIMITED");
pub const CONTACT_INVALID: ErrorCode = ErrorCode::new("REGISTRATION_CONTACT_INVALID");
pub const CONTACT_EXISTS: ErrorCode = ErrorCode::new("REGISTRATION_CONTACT_EXISTS");
pub const OWNER_PROTECTED: ErrorCode = ErrorCode::new("HOST_OWNER_PROTECTED");
pub const INVITEE_CITY_MISMATCH: ErrorCode = ErrorCode::new("INVITEE_CITY_MISMATCH");
