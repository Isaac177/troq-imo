use kernel::City;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Event {
    pub id: String,
    pub city: City,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub event_type: String,
    pub version: i32,
    pub data: Value,
}

impl Event {
    pub fn text(&self, key: &str) -> Option<&str> {
        self.data.get(key).and_then(Value::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn text_reads_string_fields_only() {
        let event = Event {
            id: "evt-1".into(),
            city: City::GOMA,
            aggregate_type: "reservation".into(),
            aggregate_id: "res-1".into(),
            event_type: "reservation.held".into(),
            version: 1,
            data: json!({ "reference": "MG-1", "nights": 3 }),
        };
        assert_eq!(event.text("reference"), Some("MG-1"));
        assert_eq!(event.text("nights"), None);
        assert_eq!(event.text("missing"), None);
    }
}
