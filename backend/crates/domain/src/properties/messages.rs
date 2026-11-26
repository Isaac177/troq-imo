use kernel::error::ErrorCode;

pub const PROPERTY_NOT_FOUND: ErrorCode = ErrorCode::new("PROPERTY_NOT_FOUND");
pub const PROPERTY_STATUS_UNKNOWN: ErrorCode = ErrorCode::new("PROPERTY_STATUS_UNKNOWN");
pub const HOST_NOT_FOUND: ErrorCode = ErrorCode::new("HOST_NOT_FOUND");
pub const HOST_NOT_ACTIVE: ErrorCode = ErrorCode::new("HOST_NOT_ACTIVE");
pub const HOST_NOT_MEMBER: ErrorCode = ErrorCode::new("HOST_NOT_MEMBER");
pub const REQUEST_IN_PROGRESS: ErrorCode = ErrorCode::new("REQUEST_IN_PROGRESS");

pub const FIELD_REQUIRED: ErrorCode = ErrorCode::new("FIELD_REQUIRED");
pub const FIELD_TOO_LONG: ErrorCode = ErrorCode::new("FIELD_TOO_LONG");
pub const TIMEZONE_UNSUPPORTED: ErrorCode = ErrorCode::new("TIMEZONE_UNSUPPORTED");
pub const COORDINATES_OUT_OF_RANGE: ErrorCode = ErrorCode::new("COORDINATES_OUT_OF_RANGE");
pub const COORDINATES_INCOMPLETE: ErrorCode = ErrorCode::new("COORDINATES_INCOMPLETE");
pub const CLOCK_INVALID: ErrorCode = ErrorCode::new("CLOCK_INVALID");
pub const CHECK_IN_WINDOW_INVALID: ErrorCode = ErrorCode::new("CHECK_IN_WINDOW_INVALID");
pub const CHOICE_INVALID: ErrorCode = ErrorCode::new("CHOICE_INVALID");
pub const PHONE_INVALID: ErrorCode = ErrorCode::new("PHONE_INVALID");
pub const EMAIL_INVALID: ErrorCode = ErrorCode::new("EMAIL_INVALID");
pub const AMENITY_MALFORMED: ErrorCode = ErrorCode::new("AMENITY_MALFORMED");

pub const MEDIA_NOT_FOUND: ErrorCode = ErrorCode::new("MEDIA_NOT_FOUND");
pub const MEDIA_CLIENT_KEY_INVALID: ErrorCode = ErrorCode::new("MEDIA_CLIENT_KEY_INVALID");
pub const MEDIA_CONTENT_INVALID: ErrorCode = ErrorCode::new("MEDIA_CONTENT_INVALID");
pub const MEDIA_MIME_TYPE_INVALID: ErrorCode = ErrorCode::new("MEDIA_MIME_TYPE_INVALID");
pub const MEDIA_ALT_TEXT_TOO_LONG: ErrorCode = ErrorCode::new("MEDIA_ALT_TEXT_TOO_LONG");
pub const MEDIA_COVER_CANNOT_BE_CLEARED: ErrorCode = ErrorCode::new("MEDIA_COVER_CANNOT_BE_CLEARED");
pub const MEDIA_COVER_NOT_READY: ErrorCode = ErrorCode::new("MEDIA_COVER_NOT_READY");
pub const MEDIA_VERSION_CONFLICT: ErrorCode = ErrorCode::new("MEDIA_VERSION_CONFLICT");

pub const CATEGORY_NOT_FOUND: ErrorCode = ErrorCode::new("CATEGORY_NOT_FOUND");
pub const CATEGORY_UNKNOWN: ErrorCode = ErrorCode::new("CATEGORY_UNKNOWN");
pub const CATEGORY_NAME_INVALID: ErrorCode = ErrorCode::new("CATEGORY_NAME_INVALID");
pub const CATEGORY_CODE_REQUIRED: ErrorCode = ErrorCode::new("CATEGORY_CODE_REQUIRED");
pub const CATEGORY_CODE_EXHAUSTED: ErrorCode = ErrorCode::new("CATEGORY_CODE_EXHAUSTED");
pub const CATEGORY_ICON_INVALID: ErrorCode = ErrorCode::new("CATEGORY_ICON_INVALID");

pub const TEMPLATE_NOT_FOUND: ErrorCode = ErrorCode::new("TEMPLATE_NOT_FOUND");
pub const TEMPLATE_AMOUNT_INVALID: ErrorCode = ErrorCode::new("TEMPLATE_AMOUNT_INVALID");
pub const TEMPLATE_STATUS_INVALID: ErrorCode = ErrorCode::new("TEMPLATE_STATUS_INVALID");
pub const TEMPLATE_VERSION_CONFLICT: ErrorCode = ErrorCode::new("TEMPLATE_VERSION_CONFLICT");
pub const TEMPLATE_ID_REQUIRED: ErrorCode = ErrorCode::new("TEMPLATE_ID_REQUIRED");

pub const LISTING_VERSION_CONFLICT: ErrorCode = ErrorCode::new("LISTING_VERSION_CONFLICT");
pub const LISTING_STATUS_INVALID: ErrorCode = ErrorCode::new("LISTING_STATUS_INVALID");
pub const LISTING_ARCHIVED: ErrorCode = ErrorCode::new("LISTING_ARCHIVED");
pub const LISTING_NOT_READY: ErrorCode = ErrorCode::new("LISTING_NOT_READY");
pub const LISTING_ACTION_UNKNOWN: ErrorCode = ErrorCode::new("LISTING_ACTION_UNKNOWN");
pub const LISTING_UPCOMING_STAYS: ErrorCode = ErrorCode::new("LISTING_UPCOMING_STAYS");
pub const REVISION_NOT_FOUND: ErrorCode = ErrorCode::new("REVISION_NOT_FOUND");
pub const REVISION_ALREADY_OPEN: ErrorCode = ErrorCode::new("REVISION_ALREADY_OPEN");
pub const REVISION_SNAPSHOT_UNREADABLE: ErrorCode = ErrorCode::new("REVISION_SNAPSHOT_UNREADABLE");
pub const DECISION_UNKNOWN: ErrorCode = ErrorCode::new("DECISION_UNKNOWN");
pub const REASON_CODES_REQUIRED: ErrorCode = ErrorCode::new("REASON_CODES_REQUIRED");
pub const REASON_CODES_NOT_ALLOWED: ErrorCode = ErrorCode::new("REASON_CODES_NOT_ALLOWED");
pub const REASON_CODE_UNKNOWN: ErrorCode = ErrorCode::new("REASON_CODE_UNKNOWN");
pub const REASON_CODE_REQUIRED: ErrorCode = ErrorCode::new("REASON_CODE_REQUIRED");
pub const QUEUE_STATUS_INVALID: ErrorCode = ErrorCode::new("QUEUE_STATUS_INVALID");
