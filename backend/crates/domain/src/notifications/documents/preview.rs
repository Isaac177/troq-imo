use kernel::Result;
use serde_json::Value;

use super::build::build;
use super::kind::{DocumentKind, Locale};
use super::store::internal;
use crate::notifications::config::NotificationsConfig;

fn fixture(config: &NotificationsConfig, kind: DocumentKind, locale: Locale) -> anyhow::Result<Value> {
    let relative = format!("templates/fixtures/{}-{}.json", kind.as_str(), locale.code());
    Ok(serde_json::from_str(&super::assets::read_text(&config.assets_dir, &relative)?)?)
}

pub async fn preview(config: &NotificationsConfig, kind: DocumentKind, locale: Locale) -> Result<(String, Vec<u8>)> {
    let engine = super::engine(config)?;
    let snapshot = fixture(config, kind, locale).map_err(internal)?;
    let built = build(engine, &config.assets_dir, &config.product_name, kind, &snapshot, locale)
        .await
        .map_err(internal)?
        .ok_or_else(|| kernel::Error::internal("fixture does not match the document state"))?;
    Ok((built.generated.file_name, built.generated.bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notifications::documents::payload::Payload;

    #[test]
    fn every_fixture_parses_into_its_payload_and_view() {
        let config = NotificationsConfig::from_env();
        for kind in DocumentKind::ALL {
            for locale in [Locale::En, Locale::Fr] {
                let snapshot = fixture(&config, kind, locale).unwrap();
                let payload = Payload::parse(kind, &snapshot, locale).unwrap().expect("fixture state");
                let view = payload.view(&config.assets_dir, &config.product_name).unwrap();
                let html = super::super::template::render_body(&config.assets_dir, kind, &view).unwrap();
                assert!(html.contains(payload.reference()));
                assert!(html.contains("Mungini"));
                assert!(!html.contains("{{"));
            }
        }
    }
}
