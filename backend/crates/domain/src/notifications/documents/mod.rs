mod assets;
mod build;
mod cancellation;
mod checks;
mod damage_claim;
mod engine;
mod format;
mod i18n;
mod issue;
mod kind;
mod layout;
mod payload;
mod payload_common;
mod pipeline;
mod preview;
mod read;
mod store;
mod template;
mod voucher;

pub use issue::{issue, Issued};
pub use kind::{DocumentKind, Locale};
pub use preview::preview;
pub use read::{fetch, list, DocumentScope};
pub use assets::verify_assets;
pub use i18n::check_parity;

use std::path::Path;
use std::sync::OnceLock;

use kernel::{Error, Result};

use super::config::NotificationsConfig;

use engine::Engine;

static ENGINE: OnceLock<Engine> = OnceLock::new();

fn engine(config: &NotificationsConfig) -> Result<&'static Engine> {
    if let Some(engine) = ENGINE.get() {
        return Ok(engine);
    }
    let chrome = config
        .chrome_path
        .as_deref()
        .filter(|path| Path::new(path).is_file())
        .ok_or_else(|| Error::internal("DOCUMENT_CHROME_PATH is not configured or does not exist"))?;
    verify_assets(&config.assets_dir).map_err(store::internal)?;
    check_parity(&config.assets_dir).map_err(store::internal)?;
    Ok(ENGINE.get_or_init(|| Engine::new(chrome.into(), config.recycle_after, config.render_timeout_secs)))
}
