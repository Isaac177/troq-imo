mod normalize;
mod print;
mod session;

use std::path::PathBuf;
use std::time::Duration;

use anyhow::{bail, Result};
use tokio::sync::Mutex;

pub use normalize::normalize;
pub use print::footer_template;

use session::Session;

const WARM_UP_HTML: &str = "<!doctype html><html><body><p>warm-up</p></body></html>";

pub struct Engine {
    chrome: PathBuf,
    recycle_after: u32,
    timeout: Duration,
    slot: Mutex<Slot>,
}

#[derive(Default)]
struct Slot {
    session: Option<Session>,
    renders: u32,
}

impl Slot {
    async fn retire(&mut self) {
        if let Some(session) = self.session.take() {
            session.close().await;
        }
        self.renders = 0;
    }
}

impl Engine {
    pub fn new(chrome: PathBuf, recycle_after: u32, timeout_secs: u64) -> Self {
        Self {
            chrome,
            recycle_after: recycle_after.max(1),
            timeout: Duration::from_secs(timeout_secs.max(1)),
            slot: Mutex::new(Slot::default()),
        }
    }

    pub async fn render_pdf(&self, html: &str, footer: Option<&str>) -> Result<Vec<u8>> {
        let mut slot = self.slot.lock().await;
        if slot.session.is_none() || slot.renders >= self.recycle_after {
            slot.retire().await;
            let session = Session::launch(&self.chrome).await?;
            session.print(WARM_UP_HTML, None).await?;
            slot.session = Some(session);
        }
        let outcome = {
            let session = slot.session.as_ref().expect("session launched above");
            tokio::time::timeout(self.timeout, session.print(html, footer)).await
        };
        match outcome {
            Ok(Ok(bytes)) => {
                slot.renders += 1;
                Ok(bytes)
            }
            Ok(Err(error)) => {
                slot.retire().await;
                Err(error)
            }
            Err(_) => {
                slot.retire().await;
                bail!("document render timed out after {}s", self.timeout.as_secs())
            }
        }
    }
}
