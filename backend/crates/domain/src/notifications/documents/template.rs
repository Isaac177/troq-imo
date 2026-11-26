use std::path::Path;

use anyhow::{Context, Result};
use handlebars::Handlebars;
use serde_json::Value;

use super::assets::read_text;
use super::kind::DocumentKind;

pub fn render_body(root: &Path, kind: DocumentKind, view: &Value) -> Result<String> {
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    let source = read_text(root, kind.template_path())?;
    handlebars.register_template_string(kind.as_str(), source).context("compiling document template")?;
    handlebars.render(kind.as_str(), view).context("rendering document template")
}
