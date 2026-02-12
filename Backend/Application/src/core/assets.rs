use crate::{
    handler::{Assets, Ledgers, Senders},
    logic,
    utils::{handle_user_err, EntityCache, RepoFactory, Repository, Response},
};
use models::{
    entities::{assets::Model, ledgers},
    enums::LifecycleState,
    structs::{AssetRequest, Environments, LedgerRequest, QueryOptions},
};
use sea_orm::prelude::Decimal;

/* ======================================================
 * CRUD / DB
 * ======================================================
 */

impl<R> Assets<R>
where
    R: Repository<AssetRequest, Model>,
{
    pub async fn insert(&self, req: AssetRequest) -> Result<Model, Response> {
        logic::assets::validate_insert(&req).map_err(handle_user_err)?;
        self.repo.insert(req).await
    }

    pub async fn select(&self, req: AssetRequest) -> Result<Option<Model>, Response> {
        self.repo.select(req).await
    }

    pub async fn select_many(
        &self,
        req: AssetRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        self.repo.select_many(req, query).await
    }

    pub async fn update(&self, req: AssetRequest) -> Result<Model, Response> {
        logic::assets::validate_update(&req).map_err(handle_user_err)?;
        self.repo.update(req).await
    }

    pub async fn delete(&self, req: AssetRequest) -> Result<u64, Response> {
        logic::assets::validate_delete(&req).map_err(handle_user_err)?;
        self.repo.delete(req).await
    }
}

/* ======================================================
 * START / LOAD CACHE
 * ======================================================
 */

impl<R> Assets<R>
where
    R: Repository<AssetRequest, Model> + Clone + Send + Sync + 'static,
    Self: EntityCache<Environments, Value = Model>,
{
    pub async fn start(
        &self,
        factory: RepoFactory,
        env: Environments,
        senders: Senders,
    ) -> Result<(), Response> {
        // STARTING
        self.set_state(env, LifecycleState::Starting)
            .await
            .map_err(Response::server_error)?;

        // Load from DB
        let assets = match self.repo.select_many(AssetRequest::default(), None).await {
            Ok(v) => v,
            Err(err) => {
                let _ = self.reset(env).await;
                return Err(err);
            }
        };

        // RUNNING
        self.set_state(env, LifecycleState::Running)
            .await
            .map_err(Response::server_error)?;

        if let Err(err) = self.set_all(env, assets).await {
            let _ = self.reset(env).await;
            return Err(Response::server_error(err));
        }

        // Runtime
        let mut orders_receiver = senders.order_sender.subscribe();
        let repo = self.repo.clone();

        let abort_handle = tokio::spawn(async move {
            // Create a blank Assets instance without repo for cache access
            let service = Assets::new(repo);

            let ledgers_repo = factory.repo::<LedgerRequest, ledgers::Model>();

            while let Ok((environment, order)) = orders_receiver.recv().await {
                // BASE ASSET
                let Ok((base_model, base_prev)) = service
                    .set_asset_balance(
                        environment,
                        order.base_asset_id,
                        order.base_asset_amount,
                        false,
                        order.is_sell,
                    )
                    .await
                else {
                    continue;
                };

                let base_req = AssetRequest::from_model(&base_model);
                let base_ledger = LedgerRequest::from_asset(&base_model)
                    .from_order(&order)
                    .update_values(
                        false,
                        order.base_asset_amount,
                        base_prev,
                        base_req.free.unwrap_or_default(),
                    );

                if service.update(base_req).await.is_err() {
                    continue;
                }

                let _ = Ledgers::new(ledgers_repo.clone()).insert(base_ledger).await;

                // QUOTE ASSET
                let Ok((quote_model, quote_prev)) = service
                    .set_asset_balance(
                        environment,
                        order.quote_asset_id,
                        order.quote_asset_amount,
                        false,
                        !order.is_sell,
                    )
                    .await
                else {
                    continue;
                };

                let quote_req = AssetRequest::from_model(&quote_model);
                let quote_ledger = LedgerRequest::from_asset(&quote_model)
                    .from_order(&order)
                    .update_values(
                        false,
                        order.quote_asset_amount,
                        quote_prev,
                        quote_req.free.unwrap_or_default(),
                    );

                if service.update(quote_req).await.is_err() {
                    continue;
                }

                let _ = Ledgers::new(ledgers_repo.clone())
                    .insert(quote_ledger)
                    .await;
            }
        })
        .abort_handle();

        self.set_abort_handle(env, abort_handle)
            .await
            .map_err(Response::server_error)?;

        Ok(())
    }
}

/* ======================================================
 * CACHE (READ / WRITE)
 * ======================================================
 */

impl<R> Assets<R>
where
    Self: EntityCache<Environments, Key = i32, Value = Model>,
{
    pub async fn update_asset_balance_cache(
        &self,
        env: Environments,
        asset_id: i32,
        value: Decimal,
        locked: bool,
        sell: bool,
    ) -> Result<(Model, Decimal), Response> {
        let mut asset = self
            .get(env, asset_id)
            .await
            .ok_or(Response::not_found("Asset not found".into()))?;

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

        self.upsert(env, asset_id, asset.clone())
            .await
            .map_err(Response::server_error)?;

        Ok((asset, previous))
    }
}

/* ======================================================
 * STOP / RESET
 * ======================================================
 */

impl<R> Assets<R>
where
    R: Send + Sync,
{
    pub async fn stop(&self, env: Environments) -> Result<(), Response> {
        self.set_state(env, LifecycleState::Stopping)
            .await
            .map_err(Response::server_error)?;

        self.remove_all(env).await.map_err(Response::server_error)?;

        self.abort_handle(env)
            .await
            .map_err(Response::server_error)?;

        self.set_state(env, LifecycleState::Off)
            .await
            .map_err(Response::server_error)?;

        Ok(())
    }
}

/* ======================================================
 * TESTS
 * ======================================================
 */

#[cfg(test)]
mod core_tests {
    use models::structs::AssetRequest;

    use crate::{handler::Assets, utils::MockRepo};

    #[tokio::test]
    async fn insert_asset_ok() {
        let repo = MockRepo::new();
        let service = Assets::new(repo);

        let req = AssetRequest {
            id: Some(1),
            name: Some("BITCOIN".into()),
            ticker: Some("BTC".into()),
            ..Default::default()
        };

        let result = service.insert(req).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, 1);
    }
}
