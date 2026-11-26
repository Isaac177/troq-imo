use anyhow::{bail, Context, Result};
use lopdf::{Dictionary, Document, Object};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PdfReport {
    pub pages: usize,
    pub fonts: Vec<String>,
    pub all_fonts_embedded: bool,
    pub unrendered_markers: bool,
    pub iso_dates: bool,
    pub missing_values: Vec<String>,
}

pub fn inspect(bytes: &[u8], required: &[String]) -> Result<PdfReport> {
    let doc = Document::load_mem(bytes).context("parsing rendered pdf")?;
    let pages = doc.get_pages();
    let mut fonts: Vec<String> = Vec::new();
    let mut embedded = true;
    for page_id in pages.values() {
        for (_, font) in doc.get_page_fonts(*page_id).unwrap_or_default() {
            let base = base_font(font).unwrap_or_default();
            if !fonts.contains(&base) {
                fonts.push(base);
            }
            embedded &= is_embedded(&doc, font);
        }
    }
    let page_numbers: Vec<u32> = pages.keys().copied().collect();
    let text = doc.extract_text(&page_numbers).unwrap_or_default();
    let squashed = compact(&text);
    let missing_values = required
        .iter()
        .filter(|value| !squashed.contains(&compact(value)))
        .cloned()
        .collect();
    Ok(PdfReport {
        pages: pages.len(),
        fonts,
        all_fonts_embedded: embedded,
        unrendered_markers: text.contains("{{") || text.contains("}}"),
        iso_dates: has_iso_date(&text),
        missing_values,
    })
}

pub fn ensure_passes(report: &PdfReport) -> Result<()> {
    if !report.all_fonts_embedded {
        bail!("document has fonts that are not embedded: {:?}", report.fonts);
    }
    if report.unrendered_markers {
        bail!("document contains unrendered template markers");
    }
    if report.iso_dates {
        bail!("document shows a raw ISO date");
    }
    if !report.missing_values.is_empty() {
        bail!("document is missing expected values: {:?}", report.missing_values);
    }
    Ok(())
}

fn base_font(font: &Dictionary) -> Option<String> {
    font.get(b"BaseFont")
        .ok()
        .and_then(|object| object.as_name().ok())
        .map(|name| String::from_utf8_lossy(name).to_string())
}

fn is_embedded(doc: &Document, font: &Dictionary) -> bool {
    let descriptor = match font.get(b"Subtype").and_then(Object::as_name) {
        Ok(b"Type0") => descendant_descriptor(doc, font),
        _ => font_descriptor(doc, font),
    };
    descriptor.is_some_and(|desc| {
        desc.has(b"FontFile") || desc.has(b"FontFile2") || desc.has(b"FontFile3")
    })
}

fn font_descriptor<'a>(doc: &'a Document, font: &'a Dictionary) -> Option<&'a Dictionary> {
    font.get_deref(b"FontDescriptor", doc)
        .ok()
        .and_then(|object| object.as_dict().ok())
}

fn descendant_descriptor<'a>(doc: &'a Document, font: &'a Dictionary) -> Option<&'a Dictionary> {
    let descendants = font.get_deref(b"DescendantFonts", doc).ok()?.as_array().ok()?;
    let descendant = match descendants.first()? {
        Object::Reference(id) => doc.get_dictionary(*id).ok()?,
        Object::Dictionary(dict) => dict,
        _ => return None,
    };
    font_descriptor(doc, descendant)
}

fn compact(value: &str) -> String {
    value.chars().filter(|c| !c.is_whitespace()).collect()
}

fn has_iso_date(text: &str) -> bool {
    text.as_bytes().windows(10).any(|window| {
        window[4] == b'-'
            && window[7] == b'-'
            && window[..4].iter().all(u8::is_ascii_digit)
            && window[5..7].iter().all(u8::is_ascii_digit)
            && window[8..10].iter().all(u8::is_ascii_digit)
    })
}
