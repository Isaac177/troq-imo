#[derive(Debug, Clone, Copy)]
pub struct EventDef {
    pub key: &'static str,
    pub category: &'static str,
    pub audience: &'static str,
    pub priority: &'static str,
    pub db_type: &'static str,
    pub default_email: bool,
}

const fn def(
    key: &'static str,
    audience: &'static str,
    priority: &'static str,
    db_type: &'static str,
    default_email: bool,
) -> EventDef {
    EventDef { key, category: "stays", audience, priority, db_type, default_email }
}

pub const RESERVATION_HELD_HOST: &str = "stay.reservation_held.host";
pub const RESERVATION_ACCEPTED_GUEST: &str = "stay.reservation_accepted.guest";
pub const RESERVATION_CANCELLED_GUEST: &str = "stay.reservation_cancelled.guest";
pub const RESERVATION_CANCELLED_HOST: &str = "stay.reservation_cancelled.host";
pub const DAMAGE_CLAIMED_GUEST: &str = "stay.damage_claimed.guest";
pub const AMENDMENT_PROPOSED_GUEST: &str = "stay.amendment_proposed.guest";
pub const AMENDMENT_DECLINED_HOST: &str = "stay.amendment_declined.host";
pub const AMENDMENT_ACCEPTED_HOST: &str = "stay.amendment_accepted.host";
pub const MESSAGE_GUEST: &str = "stay.message.guest";
pub const MESSAGE_HOST: &str = "stay.message.host";
pub const REVIEW_SUBMITTED_HOST: &str = "stay.review_submitted.host";
pub const REVIEW_REPLIED_GUEST: &str = "stay.review_replied.guest";
pub const SPECIAL_REQUEST_DECIDED_GUEST: &str = "stay.special_request_decided.guest";
pub const PROPERTY_CREATED_ADMIN: &str = "stay.property_created.admin";
pub const REVISION_SUBMITTED_ADMIN: &str = "stay.revision_submitted.admin";
pub const PROPERTY_SUSPENDED_HOST: &str = "stay.property_suspended.host";
pub const PROPERTY_REINSTATED_HOST: &str = "stay.property_reinstated.host";
pub const CASE_OPENED_ADMIN: &str = "stay.case_opened.admin";
pub const CASE_OPENED_HOST: &str = "stay.case_opened.host";
pub const CASE_CLOSED_PARTIES: &str = "stay.case_closed.parties";

pub static EVENTS: &[EventDef] = &[
    def(RESERVATION_HELD_HOST, "host", "HIGH", "REMINDER", false),
    def(RESERVATION_ACCEPTED_GUEST, "guest", "HIGH", "INFO", true),
    def(RESERVATION_CANCELLED_GUEST, "guest", "HIGH", "ALERT", true),
    def(RESERVATION_CANCELLED_HOST, "host", "HIGH", "ALERT", false),
    def(DAMAGE_CLAIMED_GUEST, "guest", "HIGH", "ALERT", true),
    def(AMENDMENT_PROPOSED_GUEST, "guest", "HIGH", "REMINDER", false),
    def(AMENDMENT_DECLINED_HOST, "host", "NORMAL", "INFO", false),
    def(AMENDMENT_ACCEPTED_HOST, "host", "NORMAL", "INFO", false),
    def(MESSAGE_GUEST, "guest", "NORMAL", "MESSAGE", false),
    def(MESSAGE_HOST, "host", "NORMAL", "MESSAGE", false),
    def(REVIEW_SUBMITTED_HOST, "host", "NORMAL", "REVIEW_RECEIVED", false),
    def(REVIEW_REPLIED_GUEST, "guest", "NORMAL", "REVIEW_RECEIVED", false),
    def(SPECIAL_REQUEST_DECIDED_GUEST, "guest", "HIGH", "INFO", false),
    def(PROPERTY_CREATED_ADMIN, "admin", "NORMAL", "INFO", false),
    def(REVISION_SUBMITTED_ADMIN, "admin", "NORMAL", "INFO", false),
    def(PROPERTY_SUSPENDED_HOST, "host", "URGENT", "ALERT", true),
    def(PROPERTY_REINSTATED_HOST, "host", "HIGH", "INFO", true),
    def(CASE_OPENED_ADMIN, "admin", "HIGH", "ALERT", false),
    def(CASE_OPENED_HOST, "host", "HIGH", "ALERT", false),
    def(CASE_CLOSED_PARTIES, "host", "NORMAL", "INFO", false),
];

pub fn find(key: &str) -> Option<&'static EventDef> {
    EVENTS.iter().find(|event| event.key == key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_are_unique_and_stay_scoped() {
        for (index, event) in EVENTS.iter().enumerate() {
            assert!(event.key.starts_with("stay."));
            assert!(EVENTS.iter().skip(index + 1).all(|other| other.key != event.key));
        }
        assert_eq!(EVENTS.len(), 20);
    }

    #[test]
    fn suspension_is_urgent_and_emailed() {
        let event = find(PROPERTY_SUSPENDED_HOST).unwrap();
        assert_eq!(event.priority, "URGENT");
        assert!(event.default_email);
    }

    #[test]
    fn audiences_are_the_three_apps() {
        assert!(EVENTS.iter().all(|event| ["guest", "host", "admin"].contains(&event.audience)));
    }
}
