use std::path::Path;

use anyhow::{anyhow, Result};
use serde_json::{Map, Value};

use super::i18n;
use super::kind::{DocumentKind, Locale};

pub fn required_str(value: &Value, key: &str) -> Result<String> {
    first_str(value, &[key]).ok_or_else(|| anyhow!("document payload is missing {key}"))
}

pub fn first_str(value: &Value, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| {
        value
            .get(*key)
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|v| !v.is_empty())
            .map(str::to_string)
    })
}

pub fn property_name(value: &Value, locale: Locale) -> Result<String> {
    let names = value.get("propertyName").ok_or_else(|| anyhow!("stay snapshot is missing propertyName"))?;
    names
        .get(locale.code())
        .and_then(Value::as_str)
        .filter(|v| !v.trim().is_empty())
        .or_else(|| names.get("en").and_then(Value::as_str))
        .map(str::to_string)
        .ok_or_else(|| anyhow!("stay snapshot has no property name"))
}

pub fn guest_name(value: &Value) -> Result<String> {
    let guest = value.get("guest").ok_or_else(|| anyhow!("stay snapshot is missing guest"))?;
    first_str(guest, &["fullName"]).ok_or_else(|| anyhow!("stay guest has no name"))
}

pub struct ViewBase {
    pub labels: Map<String, Value>,
    pub missing: String,
    pub view: Map<String, Value>,
}

pub fn view_base(root: &Path, kind: DocumentKind, locale: Locale, brand: &str) -> Result<ViewBase> {
    let labels = i18n::section(root, locale, kind.section(), brand)?;
    let missing = labels.get("missing").and_then(Value::as_str).unwrap_or_default().to_string();
    let mut view = Map::new();
    view.insert("labels".into(), Value::Object(labels.clone()));
    view.insert("brand".into(), Value::String(brand.to_string()));
    Ok(ViewBase { labels, missing, view })
}

pub fn put(view: &mut Map<String, Value>, key: &str, value: impl Into<String>) {
    view.insert(key.into(), Value::String(value.into()));
}

pub fn amount(value: &Value, key: &str) -> Option<Result<f64>> {
    value.get(key).and_then(|v| {
        v.as_str().map(str::to_string).or_else(|| v.as_f64().map(|n| n.to_string()))
    }).map(|text| text.parse::<f64>().map_err(|error| anyhow!("{key}: {error}")))
}
