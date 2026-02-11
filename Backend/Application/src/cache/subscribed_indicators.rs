use crate::handler::SubscribedIndicators;
use chrono::Local;
use models::{
    enums::{FiniteStateMachine, LifecycleState},
    structs::CacheSubscribedIndicators,
};
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::RwLock;

/* =========================================================
 * Static cache
 * ========================================================= */

static ACTIVE_SUBSCRIBED_INDICATORS: Lazy<Arc<RwLock<CacheSubscribedIndicators>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheSubscribedIndicators::new())));

impl<R> SubscribedIndicators<R>
where
    R: Send + Sync,
{
    pub async fn state(&self) -> LifecycleState {
        let cache = ACTIVE_SUBSCRIBED_INDICATORS.read().await;
        cache.status
    }

    pub async fn get_all(&self) -> CacheSubscribedIndicators {
        let cache = ACTIVE_SUBSCRIBED_INDICATORS.read().await;
        cache.clone()
    }

    pub async fn get(&self, key: &str) -> Option<i32> {
        let cache = ACTIVE_SUBSCRIBED_INDICATORS.read().await;
        let env_cache = cache.get(key)?;
        Some(*env_cache)
    }

    pub async fn set_all(&self, values: Vec<String>) -> Result<(), String> {
        let mut cache = ACTIVE_SUBSCRIBED_INDICATORS.write().await;

        if !cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        for symbol in values {
            let entry = cache.get_or_create(symbol);
            *entry += 1;
        }

        cache.last_update_date = Local::now().naive_local();
        Ok(())
    }

    pub async fn upsert(&self, key: String) -> Result<(), String> {
        let mut cache = ACTIVE_SUBSCRIBED_INDICATORS.write().await;

        if !cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let entry = cache.models.entry(key).or_insert_with(|| 0);
        *entry += 1;

        cache.last_update_date = Local::now().naive_local();
        Ok(())
    }

    pub async fn remove(&self, key: String) {
        let mut cache = ACTIVE_SUBSCRIBED_INDICATORS.write().await;

        let entry = cache.models.entry(key.clone()).or_insert_with(|| 0);
        *entry -= 1;

        cache.last_update_date = Local::now().naive_local();
    }

    pub async fn remove_entries(&self) -> Vec<String> {
        let mut cache = ACTIVE_SUBSCRIBED_INDICATORS.write().await;

        let mut removed = Vec::new();

        cache.models.retain(|key, value| {
            if *value <= 0 {
                removed.push(key.clone());
                false
            } else {
                true
            }
        });

        removed
    }

    pub async fn reset(&self) -> Result<(), String> {
        let mut cache = ACTIVE_SUBSCRIBED_INDICATORS.write().await;
        *cache = CacheSubscribedIndicators::new();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::handler::SubscribedIndicators;

    fn mock_values() -> Vec<String> {
        vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()]
    }

    fn service() -> SubscribedIndicators<()> {
        SubscribedIndicators::blank()
    }

    #[tokio::test]
    async fn subscribed_indicators_cache_should_behave_correctly() {
        let service = service();

        /* =========================
         * SET / GET ALL
         * ========================= */

        service.set_all(mock_values()).await.unwrap();

        let cache = service.get_all().await;
        assert_eq!(cache.models.len(), 2);

        assert_eq!(cache.models.get("BTCUSDT"), Some(&1));
        assert_eq!(cache.models.get("ETHUSDT"), Some(&1));

        /* =========================
         * GET (single key)
         * ========================= */

        let btc = service.get("BTCUSDT").await.unwrap();
        assert_eq!(btc, 1);

        /* =========================
         * UPSERT
         * ========================= */

        service.upsert("BTCUSDT".to_string()).await.unwrap();
        service.upsert("BTCUSDT".to_string()).await.unwrap();

        let btc = service.get("BTCUSDT").await.unwrap();
        assert_eq!(btc, 3);
    }
}
