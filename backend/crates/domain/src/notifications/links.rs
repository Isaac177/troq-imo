fn guest(entity_type: &str, id: &str) -> String {
    match entity_type {
        "reservation" | "stay" => format!("/trips?reservation={id}"),
        _ => "/notifications".to_string(),
    }
}

fn host(entity_type: &str, id: &str) -> String {
    match entity_type {
        "reservation" => format!("/reservations?reservation={id}"),
        "property" => format!("/properties/{id}"),
        "case" => format!("/cases?case={id}"),
        "task" => format!("/tasks?task={id}"),
        _ => "/".to_string(),
    }
}

fn admin(entity_type: &str) -> String {
    match entity_type {
        "property" | "revision" | "case" => "/audit".to_string(),
        _ => "/notifications".to_string(),
    }
}

pub fn deep_link(audience: &str, entity_type: Option<&str>, entity_id: Option<&str>) -> String {
    let entity_type = entity_type.unwrap_or_default();
    let id = entity_id.unwrap_or_default();
    match audience {
        "admin" => admin(entity_type),
        "host" => host(entity_type, id),
        _ => guest(entity_type, id),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guest_reservation_links_to_trips() {
        assert_eq!(deep_link("guest", Some("reservation"), Some("r1")), "/trips?reservation=r1");
    }

    #[test]
    fn host_links_by_entity() {
        assert_eq!(deep_link("host", Some("property"), Some("p1")), "/properties/p1");
        assert_eq!(deep_link("host", Some("case"), Some("c1")), "/cases?case=c1");
        assert_eq!(deep_link("host", None, None), "/");
    }

    #[test]
    fn admin_property_events_go_to_the_audit_view() {
        assert_eq!(deep_link("admin", Some("revision"), Some("p1")), "/audit");
    }
}
