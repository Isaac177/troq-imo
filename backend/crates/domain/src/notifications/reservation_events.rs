use serde_json::Value;

use super::catalog as events;
use super::event_data::{preview, text, with_names};
use super::notice::Notice;

fn reservation(notice: Notice, data: &Value, event: &str) -> Notice {
    let reservation_id = text(data, "reservationId");
    with_names(notice, data)
        .entity("reservation", reservation_id)
        .dedupe(format!("{event}:{reservation_id}:{}", text(data, "amendmentId")))
}

fn to_guest(kind: &'static str, data: &Value, event: &str) -> Option<Notice> {
    let guest = text(data, "guestUserId");
    (!guest.is_empty()).then(|| reservation(Notice::user(kind, guest), data, event))
}

fn to_host(kind: &'static str, data: &Value, event: &str) -> Option<Notice> {
    let host = text(data, "ownerUserId");
    (!host.is_empty()).then(|| reservation(Notice::user(kind, host), data, event))
}

fn message(data: &Value, event: &str) -> Option<Notice> {
    let from_guest = text(data, "senderRole").eq_ignore_ascii_case("GUEST");
    let notice = if from_guest {
        to_host(events::MESSAGE_HOST, data, event)
    } else {
        to_guest(events::MESSAGE_GUEST, data, event)
    };
    notice.map(|notice| notice.param("preview", preview(text(data, "body"))))
}

fn special_request(data: &Value, event: &str) -> Option<Notice> {
    let accepted = text(data, "specialRequestState").eq_ignore_ascii_case("ACCEPTED");
    let (decision, decision_fr) = if accepted { ("accepted", "acceptée") } else { ("declined", "refusée") };
    to_guest(events::SPECIAL_REQUEST_DECIDED_GUEST, data, event)
        .map(|notice| notice.param("decision", decision).param("decisionFr", decision_fr))
}

pub fn notices(event: &str, data: &Value) -> Vec<Notice> {
    let found = match event {
        "reservation.held" => vec![to_host(events::RESERVATION_HELD_HOST, data, event)],
        "reservation.request_accepted" => vec![to_guest(events::RESERVATION_ACCEPTED_GUEST, data, event)],
        "reservation.cancelled" => vec![
            to_guest(events::RESERVATION_CANCELLED_GUEST, data, event),
            to_host(events::RESERVATION_CANCELLED_HOST, data, event),
        ],
        "reservation.damage_claimed" => vec![to_guest(events::DAMAGE_CLAIMED_GUEST, data, event)],
        "reservation.amendment_proposed" => vec![to_guest(events::AMENDMENT_PROPOSED_GUEST, data, event)],
        "reservation.amendment_declined" => vec![to_host(events::AMENDMENT_DECLINED_HOST, data, event)],
        "reservation.amendment_accepted" => vec![to_host(events::AMENDMENT_ACCEPTED_HOST, data, event)],
        "reservation.message_sent" => vec![message(data, event)],
        "reservation.review_submitted" => vec![to_host(events::REVIEW_SUBMITTED_HOST, data, event)],
        "reservation.review_replied" => vec![to_guest(events::REVIEW_REPLIED_GUEST, data, event)],
        "reservation.special_request_decided" => vec![special_request(data, event)],
        _ => Vec::new(),
    };
    found.into_iter().flatten().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn data() -> Value {
        json!({
            "reservationId": "r1", "guestUserId": "g1", "ownerUserId": "h1", "guestName": "Ada",
            "propertyName": {"en": "Villa", "fr": "Maison"}, "amendmentId": "a1"
        })
    }

    #[test]
    fn held_notifies_the_host_only() {
        let found = notices("reservation.held", &data());
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].event, events::RESERVATION_HELD_HOST);
        assert_eq!(found[0].dedupe_key.as_deref(), Some("reservation.held:r1:a1"));
        assert_eq!(found[0].params["propertyNameFr"], "Maison");
    }

    #[test]
    fn cancelled_notifies_both_parties_and_skips_missing_users() {
        assert_eq!(notices("reservation.cancelled", &data()).len(), 2);
        let guest_only = json!({"reservationId": "r1", "guestUserId": "g1"});
        let found = notices("reservation.cancelled", &guest_only);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].event, events::RESERVATION_CANCELLED_GUEST);
    }

    #[test]
    fn guest_message_goes_to_the_host_with_a_preview() {
        let mut payload = data();
        payload["senderRole"] = json!("guest");
        payload["body"] = json!("Is parking available?");
        let found = notices("reservation.message_sent", &payload);
        assert_eq!(found[0].event, events::MESSAGE_HOST);
        assert_eq!(found[0].params["preview"], "Is parking available?");
        payload["senderRole"] = json!("HOST");
        assert_eq!(notices("reservation.message_sent", &payload)[0].event, events::MESSAGE_GUEST);
    }

    #[test]
    fn special_request_carries_both_decision_words() {
        let mut payload = data();
        payload["specialRequestState"] = json!("ACCEPTED");
        let found = notices("reservation.special_request_decided", &payload);
        assert_eq!(found[0].params["decisionFr"], "acceptée");
        payload["specialRequestState"] = json!("DECLINED");
        assert_eq!(notices("reservation.special_request_decided", &payload)[0].params["decision"], "declined");
    }

    #[test]
    fn unknown_events_produce_nothing() {
        assert!(notices("reservation.expired", &data()).is_empty());
    }
}
