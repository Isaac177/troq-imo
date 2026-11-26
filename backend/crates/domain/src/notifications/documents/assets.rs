use std::path::Path;

use anyhow::{bail, Context, Result};
use base64::Engine;

const REQUIRED: &[&str] = &[
    "assets/Onest/static/Onest-Regular.ttf",
    "assets/Onest/static/Onest-SemiBold.ttf",
    "templates/design/tokens.css",
    "templates/design/components.css",
    "templates/design/page.css",
    "templates/documents/stay_voucher.hbs",
    "templates/documents/stay_cancellation.hbs",
    "templates/documents/stay_damage_claim.hbs",
    "templates/i18n/en.json",
    "templates/i18n/fr.json",
];

pub fn verify_assets(root: &Path) -> Result<()> {
    let missing: Vec<String> = REQUIRED
        .iter()
        .filter(|rel| !root.join(rel).is_file())
        .map(|rel| rel.to_string())
        .collect();
    if missing.is_empty() {
        return Ok(());
    }
    bail!("document assets missing under {}: {}", root.display(), missing.join(", "))
}

pub fn read_text(root: &Path, relative: &str) -> Result<String> {
    let path = root.join(relative);
    std::fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))
}

pub fn font_face_css(root: &Path) -> Result<String> {
    let regular = font_data_uri(root, "assets/Onest/static/Onest-Regular.ttf")?;
    let semibold = font_data_uri(root, "assets/Onest/static/Onest-SemiBold.ttf")?;
    Ok(format!(
        "@font-face {{ font-family: 'Onest'; font-weight: 400; src: url('{regular}') format('truetype'); }}\n\
         @font-face {{ font-family: 'Onest'; font-weight: 600; src: url('{semibold}') format('truetype'); }}"
    ))
}

fn font_data_uri(root: &Path, relative: &str) -> Result<String> {
    let path = root.join(relative);
    let bytes = std::fs::read(&path).with_context(|| format!("reading {}", path.display()))?;
    Ok(format!("data:font/ttf;base64,{}", base64::engine::general_purpose::STANDARD.encode(bytes)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notifications::config::NotificationsConfig;

    #[test]
    fn shipped_assets_are_complete() {
        let root = NotificationsConfig::from_env().assets_dir;
        verify_assets(&root).unwrap();
        assert!(font_face_css(&root).unwrap().contains("Onest"));
    }

    #[test]
    fn missing_assets_are_named() {
        let error = verify_assets(Path::new("/nonexistent")).unwrap_err().to_string();
        assert!(error.contains("stay_voucher.hbs"));
    }
}
