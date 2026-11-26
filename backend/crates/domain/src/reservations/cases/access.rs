use kernel::auth::Actor;
use kernel::permissions::names::{CASES_WRITE, RESERVATIONS_READ};

use super::types::CaseView;
use crate::reservations::lifecycle::access::has_permission;

pub fn is_guest_party(case: &CaseView, user_id: &str) -> bool {
    case.parties.get("guestUserId").and_then(|value| value.as_str()) == Some(user_id)
}

fn host_allows(actor: &Actor, permission: &str, case: &CaseView) -> bool {
    has_permission(actor, permission)
        && actor.access.is_member_of(&case.operator_id)
        && actor.require_scoped(permission, Some(case.operator_id.as_str()), None).is_ok()
}

pub fn can_view(actor: &Actor, case: &CaseView) -> bool {
    is_guest_party(case, actor.user_id().as_str()) || host_allows(actor, RESERVATIONS_READ, case)
}

pub fn can_manage(actor: &Actor, case: &CaseView) -> bool {
    is_guest_party(case, actor.user_id().as_str()) || host_allows(actor, CASES_WRITE, case)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use serde_json::json;

    fn case(guest: &str) -> CaseView {
        CaseView {
            id: "case-1".into(),
            operator_id: "host-1".into(),
            reservation_id: None,
            case_type: "GENERAL".into(),
            reason: "r".into(),
            parties: json!({ "guestUserId": guest }),
            state: "OPEN".into(),
            deadline_at: Utc::now(),
            resolution: None,
            version: 1,
            created_at: Utc::now(),
            closed_at: None,
            reservation_reference: None,
        }
    }

    #[test]
    fn the_guest_party_is_read_from_the_case_parties() {
        assert!(is_guest_party(&case("u1"), "u1"));
        assert!(!is_guest_party(&case("u1"), "u2"));
    }
}
