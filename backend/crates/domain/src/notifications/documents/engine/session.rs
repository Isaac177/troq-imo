use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use anyhow::{Context, Result};
use base64::Engine as _;
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::page::PrintToPdfParams;
use chromiumoxide::Page;
use futures::StreamExt;
use tokio::task::JoinHandle;

use super::print::{MARGIN_BOTTOM_IN, MARGIN_TOP_IN};

static PROFILE_COUNTER: AtomicU64 = AtomicU64::new(0);

pub struct Session {
    browser: Browser,
    page: Page,
    handler: JoinHandle<()>,
    profile: PathBuf,
}

impl Session {
    pub async fn launch(chrome: &Path) -> Result<Self> {
        let profile = std::env::temp_dir().join(format!(
            "mungini-doc-chrome-{}-{}",
            std::process::id(),
            PROFILE_COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        let config = BrowserConfig::builder()
            .chrome_executable(chrome)
            .user_data_dir(&profile)
            .no_sandbox()
            .arg("--disable-dev-shm-usage")
            .arg("--disable-gpu")
            .arg("--hide-scrollbars")
            .arg("--font-render-hinting=none")
            .build()
            .map_err(anyhow::Error::msg)
            .context("building chromium config")?;
        let (browser, mut events) = Browser::launch(config)
            .await
            .context("launching chromium for documents")?;
        let handler = tokio::spawn(async move { while events.next().await.is_some() {} });
        let page = browser
            .new_page("about:blank")
            .await
            .context("opening document page")?;
        Ok(Self {
            browser,
            page,
            handler,
            profile,
        })
    }

    pub async fn print(&self, html: &str, footer: Option<&str>) -> Result<Vec<u8>> {
        let url = format!(
            "data:text/html;base64,{}",
            base64::engine::general_purpose::STANDARD.encode(html)
        );
        self.page
            .goto(url)
            .await
            .context("loading document html")?;
        let mut params = PrintToPdfParams::builder()
            .print_background(true)
            .prefer_css_page_size(true)
            .margin_top(MARGIN_TOP_IN)
            .margin_bottom(MARGIN_BOTTOM_IN)
            .margin_left(0.0)
            .margin_right(0.0);
        if let Some(footer) = footer {
            params = params
                .display_header_footer(true)
                .header_template("<span></span>")
                .footer_template(footer);
        }
        self.page
            .pdf(params.build())
            .await
            .context("printing document pdf")
    }

    pub async fn close(mut self) {
        if let Err(error) = self.browser.close().await {
            tracing::warn!(error = %error, "notifications.documents.chromium_close_failed");
        }
        self.browser.wait().await.ok();
        self.handler.abort();
        std::fs::remove_dir_all(&self.profile).ok();
    }
}
