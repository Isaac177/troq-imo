use std::sync::Arc;

use anyhow::Result;

use crate::auth::{AccessDirectory, TokenVerifier};
use crate::config::Config;
use crate::db::Db;
use crate::storage::Storage;

#[derive(Clone)]
pub struct Kernel {
    pub db: Db,
    pub config: Arc<Config>,
    pub storage: Storage,
    pub tokens: TokenVerifier,
    pub directory: Arc<dyn AccessDirectory>,
}

impl Kernel {
    pub async fn connect(config: Config, directory: Arc<dyn AccessDirectory>) -> Result<Self> {
        let db = Db::connect(&config.database_url).await?;
        let storage = Storage::new(config.storage.clone());
        let tokens = TokenVerifier::new(&config.jwt_secret);
        Ok(Self {
            db,
            config: Arc::new(config),
            storage,
            tokens,
            directory,
        })
    }

    pub fn with_directory(mut self, directory: Arc<dyn AccessDirectory>) -> Self {
        self.directory = directory;
        self
    }
}

pub trait KernelState {
    fn kernel(&self) -> &Kernel;
}

impl KernelState for Kernel {
    fn kernel(&self) -> &Kernel {
        self
    }
}
