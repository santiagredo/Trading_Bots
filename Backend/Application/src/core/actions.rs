use crate::{
    handler::{Actions, Strategies},
    logic,
    utils::{handle_user_err, AnyRepo, EntityCache, RepoFactory, Repository, Response, Select},
};
use models::{
    entities::{actions::Model, assets, pairs},
    enums::LifecycleState,
    structs::{ActionRequest, Environments, QueryOptions, StrategyRequest, Ticker},
};

/* ======================================================
 * CRUD / DB
 * ======================================================
 */

impl<R> Actions<R>
where
    R: Repository<ActionRequest, Model>,
{
    pub async fn insert(&self, req: ActionRequest) -> Result<Model, Response> {
        logic::actions::validate_insert(&req).map_err(handle_user_err)?;
        self.repo.insert(req).await
    }

    pub async fn select(&self, req: ActionRequest) -> Result<Option<Model>, Response> {
        self.repo.select(req).await
    }

    pub async fn select_many(
        &self,
        req: ActionRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        self.repo.select_many(req, query).await
    }

    pub async fn update(&self, req: ActionRequest) -> Result<Model, Response> {
        logic::actions::validate_update(&req).map_err(handle_user_err)?;
        self.repo.update(req).await
    }

    pub async fn delete(&self, req: ActionRequest) -> Result<u64, Response> {
        logic::actions::validate_delete(&req).map_err(handle_user_err)?;
        self.repo.delete(req).await
    }
}

/* ======================================================
 * START / LOAD CACHE
 * ======================================================
 */

impl Actions<AnyRepo<ActionRequest, Model>>
where
    Self: EntityCache<Environments>,
{
    pub async fn start(&self, factory: RepoFactory, env: Environments) -> Result<(), Response> {
        // STARTING
        self.set_state(env, LifecycleState::Starting)
            .await
            .map_err(Response::server_error)?;

        // Load active strategies
        let mut strategies_req = StrategyRequest::default();
        strategies_req.is_active = Some(true);

        let strategies_repo =
            factory.repo::<StrategyRequest, models::entities::strategies::Model>();

        let mut strategies_req = StrategyRequest::default();
        strategies_req.is_active = Some(true);

        let active_strategies = Strategies::new(strategies_repo)
            .select_many(strategies_req, None)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|s| s.id)
            .collect::<Vec<_>>();

        // Load active actions from DB
        let mut req = ActionRequest::default();
        req.is_active = Some(true);

        let actions = match self.repo.select_many(req, None).await {
            Ok(v) => v
                .into_iter()
                .filter(|a| active_strategies.contains(&a.strategy_id))
                .collect::<Vec<_>>(),
            Err(err) => {
                let _ = self.reset(env).await;
                return Err(err);
            }
        };

        // RUNNING
        self.set_state(env, LifecycleState::Running)
            .await
            .map_err(Response::server_error)?;

        if let Err(err) = self.set_all(env, actions).await {
            let _ = self.reset(env).await;
            return Err(Response::server_error(err));
        }

        Ok(())
    }
}

/* ======================================================
 * STOP
 * ======================================================
 */

impl<R> Actions<R>
where
    Self: EntityCache<Environments>,
{
    pub async fn stop(&self, env: Environments) -> Result<(), Response> {
        self.set_state(env, LifecycleState::Stopping)
            .await
            .map_err(Response::server_error)?;

        self.remove_all(env).await.map_err(Response::server_error)?;

        self.set_state(env, LifecycleState::Off)
            .await
            .map_err(Response::server_error)?;

        Ok(())
    }
}

/* ======================================================
 * MISC
 * ======================================================
 */

impl<R> Actions<R> {
    pub fn evaluate_action(
        &self,
        action: Model,
        pair: &pairs::Model,
        ticker: &Ticker,
        base_asset: &assets::Model,
        quote_asset: &assets::Model,
    ) -> Result<Model, String> {
        logic::actions::evaluate_action(action, pair, ticker, base_asset, quote_asset)
    }
}

/* ======================================================
 * TESTS
 * ======================================================
 */

#[cfg(test)]
mod core_tests {
    use models::structs::ActionRequest;
    use sea_orm::prelude::Decimal;

    use crate::{handler::Actions, utils::MockRepo};

    #[tokio::test]
    async fn insert_action_ok() {
        let repo = MockRepo::new();
        let service = Actions::new(repo);

        let req = ActionRequest {
            id: Some(1),
            strategy_id: Some(10),
            is_active: Some(true),
            is_sell: Some(false),
            is_quote_asset: Some(false),
            is_percentage: Some(false),
            pair_id: Some(1),
            value: Some(Decimal::ONE),
            ..Default::default()
        };

        let result = service.insert(req).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, 1);
    }
}
