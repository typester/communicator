use std::sync::Arc;

use logger::Logger;

mod logger;

#[uniffi::export]
pub fn setup_logger(logger: Arc<dyn Logger>) {
    
}

uniffi::setup_scaffolding!();
