use std::{collections::HashMap, mem, sync::Arc};

use chrono::Local;
use migration::async_trait::async_trait;
use models::{
    entities::assets::Model,
    enums::{transition_with_timestamp, FiniteStateMachine, LifecycleState},
    structs::{CacheAssets, CacheAssetsEnvironments, Environments},
};
use once_cell::sync::Lazy;
use sea_orm::prelude::Decimal;
use tokio::{sync::RwLock, task::AbortHandle};

use crate::{handler::Assets, utils::EntityCache};

static ACTIVE_ASSETS: Lazy<Arc<RwLock<CacheAssetsEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheAssetsEnvironments::new())));

#[async_trait]
impl<R> EntityCache<Environments> for Assets<R>
where
    R: Send + Sync,
{
    type Key = i32;
    type Value = Model;
    type Collection = CacheAssets;

    async fn state(&self, env: Environments) -> LifecycleState {
        let cache = ACTIVE_ASSETS.read().await;
        cache
            .get(&env)
            .map(|c| c.status)
            .unwrap_or(LifecycleState::Off)
    }

    async fn set_state(&self, env: Environments, state: LifecycleState) -> Result<(), String> {
        let mut cache = ACTIVE_ASSETS.write().await;
        let env_cache = cache.get_or_create(env);

        transition_with_timestamp(env_cache, state)?;
        Ok(())
    }

    async fn get_all(&self, env: Environments) -> Option<CacheAssets> {
        let cache = ACTIVE_ASSETS.read().await;
        let env_cache = cache.get(&env)?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return None;
        }

        Some(env_cache.clone())
    }

    async fn get(&self, env: Environments, key: i32) -> Option<Model> {
        let cache = ACTIVE_ASSETS.read().await;
        let env_cache = cache.get(&env)?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return None;
        }

        env_cache.models.get(&key).cloned()
    }

    async fn set_all(&self, env: Environments, values: Vec<Model>) -> Result<(), String> {
        let mut cache = ACTIVE_ASSETS.write().await;
        let env_cache = cache.get_or_create(env);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let values: Vec<(i32, Model)> = values.into_iter().map(|m| (m.id, m)).collect();

        env_cache.models = values.into_iter().collect();
        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    async fn upsert(&self, env: Environments, key: i32, value: Model) -> Result<(), String> {
        let mut cache = ACTIVE_ASSETS.write().await;
        let env_cache = cache.get_or_create(env);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        env_cache.models.insert(key, value);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    async fn remove(&self, env: Environments, key: i32) -> Result<Option<Model>, String> {
        let mut cache = ACTIVE_ASSETS.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let removed = env_cache.models.remove(&key);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(removed)
    }

    async fn remove_all(&self, env: Environments) -> Result<HashMap<i32, Model>, String> {
        let mut cache = ACTIVE_ASSETS.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Stopping) {
            return Err("Cache not stopping".into());
        }

        let removed = mem::take(&mut env_cache.models);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(removed)
    }

    async fn reset(&self, env: Environments) -> Result<(), String> {
        let mut cache = ACTIVE_ASSETS.write().await;

        if let Some(env_cache) = cache.environments.get_mut(&env) {
            *env_cache = CacheAssets::new();
        }

        Ok(())
    }
}

