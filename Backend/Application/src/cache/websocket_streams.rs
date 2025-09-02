use std::sync::Arc;

use once_cell::sync::Lazy;
use tokio::{sync::RwLock, task::AbortHandle};

use crate::{handler::WebsocketStreams, utils::Cache};

static BINANCE_WS_ABORT_HANDLE: Lazy<Arc<RwLock<Option<AbortHandle>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl WebsocketStreams<Cache> {
    pub async fn set_binance_ws_abort_handle_cache(abort_handle: AbortHandle) {
        let mut memory_abort_handle_lock = BINANCE_WS_ABORT_HANDLE.write().await;

        if let Some(memory_abort_handle) = memory_abort_handle_lock.as_ref() {
            memory_abort_handle.abort();
        }

        *memory_abort_handle_lock = Some(abort_handle);
    }

    pub async fn get_binance_ws_abort_handle_cache() -> Option<AbortHandle> {
        let memory_abort_handle_lock = BINANCE_WS_ABORT_HANDLE.read().await;

        memory_abort_handle_lock.clone()
    }
}
