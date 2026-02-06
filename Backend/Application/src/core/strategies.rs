use crate::{
    guard::StrategiesExecutionGuard,
    handler::{
        Binance, Orders, Senders, Strategies, StrategiesOverview, SubscribedIndicators, Tickers,
    },
    static_strings::{ORDERS_ENDPOINT, ORDERS_TEST_ENDPOINT},
    utils::{EntityCache, RepoFactory, Repository, Response},
};

use models::{
    entities::strategies::Model,
    enums::LifecycleState,
    structs::{CacheStrategy, Environments, QueryOptions, StrategyOverview, StrategyRequest},
};
use tokio_util::sync::CancellationToken;

impl<R> Strategies<R>
where
    R: Repository<StrategyRequest, Model>,
{
    pub async fn insert(&self, req: StrategyRequest) -> Result<Model, Response> {
        self.repo.insert(req).await
    }

    pub async fn select(&self, req: StrategyRequest) -> Result<Option<Model>, Response> {
        self.repo.select(req).await
    }

    pub async fn select_many(
        &self,
        req: StrategyRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        self.repo.select_many(req, query).await
    }

    pub async fn update(&self, req: StrategyRequest) -> Result<Model, Response> {
        self.repo.update(req).await
    }

    pub async fn delete(&self, req: StrategyRequest) -> Result<u64, Response> {
        self.repo.delete(req).await
    }
}

impl<R> Strategies<R>
where
    R: Repository<StrategyRequest, Model> + Send + Sync + 'static + Clone,
    // Self: EntityCache<Environments, i32, Model, CacheStrategies> + Send + Sync + 'static,
{
    pub async fn start_strategies(
        &self,
        factory: RepoFactory,
        env: Environments,
        token: &CancellationToken,
        senders: Senders,
    ) -> Result<(), Response> {
        /* ===========================
         * STARTING
         * ===========================
         */

        self.set_state(env, LifecycleState::Starting)
            .await
            .map_err(Response::server_error)?;

        /* ===========================
         * LOAD FROM DB
         * ===========================
         */

        let mut req = StrategyRequest::default();
        req.is_active = Some(true);

        let models = match self.repo.select_many(req, None).await {
            Ok(m) => m.into_iter().filter(|s| s.is_active).collect::<Vec<_>>(),
            Err(err) => {
                let _ = self.reset(env).await;
                return Err(err);
            }
        };

        let values: Vec<CacheStrategy> = models
            .into_iter()
            .map(|m| CacheStrategy {
                model: m,
                ..Default::default()
            })
            .collect();

        /* ===========================
         * RUNNING
         * ===========================
         */

        self.set_state(env, LifecycleState::Running)
            .await
            .map_err(Response::server_error)?;

        self.set_all(env, values).await.map_err(|e| {
            let _ = self.reset(env);
            Response::server_error(e)
        })?;

        /* ===========================
         * RUNTIME
         * ===========================
         */

        let mut tickers_receiver = senders.event_sender.subscribe();

        let cancellation_token = token.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        break;
                    }

                    msg = tickers_receiver.recv() => {
                        let Ok(ticker_evt) = msg else {
                            continue;
                        };

                        let Some(ticker) = Tickers::get_ticker(ticker_evt.symbol).await else {
                            continue;
                        };

                        let subscribed = SubscribedIndicators::blank().get_all(env)
                            .await
                            .and_then(|m| m.models.get(&ticker.symbol).cloned())
                            .unwrap_or_default();

                        for strategy_id in subscribed {
                            if let Some(overview) = StrategiesOverview::get_strategy_overview(
                                env,
                                &strategy_id,
                                ticker.symbol.clone(),
                            )
                            .await
                            {
                                Self::evaluate_strategy_runtime(factory.clone(), env, overview).await;
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    /* ===========================
     * RUNTIME EVALUATION
     * ===========================
     */

    async fn evaluate_strategy_runtime(
        factory: RepoFactory,
        environment: Environments,
        strategy_overview: StrategyOverview,
    ) {
        // READY -> RUNNING
        let mut guard = match StrategiesExecutionGuard::new(
            factory.clone(),
            environment,
            Strategies::into_request(strategy_overview.strategy.clone()),
        )
        .await
        {
            Err(err) => {
                dbg!(err);
                return;
            }
            Ok(val) => val,
        };

        let mut order = match StrategiesOverview::evaluate_strategy_overview(&strategy_overview) {
            Ok(o) => o,
            Err(err) => {
                let _ = guard.err(err).await;
                return;
            }
        };

        // RUNNING -> TRADING
        if let Err(err) = guard.lock().await {
            let _ = guard.err(err).await;
            return;
        }

        if strategy_overview.strategy.can_trade {
            let (api_key, secret_pass) = match Binance::blank()
                .resolve_binance_credentials(factory.clone(), environment)
                .await
            {
                Ok(val) => val,
                Err(err) => {
                    let _ = guard.ext_err(err.message).await;
                    return;
                }
            };

            let endpoint = match environment {
                Environments::DEV => ORDERS_TEST_ENDPOINT,
                Environments::PROD => ORDERS_ENDPOINT,
            };

            if let Err(err) = Binance::blank()
                .post_new_order(
                    factory.clone(),
                    strategy_overview.ticker.symbol.clone(),
                    &mut order,
                    api_key,
                    secret_pass,
                    endpoint,
                )
                .await
            {
                let _ = guard.ext_err(err.message).await;
                return;
            }
        }

        // TRADING -> SAVING
        let _ = guard.saving().await;

        let repo = factory.repo();

        match Orders::new(repo).insert(order).await {
            Err(err) => {
                let _ = guard.err(err.message).await;
                return;
            }
            Ok(order) => {
                let senders = Senders::get_senders().await;
                let _ = senders.order_sender.send((environment, order));
            }
        };

        // SAVING -> READY
        guard.ok();
    }
}

impl<R> Strategies<R>
where
    R: Send + Sync,
{
    /* ===========================
     * STOP ACTIVE STRATEGIES
     * ===========================
     */

    pub async fn stop_strategies(&self, environment: Environments) -> Result<(), Response> {
        // STOPPING
        self.set_state(environment, LifecycleState::Stopping)
            .await
            .map_err(|e| Response {
                code: 500,
                message: e,
            })?;

        // Remove cache
        self.remove_all(environment).await.map_err(|e| Response {
            code: 500,
            message: e,
        })?;

        // OFF
        self.set_state(environment, LifecycleState::Off)
            .await
            .map_err(|e| Response {
                code: 500,
                message: e,
            })?;

        Ok(())
    }

    /* ===========================
     * RESET ACTIVE STRATEGIES
     * ===========================
     */

    pub async fn reset_strategies(&self, environment: Environments) -> Result<(), Response> {
        self.reset(environment).await.map_err(|e| Response {
            code: 500,
            message: e,
        })
    }
}