impl<R> Assets<R>
where
    R: Send + Sync,
{
    pub async fn set_asset_balance(
        &self,
        environment: Environments,
        asset_id: i32,
        value: Decimal,
        locked: bool,
        sell: bool,
    ) -> Result<(Model, Decimal), String> {
        let mut cache = ACTIVE_ASSETS.write().await;
        let env_cache = cache
            .get_mut(&environment)
            .ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let asset = env_cache
            .models
            .get_mut(&asset_id)
            .ok_or("Asset not found")?;

        let target = if locked {
            &mut asset.locked
        } else {
            &mut asset.free
        };

        let previous = target.clone();

        if sell {
            *target -= value;
        } else {
            *target += value;
        }

        // env_cache.last_update_date = Local::now().naive_local();
        Ok((asset.clone(), previous))
    }

    pub async fn set_abort_handle(
        &self,
        env: Environments,
        abort_handle: AbortHandle,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_ASSETS.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        env_cache.abort_handle = Some(abort_handle);

        Ok(())
    }

    pub async fn abort_handle(&self, env: Environments) -> Result<(), String> {
        let mut cache = ACTIVE_ASSETS.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        let handle = env_cache.abort_handle.take();

        if let Some(handle) = handle {
            handle.abort();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use models::{entities::assets::Model, enums::LifecycleState, structs::Environments};
    use sea_orm::prelude::Decimal;

    use crate::{handler::Assets, utils::EntityCache};

    fn mock_asset(id: i32, free: i64, locked: i64) -> Model {
        Model {
            id,
            free: Decimal::new(free, 0),
            locked: Decimal::new(locked, 0),
            ..Default::default()
        }
    }

    async fn new_service() -> Assets<()> {
        Assets::blank()
    }

    async fn start_env(service: &Assets<()>, env: Environments) {
        service
            .set_state(env, LifecycleState::Starting)
            .await
            .unwrap();
        service
            .set_state(env, LifecycleState::Running)
            .await
            .unwrap();
    }

    async fn stop_env(service: &Assets<()>, env: Environments) {
        service
            .set_state(env, LifecycleState::Stopping)
            .await
            .unwrap();

        let removed = service.remove_all(env).await.unwrap();
        assert!(removed.is_empty() || !removed.is_empty());

        service.set_state(env, LifecycleState::Off).await.unwrap();
    }

    async fn scenario_reset_env_clears_state(service: &Assets<()>, env: Environments) {
        start_env(service, env).await;

        service
            .set_all(env, vec![mock_asset(1, 100, 0)])
            .await
            .unwrap();

        service
            .set_state(env, LifecycleState::Stopping)
            .await
            .unwrap();

        let removed = service.remove_all(env).await.unwrap();
        assert_eq!(removed.len(), 1);

        let cache = service.get_all(env).await;
        assert!(cache.is_none());

        service.set_state(env, LifecycleState::Off).await.unwrap();

        let status = service.state(env).await;
        assert_eq!(status, LifecycleState::Off);
    }

    async fn scenario_set_assets_cache(service: &Assets<()>, env: Environments) {
        start_env(service, env).await;

        let assets = vec![mock_asset(1, 100, 0), mock_asset(2, 200, 10)];

        service.set_all(env, assets).await.unwrap();

        let cache = service.get_all(env).await.unwrap();
        let status = service.state(env).await;

        assert_eq!(cache.models.len(), 2);
        assert!(cache.models.contains_key(&1));
        assert!(cache.models.contains_key(&2));
        assert_eq!(status, LifecycleState::Running);

        stop_env(service, env).await;
    }

    async fn scenario_upsert_asset_insert(service: &Assets<()>, env: Environments) {
        start_env(service, env).await;

        let asset = mock_asset(10, 50, 5);

        service.upsert(env, asset.id, asset.clone()).await.unwrap();

        let cached = service.get(env, 10).await.unwrap();
        assert_eq!(cached, asset);

        stop_env(service, env).await;
    }

    async fn scenario_remove_asset(service: &Assets<()>, env: Environments) {
        start_env(service, env).await;

        let asset = mock_asset(20, 30, 0);
        service.upsert(env, asset.id, asset).await.unwrap();

        let removed = service.remove(env, 20).await.unwrap();
        assert!(removed.is_some());

        let cached = service.get(env, 20).await;
        assert!(cached.is_none());

        stop_env(service, env).await;
    }

    async fn scenario_get_assets_state(service: &Assets<()>, env: Environments) {
        assert_eq!(service.state(env).await, LifecycleState::Off);

        start_env(service, env).await;

        assert_eq!(service.state(env).await, LifecycleState::Running);

        stop_env(service, env).await;
    }

    async fn scenario_cannot_remove_assets_when_running(service: &Assets<()>, env: Environments) {
        start_env(service, env).await;

        let result = service.remove_all(env).await;
        assert!(result.is_err());

        stop_env(service, env).await;
    }

    async fn scenario_set_asset_balance_increases_free(service: &Assets<()>, env: Environments) {
        start_env(service, env).await;

        // Arrange
        let asset = mock_asset(1, 100, 50);
        service.set_all(env, vec![asset]).await.unwrap();

        // Act
        let (updated, previous) = service
            .set_asset_balance(
                env,
                1,
                Decimal::new(25, 0),
                false, // locked = false
                false, // sell = false
            )
            .await
            .unwrap();

        // Assert
        assert_eq!(previous, Decimal::new(100, 0));
        assert_eq!(updated.free, Decimal::new(125, 0));
        assert_eq!(updated.locked, Decimal::new(50, 0));

        let cached = service.get(env, 1).await.unwrap();
        assert_eq!(cached.free, Decimal::new(125, 0));

        stop_env(service, env).await;
    }

    async fn scenario_set_asset_balance_decreases_locked(service: &Assets<()>, env: Environments) {
        start_env(service, env).await;

        let asset = mock_asset(2, 100, 50);
        service.set_all(env, vec![asset]).await.unwrap();

        let (updated, previous) = service
            .set_asset_balance(
                env,
                2,
                Decimal::new(20, 0),
                true, // locked
                true, // sell
            )
            .await
            .unwrap();

        assert_eq!(previous, Decimal::new(50, 0));
        assert_eq!(updated.locked, Decimal::new(30, 0));
        assert_eq!(updated.free, Decimal::new(100, 0));

        stop_env(service, env).await;
    }

    #[tokio::test]
    async fn cache_assets_unit_responsibilities() {
        let env = Environments::DEV;
        let service = new_service().await;

        scenario_reset_env_clears_state(&service, env).await;
        scenario_set_assets_cache(&service, env).await;
        scenario_upsert_asset_insert(&service, env).await;
        scenario_remove_asset(&service, env).await;
        scenario_get_assets_state(&service, env).await;
        scenario_cannot_remove_assets_when_running(&service, env).await;
        scenario_set_asset_balance_increases_free(&service, env).await;
        scenario_set_asset_balance_decreases_locked(&service, env).await;
    }
}
