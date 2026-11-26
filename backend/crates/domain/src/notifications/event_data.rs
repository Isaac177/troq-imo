use serde_json::Value;

use super::notice::Notice;

pub fn text<'a>(data: &'a Value, key: &str) -> &'a str {
    data.get(key).and_then(Value::as_str).unwrap_or_default()
}

pub fn property_name(data: &Value, language: &str) -> String {
    let names = data.get("propertyName").unwrap_or(&Value::Null);
    names
        .get(language)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .or_else(|| names.get("en").and_then(Value::as_str))
        .unwrap_or_default()
        .to_string()
}

pub fn preview(body: &str) -> String {
    let trimmed: String = body.chars().take(90).collect();
    if body.chars().count() > 90 {
        format!("{trimmed}...")
    } else {
        trimmed
    }
}

pub fn with_names(notice: Notice, data: &Value) -> Notice {
    notice
        .param("propertyName", property_name(data, "en"))
        .param("propertyNameFr", property_name(data, "fr"))
        .param("guestName", text(data, "guestName"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn property_name_prefers_the_language_then_english() {
        let data = json!({"propertyName": {"en": "Villa", "fr": ""}});
        assert_eq!(property_name(&data, "fr"), "Villa");
        assert_eq!(property_name(&data, "en"), "Villa");
        assert_eq!(property_name(&json!({}), "en"), "");
    }

    #[test]
    fn preview_cuts_long_bodies_at_ninety_characters() {
        assert_eq!(preview("short"), "short");
        let long = "a".repeat(100);
        assert_eq!(preview(&long), format!("{}...", "a".repeat(90)));
    }
}
