use std::path::Path;

use anyhow::{Context, Result};
use sha2::{Digest, Sha256};

use super::assets::font_face_css;
use super::checks::{ensure_passes, inspect, PdfReport};
use super::engine::{footer_template, normalize, Engine};
use super::i18n;
use super::kind::{DocumentKind, Locale};
use super::layout::compose;
use super::template::render_body;

pub struct RenderRequest<'a> {
    pub kind: DocumentKind,
    pub locale: Locale,
    pub brand: &'a str,
    pub view: &'a serde_json::Value,
    pub title: &'a str,
    pub required: &'a [String],
    pub file_stem: &'a str,
}

pub struct Generated {
    pub bytes: Vec<u8>,
    pub sha256: String,
    pub page_count: usize,
    pub file_name: String,
    pub report: PdfReport,
}

pub async fn render(engine: &Engine, root: &Path, request: RenderRequest<'_>) -> Result<Generated> {
    let body = render_body(root, request.kind, request.view)?;
    let html = compose(root, request.locale, request.title, &body)?;
    let document = i18n::section(root, request.locale, "document", request.brand)?;
    let brand = document.get("brand").and_then(|v| v.as_str()).unwrap_or_default();
    let page_word = document.get("page").and_then(|v| v.as_str()).unwrap_or_default();
    let footer = footer_template(&font_face_css(root)?, brand, page_word);
    let raw = engine.render_pdf(&html, Some(&footer)).await.context("rendering document pdf")?;
    let bytes = normalize(&raw);
    let report = inspect(&bytes, request.required)?;
    ensure_passes(&report)?;
    let sha256 = hex::encode(Sha256::digest(&bytes));
    Ok(Generated {
        page_count: report.pages,
        file_name: format!("{}-{}.pdf", request.kind.file_prefix(), request.file_stem),
        bytes,
        sha256,
        report,
    })
}
