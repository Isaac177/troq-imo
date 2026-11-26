use chrono::{DateTime, Utc};

use super::types::{CalendarBlock, CalendarReservation};

const UID_DOMAIN: &str = "stays.mungini";

pub fn build(
    property_id: &str,
    reservations: &[CalendarReservation],
    blocks: &[CalendarBlock],
    now: DateTime<Utc>,
) -> String {
    let stamp = now.format("%Y%m%dT%H%M%SZ").to_string();
    let mut lines: Vec<String> = vec![
        "BEGIN:VCALENDAR".into(),
        "VERSION:2.0".into(),
        "PRODID:-//Mungini//Stays//EN".into(),
        "CALSCALE:GREGORIAN".into(),
        format!("X-WR-CALNAME:{}", escape(&format!("Stays {property_id}"))),
    ];
    for reservation in reservations {
        let (summary, status) = match reservation.state.as_str() {
            "REQUESTED" | "PENDING_PAYMENT" => (format!("Request {}", reservation.reference), "TENTATIVE"),
            "CHECKED_OUT" => (format!("Stayed {}", reservation.reference), "CONFIRMED"),
            _ => (format!("Stay {}", reservation.reference), "CONFIRMED"),
        };
        lines.extend(event(
            &format!("{}@{UID_DOMAIN}", reservation.id),
            &stamp,
            reservation.check_in.format("%Y%m%d").to_string(),
            reservation.check_out.format("%Y%m%d").to_string(),
            &summary,
            status,
        ));
    }
    for block in blocks {
        lines.extend(event(
            &format!("{}@{UID_DOMAIN}", block.id),
            &stamp,
            block.start_date.format("%Y%m%d").to_string(),
            block.end_date.format("%Y%m%d").to_string(),
            &format!("Unavailable ({})", block.reason.to_lowercase()),
            "CONFIRMED",
        ));
    }
    lines.push("END:VCALENDAR".into());
    lines.join("\r\n") + "\r\n"
}

fn event(uid: &str, stamp: &str, start: String, end: String, summary: &str, status: &str) -> Vec<String> {
    vec![
        "BEGIN:VEVENT".into(),
        format!("UID:{}", escape(uid)),
        format!("DTSTAMP:{stamp}"),
        format!("DTSTART;VALUE=DATE:{start}"),
        format!("DTEND;VALUE=DATE:{end}"),
        format!("SUMMARY:{}", escape(summary)),
        format!("STATUS:{status}"),
        "TRANSP:OPAQUE".into(),
        "END:VEVENT".into(),
    ]
}

fn escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace(';', "\\;")
        .replace(',', "\\,")
        .replace('\n', "\\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, TimeZone};

    #[test]
    fn escapes_ics_text() {
        assert_eq!(escape("a,b;c\\d"), "a\\,b\\;c\\\\d");
    }

    #[test]
    fn builds_a_valid_envelope_with_date_only_events() {
        let reservation = CalendarReservation {
            id: "res-1".into(),
            reference: "STY-GOMA-0001".into(),
            state: "CONFIRMED".into(),
            version: 1,
            total: "0.00".into(),
            adult_count: 1,
            child_count: 0,
            unit_type_id: "u".into(),
            physical_unit_id: None,
            check_in: NaiveDate::from_ymd_opt(2026, 12, 22).unwrap(),
            check_out: NaiveDate::from_ymd_opt(2026, 12, 24).unwrap(),
            guest_name: None,
            pending_amendment: None,
            unit_type_cover_url: None,
        };
        let now = Utc.with_ymd_and_hms(2026, 10, 12, 10, 0, 0).unwrap();
        let text = build("p", &[reservation], &[], now);
        assert!(text.starts_with("BEGIN:VCALENDAR\r\n"));
        assert!(text.contains("DTSTART;VALUE=DATE:20261222\r\n"));
        assert!(text.contains("DTEND;VALUE=DATE:20261224\r\n"));
        assert!(text.ends_with("END:VCALENDAR\r\n"));
    }
}
