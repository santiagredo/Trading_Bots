use std::sync::Arc;

use models::structs::{CacheRuntimeTokens, Environments};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

use crate::{handler::Cancellations, utils::Cache};

static ACTIVE_RUNTIME_TOKEN: Lazy<Arc<RwLock<CacheRuntimeTokens>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheRuntimeTokens::default())));

impl Cancellations<Cache> {
    pub async fn set_runtime_token_cache(self, token: CancellationToken) {
        let mut cache = ACTIVE_RUNTIME_TOKEN.write().await;

        if let Some(token) = cache.environment_tokens.get_mut(&self.environment) {
            token.cancel();
        }

        cache.environment_tokens.insert(self.environment, token);
    }

    pub async fn get_runtime_token_cache(environment: Environments) -> Option<CancellationToken> {
        let cache = ACTIVE_RUNTIME_TOKEN.read().await;

        cache.environment_tokens.get(&environment).cloned()
    }

    pub async fn stop_runtime_cache(self) {
        let mut cache = ACTIVE_RUNTIME_TOKEN.write().await;

        if let Some(token) = cache.environment_tokens.remove(&self.environment) {
            token.cancel();
        }
    }
}
