use std::path::Path;

use anyhow::{Context, Result};
use serde_json::Value;

use super::engine::Engine;
use super::i18n;
use super::kind::{DocumentKind, Locale};
use super::payload::Payload;
use super::pipeline::{self, Generated, RenderRequest};

pub struct Built {
    pub generated: Generated,
    pub reference: String,
}

pub async fn build(
    engine: &Engine,
    root: &Path,
    brand: &str,
    kind: DocumentKind,
    snapshot: &Value,
    locale: Locale,
) -> Result<Option<Built>> {
    let Some(payload) = Payload::parse(kind, snapshot, locale)? else {
        return Ok(None);
    };
    let view = payload.view(root, brand)?;
    let title = i18n::section(root, locale, kind.section(), brand)?
        .get("title")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let required = payload.required_values();
    let stem = format!("{}-{}", payload.reference(), locale.code());
    let generated = pipeline::render(
        engine,
        root,
        RenderRequest { kind, locale, brand, view: &view, title: &title, required: &required, file_stem: &stem },
    )
    .await
    .with_context(|| format!("building {}", kind.as_str()))?;
    Ok(Some(Built { generated, reference: payload.reference().to_string() }))
}

pub fn mail_copy(root: &Path, brand: &str, kind: DocumentKind, locale: Locale, reference: &str) -> Result<(String, String)> {
    let section = i18n::section(root, locale, kind.section(), brand)?;
    let text = |key: &str| section.get(key).and_then(Value::as_str).unwrap_or_default().to_string();
    Ok((text("subject").replace("{{reference}}", reference), text("body")))
}
