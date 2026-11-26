use kernel::error::ErrorCode;

pub const PROPERTY_NOT_FOUND: ErrorCode = ErrorCode::new("PROPERTY_NOT_FOUND");
pub const UNIT_TYPE_NOT_FOUND: ErrorCode = ErrorCode::new("UNIT_TYPE_NOT_FOUND");
pub const DATE_RANGE_INVALID: ErrorCode = ErrorCode::new("DATE_RANGE_INVALID");
pub const DATE_RANGE_TOO_LONG: ErrorCode = ErrorCode::new("DATE_RANGE_TOO_LONG");
pub const IDEMPOTENCY_IN_PROGRESS: ErrorCode = ErrorCode::new("IDEMPOTENCY_IN_PROGRESS");
pub const INVENTORY_INCONSISTENT: ErrorCode = ErrorCode::new("INVENTORY_INCONSISTENT");
pub const INVENTORY_VERSION_CONFLICT: ErrorCode = ErrorCode::new("INVENTORY_VERSION_CONFLICT");

pub const BLOCK_NOT_FOUND: ErrorCode = ErrorCode::new("BLOCK_NOT_FOUND");
pub const BLOCK_REASON_UNKNOWN: ErrorCode = ErrorCode::new("BLOCK_REASON_UNKNOWN");
pub const BLOCK_PLATFORM_REASON_RESERVED: ErrorCode = ErrorCode::new("BLOCK_PLATFORM_REASON_RESERVED");
pub const BLOCK_ALREADY_REMOVED: ErrorCode = ErrorCode::new("BLOCK_ALREADY_REMOVED");
pub const BLOCK_PLATFORM_ONLY: ErrorCode = ErrorCode::new("BLOCK_PLATFORM_ONLY");
pub const BLOCK_VERSION_CONFLICT: ErrorCode = ErrorCode::new("BLOCK_VERSION_CONFLICT");
pub const NIGHTS_UNAVAILABLE: ErrorCode = ErrorCode::new("NIGHTS_UNAVAILABLE");

pub const ITEM_NOT_FOUND: ErrorCode = ErrorCode::new("RESERVATION_ITEM_NOT_FOUND");
pub const ASSIGNMENT_NOT_FOUND: ErrorCode = ErrorCode::new("ASSIGNMENT_NOT_FOUND");
pub const ASSIGNMENT_STATE_INVALID: ErrorCode = ErrorCode::new("ASSIGNMENT_STATE_INVALID");
pub const ASSIGNMENT_INACTIVE: ErrorCode = ErrorCode::new("ASSIGNMENT_INACTIVE");
pub const ASSIGNMENT_CHANGED: ErrorCode = ErrorCode::new("ASSIGNMENT_CHANGED");
pub const ASSIGNMENT_RESERVATION_CHANGED: ErrorCode = ErrorCode::new("ASSIGNMENT_RESERVATION_CHANGED");
pub const ASSIGNMENT_ALREADY_HAS_UNIT: ErrorCode = ErrorCode::new("ASSIGNMENT_ALREADY_HAS_UNIT");
pub const ASSIGNMENT_MOVE_DATE_INVALID: ErrorCode = ErrorCode::new("ASSIGNMENT_MOVE_DATE_INVALID");
pub const ASSIGNMENT_SAME_UNIT: ErrorCode = ErrorCode::new("ASSIGNMENT_SAME_UNIT");
pub const UNIT_INCOMPATIBLE: ErrorCode = ErrorCode::new("UNIT_INCOMPATIBLE");
pub const UNIT_UNAVAILABLE: ErrorCode = ErrorCode::new("UNIT_UNAVAILABLE");
pub const UNIT_OVERLAP: ErrorCode = ErrorCode::new("UNIT_OVERLAP");
pub const UNIT_TURNOVER_BUFFER: ErrorCode = ErrorCode::new("UNIT_TURNOVER_BUFFER");

