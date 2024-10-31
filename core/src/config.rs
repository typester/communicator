use std::sync::Arc;

use crate::token_store::TokenStore;

#[derive(uniffi::Record)]
pub struct Config {
    pub token_store: Arc<dyn TokenStore>,
    pub database_url: String,
}
