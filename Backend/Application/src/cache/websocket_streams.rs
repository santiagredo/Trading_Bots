use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

use crate::handler::WebsocketStreams;

static WEBSOCKET_CANCELLATION_TOKEN: Lazy<Arc<RwLock<Option<CancellationToken>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl WebsocketStreams {
    pub async fn set_cancellation_token_cache(token: CancellationToken) {
        let mut token_lock = WEBSOCKET_CANCELLATION_TOKEN.write().await;

        // Cancel previous token if exists
        if let Some(prev_token) = token_lock.as_ref() {
            prev_token.cancel();
        }

        *token_lock = Some(token);
    }

    pub async fn get_cancellation_token_cache() -> Option<CancellationToken> {
        WEBSOCKET_CANCELLATION_TOKEN.read().await.clone()
    }

    pub async fn remove_cancellation_token_cache() -> Option<CancellationToken> {
        WEBSOCKET_CANCELLATION_TOKEN.write().await.take()
    }
}
