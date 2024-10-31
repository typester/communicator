use std::{
    fmt::Debug,
    sync::{Arc, LazyLock, Once},
};

use db::Db;
use error::CommunicatorError;
use tracing_subscriber::{layer::SubscriberExt, Registry};

use config::Config;
use logger::{FFILogLayer, Logger};

mod config;
mod db;
mod error;
mod logger;
mod token_store;

uniffi::setup_scaffolding!();

static RT: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to initialize tokio runtime")
});

pub(crate) fn rt() -> &'static tokio::runtime::Runtime {
    &*RT
}

static INIT_LOGGER: Once = Once::new();

#[uniffi::export]
pub fn init_logger(logger: Arc<dyn Logger>) {
    INIT_LOGGER.call_once(|| {
        let subscriber = Registry::default().with(FFILogLayer(logger));
        tracing::subscriber::set_global_default(subscriber)
            .expect("failed to set global log subscriber");
    });
}

#[derive(uniffi::Object)]
pub struct Communicator {
    config: Config,
    db: Arc<Db>,
}

#[uniffi::export]
impl Communicator {
    #[uniffi::constructor]
    pub fn new(config: Config) -> Result<Arc<Self>, CommunicatorError> {
        rt().block_on(async move {
            let db = Db::new(config.database_url.clone())?;
            Ok(Arc::new(Self { config, db }))
        })
    }

    pub async fn db_migration_available(&self) -> Result<bool, CommunicatorError> {
        let db = self.db.clone();
        rt().spawn(async move { db.migration_available().await })
            .await?
    }

    pub async fn db_migration(&self) -> Result<(), CommunicatorError> {
        let db = self.db.clone();
        Ok(rt().spawn(async move { db.do_migration().await }).await??)
    }

    pub async fn db_reset(&self) -> Result<(), CommunicatorError> {
        Ok(self.db.reset()?)
    }

    pub fn is_token_available(&self) -> bool {
        self.config.token_store.get().is_some()
    }

    pub async fn login(&self) -> Result<(), CommunicatorError> {
        unimplemented!()
    }

    pub async fn register_app(&self) -> Result<(), CommunicatorError> {
        unimplemented!()
    }
}
