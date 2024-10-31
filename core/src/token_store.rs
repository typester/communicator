use std::{
    fmt::Debug,
    sync::{Arc, RwLock},
};

#[uniffi::export(with_foreign)]
pub trait TokenStore: Send + Sync + Debug {
    fn get(&self) -> Option<String>;
    fn set(&self, token: String);
}

#[derive(Debug)]
pub(crate) struct MemoryTokenStore {
    inner: Arc<RwLock<Option<String>>>,
}

impl TokenStore for MemoryTokenStore {
    fn get(&self) -> Option<String> {
        self.inner.read().ok()?.clone()
    }

    fn set(&self, token: String) {
        match self.inner.write() {
            Ok(mut inner) => *inner = Some(token),
            Err(err) => {
                tracing::error!(%err, "failed to store token");
            }
        }
    }
}
