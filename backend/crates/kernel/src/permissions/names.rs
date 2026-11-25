pub const PROPERTIES_READ: &str = "properties.read";
pub const PROPERTIES_WRITE: &str = "properties.write";
pub const PROPERTIES_CONFIGURE: &str = "properties.configure";
pub const PROPERTIES_MODERATE: &str = "properties.moderate";
pub const INVENTORY_WRITE: &str = "inventory.write";
pub const INVENTORY_INTEGRATIONS: &str = "inventory.integrations";
pub const PRICING_WRITE: &str = "pricing.write";
pub const PRICING_CAMPAIGNS: &str = "pricing.campaigns";
pub const RESERVATIONS_READ: &str = "reservations.read";
pub const RESERVATIONS_WRITE: &str = "reservations.write";
pub const RESERVATIONS_GUEST_DETAILS: &str = "reservations.guest_details";
pub const OPERATIONS_CHECKIN: &str = "operations.checkin";
pub const OPERATIONS_HOUSEKEEPING: &str = "operations.housekeeping";
pub const OPERATIONS_MAINTENANCE: &str = "operations.maintenance";
pub const MESSAGES_WRITE: &str = "messages.write";
pub const CASES_WRITE: &str = "cases.write";
pub const REVIEWS_WRITE: &str = "reviews.write";
pub const REVIEWS_MODERATE: &str = "reviews.moderate";
pub const BILLING_READ: &str = "billing.read";
pub const BILLING_REQUEST_REFUNDS: &str = "billing.request_refunds";
pub const BILLING_APPROVE_REFUNDS: &str = "billing.approve_refunds";
pub const BILLING_PAYOUT_DETAILS: &str = "billing.payout_details";
pub const BILLING_SETTLEMENTS: &str = "billing.settlements";
pub const BILLING_APPROVE_PAYOUTS: &str = "billing.approve_payouts";
pub const ANALYTICS_READ: &str = "analytics.read";
pub const ANALYTICS_EXPORT: &str = "analytics.export";
pub const HOSTS_TEAM: &str = "hosts.team";
pub const HOSTS_SETTINGS: &str = "hosts.settings";
pub const HOSTS_REVIEW_APPLICATIONS: &str = "hosts.review_applications";
pub const AUDIT_READ: &str = "audit.read";

pub const ALL: &[&str] = &[
    PROPERTIES_READ,
    PROPERTIES_WRITE,
    PROPERTIES_CONFIGURE,
    PROPERTIES_MODERATE,
    INVENTORY_WRITE,
    INVENTORY_INTEGRATIONS,
    PRICING_WRITE,
    PRICING_CAMPAIGNS,
    RESERVATIONS_READ,
    RESERVATIONS_WRITE,
    RESERVATIONS_GUEST_DETAILS,
    OPERATIONS_CHECKIN,
    OPERATIONS_HOUSEKEEPING,
    OPERATIONS_MAINTENANCE,
    MESSAGES_WRITE,
    CASES_WRITE,
    REVIEWS_WRITE,
    REVIEWS_MODERATE,
    BILLING_READ,
    BILLING_REQUEST_REFUNDS,
    BILLING_APPROVE_REFUNDS,
    BILLING_PAYOUT_DETAILS,
    BILLING_SETTLEMENTS,
    BILLING_APPROVE_PAYOUTS,
    ANALYTICS_READ,
    ANALYTICS_EXPORT,
    HOSTS_TEAM,
    HOSTS_SETTINGS,
    HOSTS_REVIEW_APPLICATIONS,
    AUDIT_READ,
];

pub const LEGACY: &[(&str, &str)] = &[
    ("STAY_VIEW_PROPERTIES", PROPERTIES_READ),
    ("STAY_MANAGE_PROPERTIES", PROPERTIES_WRITE),
    ("STAY_MANAGE_CONFIGURATION", PROPERTIES_CONFIGURE),
    ("STAY_MODERATE_LISTINGS", PROPERTIES_MODERATE),
    ("STAY_MANAGE_AVAILABILITY", INVENTORY_WRITE),
    ("STAY_MANAGE_INTEGRATIONS", INVENTORY_INTEGRATIONS),
    ("STAY_MANAGE_RATES", PRICING_WRITE),
    ("STAY_MANAGE_CAMPAIGNS", PRICING_CAMPAIGNS),
    ("STAY_VIEW_RESERVATIONS", RESERVATIONS_READ),
    ("STAY_MANAGE_RESERVATIONS", RESERVATIONS_WRITE),
    ("STAY_VIEW_GUEST_DETAILS", RESERVATIONS_GUEST_DETAILS),
    ("STAY_MANAGE_CHECKIN", OPERATIONS_CHECKIN),
    ("STAY_MANAGE_HOUSEKEEPING", OPERATIONS_HOUSEKEEPING),
    ("STAY_MANAGE_MAINTENANCE", OPERATIONS_MAINTENANCE),
    ("STAY_MANAGE_MESSAGES", MESSAGES_WRITE),
    ("STAY_MANAGE_CASES", CASES_WRITE),
    ("STAY_MANAGE_REVIEWS", REVIEWS_WRITE),
    ("STAY_MODERATE_REVIEWS", REVIEWS_MODERATE),
    ("STAY_VIEW_FINANCE", BILLING_READ),
    ("STAY_REQUEST_REFUNDS", BILLING_REQUEST_REFUNDS),
    ("STAY_APPROVE_REFUNDS", BILLING_APPROVE_REFUNDS),
    ("STAY_MANAGE_PAYOUT_DETAILS", BILLING_PAYOUT_DETAILS),
    ("STAY_MANAGE_SETTLEMENTS", BILLING_SETTLEMENTS),
    ("STAY_APPROVE_PAYOUTS", BILLING_APPROVE_PAYOUTS),
    ("STAY_VIEW_ANALYTICS", ANALYTICS_READ),
    ("STAY_EXPORT_DATA", ANALYTICS_EXPORT),
    ("STAY_MANAGE_TEAM", HOSTS_TEAM),
    ("STAY_MANAGE_BUSINESS_SETTINGS", HOSTS_SETTINGS),
    ("STAY_REVIEW_APPLICATIONS", HOSTS_REVIEW_APPLICATIONS),
    ("STAY_VIEW_AUDIT", AUDIT_READ),
];

pub fn from_legacy(name: &str) -> Option<&'static str> {
    LEGACY
        .iter()
        .find(|(legacy, _)| *legacy == name)
        .map(|(_, current)| *current)
}

pub fn is_known(name: &str) -> bool {
    ALL.contains(&name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_is_domain_dot_action() {
        for name in ALL {
            let (domain, action) = name.split_once('.').expect("domain.action");
            assert!(!domain.is_empty() && !action.is_empty());
            assert!(name
                .chars()
                .all(|c| c.is_ascii_lowercase() || c == '.' || c == '_'));
        }
    }

    #[test]
    fn names_are_unique() {
        let mut sorted = ALL.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ALL.len());
    }

    #[test]
    fn every_legacy_permission_maps_to_a_known_name_once() {
        assert_eq!(LEGACY.len(), ALL.len());
        for (_, current) in LEGACY {
            assert!(is_known(current));
        }
        assert_eq!(from_legacy("STAY_VIEW_PROPERTIES"), Some(PROPERTIES_READ));
        assert_eq!(from_legacy("STAY_UNKNOWN"), None);
    }
}
