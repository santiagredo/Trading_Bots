use std::time::Instant;

use crate::{
    handler::{Metrics, Strategies},
    utils::RepoFactory,
};
use chrono::Local;
use models::{
    entities::strategies::Model,
    enums::TradingState,
    structs::{Environments, StrategyRequest},
};

#[derive(Debug, Clone)]
pub struct StrategiesExecutionGuard
// where
//     R: Repository<StrategyRequest, Model> + Clone + Send + Sync + 'static,
{
    pub factory: RepoFactory,
    pub environment: Environments,
    pub start: Instant,
    pub success: bool,
    pub model: StrategyRequest,
}

impl StrategiesExecutionGuard
// where
//     R: Repository<StrategyRequest, Model> + Clone + Send + Sync + 'static,
{
    pub async fn new(
        factory: RepoFactory,
        environment: Environments,
        model: StrategyRequest,
    ) -> Result<Self, String> {
        let start = Instant::now();

        Strategies::blank()
            .set_strategy_state(
                environment,
                model.id.unwrap_or_default(),
                TradingState::Running,
            )
            .await?;

        Ok(StrategiesExecutionGuard {
            factory,
            environment,
            start,
            success: false,
            model,
        })
    }

    pub fn ok(&mut self) {
        self.success = true;
    }

    pub async fn err(&mut self, error: String) -> Result<(), String> {
        self.success = false;

        Strategies::blank()
            .set_strategy_error(
                self.environment,
                self.model.id.unwrap_or_default(),
                Some(error),
            )
            .await?;

        Strategies::blank()
            .set_strategy_state(
                self.environment,
                self.model.id.unwrap_or_default(),
                TradingState::Ready,
            )
            .await
    }

    // External integration error (exchange, network, etc)
    // Updates error_last_date and persists to DB to enable cooldown logic
    pub async fn ext_err(&mut self, error: String) -> Result<(), String> {
        self.success = false;

        self.model.error_last_date = Some(Local::now().naive_local());

        Strategies::blank()
            .set_strategy_error(
                self.environment,
                self.model.id.unwrap_or_default(),
                Some(error),
            )
            .await?;

        Strategies::blank()
            .set_strategy_model_error(self.environment, self.model.id.unwrap_or_default())
            .await?;

        let strategies_repo = self.factory.clone().repo::<StrategyRequest, Model>();

        Strategies::new(strategies_repo)
            .update(self.model.clone())
            .await
            .map_err(|response| response.message)?;

        Strategies::blank()
            .set_strategy_state(
                self.environment,
                self.model.id.unwrap_or_default(),
                TradingState::Ready,
            )
            .await
    }

    pub async fn saving(&mut self) -> Result<(), String> {
        Strategies::blank()
            .set_strategy_state(
                self.environment,
                self.model.id.unwrap_or_default(),
                TradingState::Saving,
            )
            .await
    }

    pub async fn lock(&self) -> Result<(), String> {
        Strategies::blank()
            .set_strategy_state(
                self.environment,
                self.model.id.unwrap_or_default(),
                TradingState::Trading,
            )
            .await
    }
}

impl Drop for StrategiesExecutionGuard
// where
//     R: Repository<StrategyRequest, Model> + Clone + Send + Sync + 'static,
{
    fn drop(&mut self) {
        let environment = self.environment;
        let elapsed = self.start.elapsed();
        let success = self.success;
        let mut strategy_request = self.model.clone();
        let now = Local::now().naive_local();
        let factory = self.factory.clone();

        tokio::spawn(async move {
            Metrics::blank()
                .set_execution_metrics(environment, elapsed, success)
                .await;

            if success {
                strategy_request.last_execution = Some(now);

                // update last exec
                let strategies_repo = factory.clone().repo::<StrategyRequest, Model>();

                let _ = Strategies::blank()
                    .set_strategy_model_last_exec(
                        environment,
                        strategy_request.id.unwrap_or_default(),
                    )
                    .await;

                // update last exec in db
                if let Err(err) = Strategies::new(strategies_repo)
                    .update(strategy_request.clone())
                    .await
                {
                    dbg!(eprintln!("{}", err.message));
                    return;
                };

                // update trading state in cache
                let _ = Strategies::blank()
                    .set_strategy_state(
                        environment,
                        strategy_request.id.unwrap_or_default(),
                        TradingState::Ready,
                    )
                    .await;
            }
        });
    }
}
