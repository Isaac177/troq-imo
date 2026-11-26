use kernel::permissions::names;

pub const DELEGABLE: &[&str] = &[
    names::PROPERTIES_READ,
    names::PROPERTIES_WRITE,
    names::PROPERTIES_CONFIGURE,
    names::INVENTORY_WRITE,
    names::INVENTORY_INTEGRATIONS,
    names::PRICING_WRITE,
    names::PRICING_CAMPAIGNS,
    names::RESERVATIONS_READ,
    names::RESERVATIONS_WRITE,
    names::RESERVATIONS_GUEST_DETAILS,
    names::OPERATIONS_CHECKIN,
    names::OPERATIONS_HOUSEKEEPING,
    names::OPERATIONS_MAINTENANCE,
    names::MESSAGES_WRITE,
    names::REVIEWS_WRITE,
    names::BILLING_READ,
    names::BILLING_REQUEST_REFUNDS,
    names::BILLING_PAYOUT_DETAILS,
    names::ANALYTICS_READ,
    names::ANALYTICS_EXPORT,
    names::HOSTS_TEAM,
    names::HOSTS_SETTINGS,
    names::AUDIT_READ,
];

pub const PLATFORM_ONLY: &[&str] = &[
    names::PROPERTIES_MODERATE,
    names::REVIEWS_MODERATE,
    names::BILLING_APPROVE_REFUNDS,
    names::BILLING_SETTLEMENTS,
    names::BILLING_APPROVE_PAYOUTS,
    names::HOSTS_REVIEW_APPLICATIONS,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresetAudience {
    Host,
    Platform,
}

impl PresetAudience {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Host => "HOST",
            Self::Platform => "PLATFORM",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Preset {
    pub code: &'static str,
    pub audience: PresetAudience,
    pub permissions: &'static [&'static str],
}

pub const PRESETS: &[Preset] = &[
    Preset {
        code: "HOST_OWNER",
        audience: PresetAudience::Host,
        permissions: DELEGABLE,
    },
    Preset {
        code: "PROPERTY_MANAGER",
        audience: PresetAudience::Host,
        permissions: &[
            names::PROPERTIES_READ,
            names::PROPERTIES_WRITE,
            names::PROPERTIES_CONFIGURE,
            names::INVENTORY_WRITE,
            names::PRICING_WRITE,
            names::RESERVATIONS_READ,
            names::RESERVATIONS_WRITE,
            names::OPERATIONS_CHECKIN,
            names::MESSAGES_WRITE,
            names::REVIEWS_WRITE,
            names::ANALYTICS_READ,
        ],
    },
    Preset {
        code: "RESERVATIONS_DESK",
        audience: PresetAudience::Host,
        permissions: &[
            names::RESERVATIONS_READ,
            names::RESERVATIONS_WRITE,
            names::RESERVATIONS_GUEST_DETAILS,
            names::MESSAGES_WRITE,
            names::OPERATIONS_CHECKIN,
        ],
    },
    Preset {
        code: "REVENUE_SPECIALIST",
        audience: PresetAudience::Host,
        permissions: &[
            names::PROPERTIES_READ,
            names::PRICING_WRITE,
            names::PRICING_CAMPAIGNS,
            names::ANALYTICS_READ,
            names::ANALYTICS_EXPORT,
        ],
    },
    Preset {
        code: "HOUSEKEEPING",
        audience: PresetAudience::Host,
        permissions: &[
            names::RESERVATIONS_READ,
            names::OPERATIONS_CHECKIN,
            names::OPERATIONS_HOUSEKEEPING,
        ],
    },
    Preset {
        code: "MAINTENANCE",
        audience: PresetAudience::Host,
        permissions: &[names::PROPERTIES_READ, names::OPERATIONS_MAINTENANCE],
    },
    Preset {
        code: "HOST_ACCOUNTANT",
        audience: PresetAudience::Host,
        permissions: &[
            names::BILLING_READ,
            names::BILLING_REQUEST_REFUNDS,
            names::BILLING_PAYOUT_DETAILS,
            names::ANALYTICS_READ,
        ],
    },
    Preset {
        code: "PLATFORM_REVIEWER",
        audience: PresetAudience::Platform,
        permissions: &[
            names::PROPERTIES_READ,
            names::PROPERTIES_MODERATE,
            names::REVIEWS_MODERATE,
            names::HOSTS_REVIEW_APPLICATIONS,
            names::AUDIT_READ,
        ],
    },
    Preset {
        code: "SUPPORT",
        audience: PresetAudience::Platform,
        permissions: &[
            names::PROPERTIES_READ,
            names::RESERVATIONS_READ,
            names::RESERVATIONS_WRITE,
            names::RESERVATIONS_GUEST_DETAILS,
            names::MESSAGES_WRITE,
            names::CASES_WRITE,
            names::REVIEWS_MODERATE,
        ],
    },
    Preset {
        code: "PLATFORM_FINANCE",
        audience: PresetAudience::Platform,
        permissions: &[
            names::BILLING_READ,
            names::BILLING_APPROVE_REFUNDS,
            names::BILLING_SETTLEMENTS,
            names::BILLING_APPROVE_PAYOUTS,
            names::ANALYTICS_READ,
            names::ANALYTICS_EXPORT,
        ],
    },
];

pub fn preset(code: &str) -> Option<&'static Preset> {
    PRESETS.iter().find(|preset| preset.code == code)
}

pub fn is_delegable(permission: &str) -> bool {
    DELEGABLE.contains(&permission)
}

#[cfg(test)]
pub fn validate_delegable(permissions: &[String]) -> Vec<&str> {
    permissions
        .iter()
        .map(String::as_str)
        .filter(|permission| !is_delegable(permission))
        .collect()
}

pub fn domain(permission: &str) -> &str {
    permission.split_once('.').map(|(domain, _)| domain).unwrap_or(permission)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_preset_uses_known_permissions() {
        for preset in PRESETS {
            assert!(!preset.permissions.is_empty());
            for permission in preset.permissions {
                assert!(names::is_known(permission));
            }
        }
    }

    #[test]
    fn delegable_and_platform_only_sets_do_not_overlap() {
        for permission in PLATFORM_ONLY {
            assert!(names::is_known(permission));
            assert!(!is_delegable(permission));
        }
    }

    #[test]
    fn unknown_presets_and_permissions_are_reported() {
        assert!(preset("HOST_OWNER").is_some());
        assert!(preset("UNKNOWN").is_none());
        let values = [names::PROPERTIES_READ.to_string(), names::BILLING_SETTLEMENTS.to_string()];
        let rejected = validate_delegable(&values);
        assert_eq!(rejected, vec![names::BILLING_SETTLEMENTS]);
    }
}
