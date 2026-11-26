use std::path::Path;

use anyhow::{bail, Result};
use serde_json::{Map, Value};

use super::assets::read_text;
use super::kind::Locale;

pub fn load(root: &Path, locale: Locale) -> Result<Value> {
    let relative = format!("templates/i18n/{}.json", locale.code());
    Ok(serde_json::from_str(&read_text(root, &relative)?)?)
}

fn apply_brand(value: &Value, brand: &str) -> Value {
    match value {
        Value::String(text) => Value::String(text.replace("{brand}", brand)),
        Value::Object(map) => Value::Object(map.iter().map(|(key, child)| (key.clone(), apply_brand(child, brand))).collect()),
        other => other.clone(),
    }
}

pub fn section(root: &Path, locale: Locale, name: &str, brand: &str) -> Result<Map<String, Value>> {
    match load(root, locale)?.get(name) {
        Some(found @ Value::Object(_)) => match apply_brand(found, brand) {
            Value::Object(map) => Ok(map),
            _ => bail!("translation section {name} is not an object"),
        },
        _ => bail!("translation section {name} missing in {}", locale.code()),
    }
}

pub fn key_paths(value: &Value, prefix: &str, out: &mut Vec<String>) {
    if let Value::Object(map) = value {
        for (key, child) in map {
            let path = if prefix.is_empty() { key.clone() } else { format!("{prefix}.{key}") };
            key_paths(child, &path, out);
        }
    } else {
        out.push(prefix.to_string());
    }
}

pub fn check_parity(root: &Path) -> Result<()> {
    let mut en = Vec::new();
    let mut fr = Vec::new();
    key_paths(&load(root, Locale::En)?, "", &mut en);
    key_paths(&load(root, Locale::Fr)?, "", &mut fr);
    en.sort();
    fr.sort();
    if en != fr {
        bail!("translation keys differ between en and fr");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notifications::config::NotificationsConfig;

    #[test]
    fn english_and_french_share_every_key() {
        check_parity(&NotificationsConfig::from_env().assets_dir).unwrap();
    }

    #[test]
    fn brand_placeholder_is_replaced_everywhere() {
        let root = NotificationsConfig::from_env().assets_dir;
        for locale in [Locale::En, Locale::Fr] {
            let document = section(&root, locale, "document", "Mungini").unwrap();
            assert_eq!(document["brand"], "Mungini");
            let claim = section(&root, locale, "stayDamageClaim", "Mungini").unwrap();
            assert!(claim["footer"].as_str().unwrap().contains("Mungini"));
            assert!(!claim["footer"].as_str().unwrap().contains("{brand}"));
        }
    }

    #[test]
    fn unknown_section_is_an_error() {
        let root = NotificationsConfig::from_env().assets_dir;
        assert!(section(&root, Locale::En, "invoice", "Mungini").is_err());
    }
}
