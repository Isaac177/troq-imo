use kernel::error::ErrorCode;

pub const REVIEW_RATING_INVALID: ErrorCode = ErrorCode::new("REVIEW_RATING_INVALID");
pub const REVIEW_TEXT_REQUIRED: ErrorCode = ErrorCode::new("REVIEW_TEXT_REQUIRED");
pub const REVIEW_TEXT_TOO_LONG: ErrorCode = ErrorCode::new("REVIEW_TEXT_TOO_LONG");
pub const REVIEW_STATUS_INVALID: ErrorCode = ErrorCode::new("REVIEW_STATUS_INVALID");
pub const REVIEW_MODERATION_STATUS_INVALID: ErrorCode = ErrorCode::new("REVIEW_MODERATION_STATUS_INVALID");
pub const REVIEW_HIDE_REASON_REQUIRED: ErrorCode = ErrorCode::new("REVIEW_HIDE_REASON_REQUIRED");
pub const REVIEW_NOT_FOUND: ErrorCode = ErrorCode::new("REVIEW_NOT_FOUND");
pub const REVIEW_RESERVATION_NOT_FOUND: ErrorCode = ErrorCode::new("REVIEW_RESERVATION_NOT_FOUND");
pub const REVIEW_NOT_CHECKED_OUT: ErrorCode = ErrorCode::new("REVIEW_NOT_CHECKED_OUT");
pub const REVIEW_ALREADY_EXISTS: ErrorCode = ErrorCode::new("REVIEW_ALREADY_EXISTS");
pub const REVIEW_NOT_APPROVED: ErrorCode = ErrorCode::new("REVIEW_NOT_APPROVED");
