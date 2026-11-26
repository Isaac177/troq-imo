use std::path::Path;

use anyhow::Result;

use super::assets::{font_face_css, read_text};
use super::kind::Locale;

pub fn compose(root: &Path, locale: Locale, title: &str, body: &str) -> Result<String> {
    let styles = [
        font_face_css(root)?,
        read_text(root, "templates/design/tokens.css")?,
        read_text(root, "templates/design/components.css")?,
        read_text(root, "templates/design/page.css")?,
    ]
    .join("\n");
    Ok(format!(
        "<!doctype html>\n<html lang=\"{lang}\">\n<head>\n<meta charset=\"utf-8\">\n<title>{title}</title>\n<style>\n{styles}\n</style>\n</head>\n<body>\n{body}\n</body>\n</html>",
        lang = locale.code(),
        title = escape_title(title),
    ))
}

fn escape_title(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}