pub const RESTRICTION_KIND_UNKNOWN: ErrorCode = ErrorCode::new("RESTRICTION_KIND_UNKNOWN");
pub const RESTRICTION_BASIS_INVALID: ErrorCode = ErrorCode::new("RESTRICTION_BASIS_INVALID");
pub const RESTRICTION_VALUE_INVALID: ErrorCode = ErrorCode::new("RESTRICTION_VALUE_INVALID");
pub const RESTRICTION_VALUE_NOT_ALLOWED: ErrorCode = ErrorCode::new("RESTRICTION_VALUE_NOT_ALLOWED");
pub const RESTRICTION_WINDOW_INVALID: ErrorCode = ErrorCode::new("RESTRICTION_WINDOW_INVALID");
pub const RESTRICTION_WINDOW_TOO_LONG: ErrorCode = ErrorCode::new("RESTRICTION_WINDOW_TOO_LONG");
pub const RESTRICTION_NOT_FOUND: ErrorCode = ErrorCode::new("RESTRICTION_NOT_FOUND");
pub const RESTRICTION_ID_REQUIRED: ErrorCode = ErrorCode::new("RESTRICTION_ID_REQUIRED");
pub const RESTRICTION_KIND_REQUIRED: ErrorCode = ErrorCode::new("RESTRICTION_KIND_REQUIRED");
pub const BULK_SIZE_INVALID: ErrorCode = ErrorCode::new("RESTRICTION_BULK_SIZE_INVALID");
pub const BULK_ACTION_UNKNOWN: ErrorCode = ErrorCode::new("RESTRICTION_BULK_ACTION_UNKNOWN");

pub const AVAILABILITY_RANGE_INVALID: ErrorCode = ErrorCode::new("AVAILABILITY_RANGE_INVALID");
pub const FEED_NOT_FOUND: ErrorCode = ErrorCode::new("FEED_NOT_FOUND");
pub const POOL_QUANTITY_INVALID: ErrorCode = ErrorCode::new("POOL_QUANTITY_INVALID");
pub const POOL_NOT_FOUND: ErrorCode = ErrorCode::new("POOL_NOT_FOUND");

pub const UNIT_TYPE_NAME_INVALID: ErrorCode = ErrorCode::new("UNIT_TYPE_NAME_INVALID");
pub const UNIT_TYPE_CAPACITY_INVALID: ErrorCode = ErrorCode::new("UNIT_TYPE_CAPACITY_INVALID");
pub const UNIT_TYPE_QUANTITY_INVALID: ErrorCode = ErrorCode::new("UNIT_TYPE_QUANTITY_INVALID");
pub const UNIT_TYPE_ACCESSIBILITY_INVALID: ErrorCode = ErrorCode::new("UNIT_TYPE_ACCESSIBILITY_INVALID");
pub const UNIT_TYPE_MODEL_INVALID: ErrorCode = ErrorCode::new("UNIT_TYPE_MODEL_INVALID");
pub const UNIT_TYPE_CATEGORY_UNKNOWN: ErrorCode = ErrorCode::new("UNIT_TYPE_CATEGORY_UNKNOWN");
pub const UNIT_TYPE_TEMPLATE_UNKNOWN: ErrorCode = ErrorCode::new("UNIT_TYPE_TEMPLATE_UNKNOWN");
pub const UNIT_TYPE_VERSION_CONFLICT: ErrorCode = ErrorCode::new("UNIT_TYPE_VERSION_CONFLICT");
pub const UNIT_TYPE_QUANTITY_LOCKED: ErrorCode = ErrorCode::new("UNIT_TYPE_QUANTITY_LOCKED");
pub const UNIT_TYPE_HAS_RESERVATIONS: ErrorCode = ErrorCode::new("UNIT_TYPE_HAS_RESERVATIONS");

pub mod violation {
    pub const ARRIVAL_CLOSED: &str = "ARRIVAL_CLOSED";
    pub const DEPARTURE_CLOSED: &str = "DEPARTURE_CLOSED";
    pub const MIN_NIGHTS: &str = "MIN_NIGHTS";
    pub const MAX_NIGHTS: &str = "MAX_NIGHTS";
    pub const ADVANCE_NOTICE: &str = "ADVANCE_NOTICE";
    pub const MAX_HORIZON: &str = "MAX_HORIZON";
    pub const TURNOVER_BUFFER: &str = "TURNOVER_BUFFER";
}

pub const STAY_RULE_VIOLATED: ErrorCode = ErrorCode::new("STAY_RULE_VIOLATED");
