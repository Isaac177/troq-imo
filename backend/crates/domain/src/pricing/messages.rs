use kernel::ErrorCode;

pub const PROPERTY_NOT_FOUND: ErrorCode = ErrorCode::new("PROPERTY_NOT_FOUND");
pub const RATE_PLAN_NOT_FOUND: ErrorCode = ErrorCode::new("RATE_PLAN_NOT_FOUND");
pub const RATE_RULE_NOT_FOUND: ErrorCode = ErrorCode::new("RATE_RULE_NOT_FOUND");
pub const PROMOTION_NOT_FOUND: ErrorCode = ErrorCode::new("PROMOTION_NOT_FOUND");
pub const QUOTE_NOT_FOUND: ErrorCode = ErrorCode::new("QUOTE_NOT_FOUND");
pub const POLICY_NOT_FOUND: ErrorCode = ErrorCode::new("POLICY_NOT_FOUND");
pub const UNIT_TYPE_NOT_IN_PROPERTY: ErrorCode = ErrorCode::new("UNIT_TYPE_NOT_IN_PROPERTY");
pub const RATE_PLAN_NOT_IN_PROPERTY: ErrorCode = ErrorCode::new("RATE_PLAN_NOT_IN_PROPERTY");

pub const CHOICE_INVALID: ErrorCode = ErrorCode::new("CHOICE_INVALID");
pub const AMOUNT_TOO_LARGE: ErrorCode = ErrorCode::new("AMOUNT_TOO_LARGE");
pub const BASE_RATE_NOT_POSITIVE: ErrorCode = ErrorCode::new("BASE_RATE_NOT_POSITIVE");
pub const PERCENT_OUT_OF_RANGE: ErrorCode = ErrorCode::new("PERCENT_OUT_OF_RANGE");
pub const PERIOD_INVALID: ErrorCode = ErrorCode::new("PERIOD_INVALID");
pub const PRIORITY_OUT_OF_RANGE: ErrorCode = ErrorCode::new("PRIORITY_OUT_OF_RANGE");
pub const WEEKDAYS_INVALID: ErrorCode = ErrorCode::new("WEEKDAYS_INVALID");
pub const MIN_NIGHTS_OUT_OF_RANGE: ErrorCode = ErrorCode::new("MIN_NIGHTS_OUT_OF_RANGE");
pub const REDEMPTION_LIMIT_OUT_OF_RANGE: ErrorCode = ErrorCode::new("REDEMPTION_LIMIT_OUT_OF_RANGE");

pub const RATE_PLAN_ARCHIVED: ErrorCode = ErrorCode::new("RATE_PLAN_ARCHIVED");
pub const RATE_PLAN_NEEDS_BASE_RATE: ErrorCode = ErrorCode::new("RATE_PLAN_NEEDS_BASE_RATE");
pub const RATE_PLAN_CHANGED: ErrorCode = ErrorCode::new("RATE_PLAN_CHANGED");
pub const BASE_RATE_OVERLAP: ErrorCode = ErrorCode::new("BASE_RATE_OVERLAP");
pub const REQUEST_IN_PROGRESS: ErrorCode = ErrorCode::new("REQUEST_IN_PROGRESS");
pub const PROMOTION_LIMIT_BELOW_REDEEMED: ErrorCode = ErrorCode::new("PROMOTION_LIMIT_BELOW_REDEEMED");
pub const PROMOTION_GONE: ErrorCode = ErrorCode::new("PROMOTION_GONE");
pub const PROMOTION_BUDGET_USED: ErrorCode = ErrorCode::new("PROMOTION_BUDGET_USED");
pub const FUNDING_UNKNOWN: ErrorCode = ErrorCode::new("FUNDING_UNKNOWN");

pub const NO_BASE_RATE: ErrorCode = ErrorCode::new("NO_BASE_RATE");
pub const BASE_RATE_CONFLICT: ErrorCode = ErrorCode::new("BASE_RATE_CONFLICT");
pub const STAY_DATES_INVALID: ErrorCode = ErrorCode::new("STAY_DATES_INVALID");
pub const STAY_TOO_LONG: ErrorCode = ErrorCode::new("STAY_TOO_LONG");
pub const GUEST_COUNT_INVALID: ErrorCode = ErrorCode::new("GUEST_COUNT_INVALID");
pub const CHECK_IN_IN_PAST: ErrorCode = ErrorCode::new("CHECK_IN_IN_PAST");
pub const PROPERTY_NOT_PUBLISHED: ErrorCode = ErrorCode::new("PROPERTY_NOT_PUBLISHED");
pub const HOST_NOT_ACTIVE: ErrorCode = ErrorCode::new("HOST_NOT_ACTIVE");
pub const UNIT_TYPE_NOT_ACTIVE: ErrorCode = ErrorCode::new("UNIT_TYPE_NOT_ACTIVE");
pub const RATE_PLAN_NOT_ACTIVE: ErrorCode = ErrorCode::new("RATE_PLAN_NOT_ACTIVE");
pub const RATE_PLAN_UNIT_TYPE_MISMATCH: ErrorCode = ErrorCode::new("RATE_PLAN_UNIT_TYPE_MISMATCH");
pub const GUEST_CAPACITY_EXCEEDED: ErrorCode = ErrorCode::new("GUEST_CAPACITY_EXCEEDED");
pub const NOT_AVAILABLE: ErrorCode = ErrorCode::new("NOT_AVAILABLE");
pub const POLICIES_NOT_ACTIVE: ErrorCode = ErrorCode::new("POLICIES_NOT_ACTIVE");
pub const CANCELLATION_POLICY_MISSING: ErrorCode = ErrorCode::new("CANCELLATION_POLICY_MISSING");
pub const CANCELLATION_POLICY_INVALID: ErrorCode = ErrorCode::new("CANCELLATION_POLICY_INVALID");
pub const TIMEZONE_UNSUPPORTED: ErrorCode = ErrorCode::new("TIMEZONE_UNSUPPORTED");

pub const SETTING_UNKNOWN: ErrorCode = ErrorCode::new("SETTING_UNKNOWN");
pub const SETTING_OUT_OF_RANGE: ErrorCode = ErrorCode::new("SETTING_OUT_OF_RANGE");
pub const SETTING_REASON_UNKNOWN: ErrorCode = ErrorCode::new("SETTING_REASON_UNKNOWN");
pub const SETTING_UNCHANGED: ErrorCode = ErrorCode::new("SETTING_UNCHANGED");
