use serde_json::Value;

use super::catalog as events;
use super::event_data::{text, with_names};
use super::notice::Notice;

fn property(notice: Notice, data: &Value, event: &str, entity: &str) -> Notice {
    let property_id = text(data, "propertyId");
    with_names(notice, data)
        .entity(entity, property_id)
        .dedupe(format!("{event}:{property_id}:{}", text(data, "revisionId")))
}

fn host_notice(kind: &'static str, data: &Value, event: &str) -> Option<Notice> {
    let host = text(data, "ownerUserId");
    (!host.is_empty()).then(|| property(Notice::user(kind, host), data, event, "property"))
}

pub fn property_notices(event: &str, data: &Value) -> Vec<Notice> {
    let found = match event {
        "property.created" => {
            Some(property(Notice::platform_team(events::PROPERTY_CREATED_ADMIN), data, event, "property"))
        }
        "property.revision.submitted" => {
            Some(property(Notice::platform_team(events::REVISION_SUBMITTED_ADMIN), data, event, "revision"))
        }
        "property.suspended" => host_notice(events::PROPERTY_SUSPENDED_HOST, data, event),
        "property.reinstated" => host_notice(events::PROPERTY_REINSTATED_HOST, data, event),
        _ => None,
    };
    found.into_iter().collect()
}

pub fn case_notices(event: &str, data: &Value) -> Vec<Notice> {
    let host = text(data, "ownerUserId");
    let case_id = text(data, "caseId");
    match event {
        "case.opened" => {
            let admins = Notice::platform_team(events::CASE_OPENED_ADMIN)
                .entity("case", case_id)
                .dedupe(format!("case.opened.admin:{case_id}"));
            let mut found = vec![admins];
            if !host.is_empty() {
                found.push(
                    Notice::user(events::CASE_OPENED_HOST, host)
                        .entity("case", case_id)
                        .dedupe(format!("case.opened.host:{case_id}")),
                );
            }
            found
        }
        "case.closed" if !host.is_empty() => vec![Notice::user(events::CASE_CLOSED_PARTIES, host)
            .entity("case", case_id)
            .dedupe(format!("case.closed:{case_id}"))],
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn property_created_targets_the_platform_team() {
        let data = json!({"propertyId": "p1", "propertyName": {"en": "Villa"}});
        let found = property_notices("property.created", &data);
        assert_eq!(found[0].event, events::PROPERTY_CREATED_ADMIN);
        assert_eq!(found[0].entity, Some(("property".into(), "p1".into())));
    }

    #[test]
    fn revision_uses_the_revision_entity_and_dedupe() {
        let data = json!({"propertyId": "p1", "revisionId": "v2"});
        let found = property_notices("property.revision.submitted", &data);
        assert_eq!(found[0].entity, Some(("revision".into(), "p1".into())));
        assert_eq!(found[0].dedupe_key.as_deref(), Some("property.revision.submitted:p1:v2"));
    }

    #[test]
    fn suspension_needs_an_owner() {
        let data = json!({"propertyId": "p1"});
        assert!(property_notices("property.suspended", &data).is_empty());
        let owned = json!({"propertyId": "p1", "ownerUserId": "h1"});
        assert_eq!(property_notices("property.reinstated", &owned)[0].event, events::PROPERTY_REINSTATED_HOST);
    }

    #[test]
    fn case_opened_notifies_admins_and_owner() {
        let data = json!({"caseId": "c1", "ownerUserId": "h1"});
        assert_eq!(case_notices("case.opened", &data).len(), 2);
        assert_eq!(case_notices("case.opened", &json!({"caseId": "c1"})).len(), 1);
        assert_eq!(case_notices("case.closed", &data)[0].event, events::CASE_CLOSED_PARTIES);
        assert!(case_notices("case.closed", &json!({"caseId": "c1"})).is_empty());
    }
}
