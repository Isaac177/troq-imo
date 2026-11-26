use serde_json::{Map, Value};

fn param_text(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Number(number) => number.to_string(),
        Value::Bool(flag) => flag.to_string(),
        _ => String::new(),
    }
}

pub fn fill(template: &str, params: &Map<String, Value>) -> String {
    let mut out = String::with_capacity(template.len() + 16);
    let mut rest = template;
    while let Some(start) = rest.find('{') {
        out.push_str(&rest[..start]);
        let after = &rest[start + 1..];
        match after.find('}') {
            Some(end) => {
                let key = &after[..end];
                let value = params
                    .get(key)
                    .or_else(|| key.strip_suffix("Fr").and_then(|base| params.get(base)));
                if let Some(value) = value {
                    out.push_str(&param_text(value));
                }
                rest = &after[end + 1..];
            }
            None => {
                out.push_str(&rest[start..]);
                rest = "";
            }
        }
    }
    out.push_str(rest);
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn pick<'a>(language: &str, english: &'a str, french: &'a str) -> &'a str {
    if language.to_ascii_lowercase().starts_with("fr") && !french.is_empty() {
        french
    } else {
        english
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn params(value: Value) -> Map<String, Value> {
        value.as_object().cloned().unwrap()
    }

    #[test]
    fn fills_placeholders_and_collapses_whitespace() {
        let filled = fill("{guestName} requested  {propertyName}.", &params(json!({"guestName": "Ada", "propertyName": "Villa"})));
        assert_eq!(filled, "Ada requested Villa.");
    }

    #[test]
    fn french_suffix_falls_back_to_the_base_key() {
        let only_base = params(json!({"propertyName": "Villa"}));
        assert_eq!(fill("{propertyNameFr}", &only_base), "Villa");
        let both = params(json!({"propertyName": "Villa", "propertyNameFr": "Maison"}));
        assert_eq!(fill("{propertyNameFr}", &both), "Maison");
    }

    #[test]
    fn missing_keys_render_empty_and_unclosed_braces_stay() {
        assert_eq!(fill("a {x} b", &Map::new()), "a b");
        assert_eq!(fill("a {x", &Map::new()), "a {x");
    }

    #[test]
    fn picks_french_only_for_french_languages() {
        assert_eq!(pick("fr-CD", "Hello", "Bonjour"), "Bonjour");
        assert_eq!(pick("en", "Hello", "Bonjour"), "Hello");
        assert_eq!(pick("fr", "Hello", ""), "Hello");
    }
}
