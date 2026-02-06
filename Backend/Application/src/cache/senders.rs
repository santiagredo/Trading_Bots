use crate::handler::Senders;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::RwLock;

static ACTIVE_SENDERS: Lazy<Arc<RwLock<Senders>>> =
    Lazy::new(|| Arc::new(RwLock::new(Senders::new())));

impl Senders {
    pub async fn get_senders_cache() -> Senders {
        let senders_lock = ACTIVE_SENDERS.read().await;

        senders_lock.clone()
    }
}
