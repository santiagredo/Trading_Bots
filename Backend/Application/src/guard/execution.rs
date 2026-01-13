use std::time::Instant;

use chrono::Local;
use models::structs::{Environments, StrategyRequest};

use crate::handler::{Metrics, Strategies};

#[derive(Debug, Clone)]
pub struct StrategiesExecutionGuard {
    pub environment: Environments,
    pub start: Instant,
    pub success: Option<bool>,
    pub model: StrategyRequest,
}

impl StrategiesExecutionGuard {
    pub fn new(environment: Environments, model: StrategyRequest) -> StrategiesExecutionGuard {
        StrategiesExecutionGuard {
            environment,
            start: Instant::now(),
            success: None,
            model,
        }
    }

    pub fn ok(&mut self) {
        self.success = Some(true);
    }

    pub fn err(&mut self) {
        self.success = Some(false);
    }

    pub fn none(&mut self) {
        self.success = None;
    }

    pub async fn lock(&self) -> Result<(), String> {
        let strategy = self.model.clone();

        Strategies::new(strategy)
            .with_env(self.environment)
            .set_strategy_posting(true)
            .await
    }
}

impl Drop for StrategiesExecutionGuard {
    fn drop(&mut self) {
        let environment = self.environment;
        let elapsed = self.start.elapsed();
        let mut strategy_request = self.model.clone();
        let now = Local::now().naive_local();

        let success = match self.success {
            // save failure for execution metrics
            None => false,

            // save execution error and last error date
            Some(false) => {
                strategy_request.error_last_date = Some(now);

                false
            }

            // save success and last execution date
            Some(true) => {
                strategy_request.last_execution = Some(now);

                true
            }
        };

        tokio::spawn(async move {
            Metrics::set_execution_metrics(environment, elapsed, success).await;

            // update posting in cache
            let _ = Strategies::new(strategy_request.clone())
                .with_env(environment)
                .set_strategy_posting(false)
                .await;

            // update last exec and error date in cache
            let _ = Strategies::new(strategy_request.clone())
                .with_env(environment)
                .upsert_strategy()
                .await;

            // update last exec and error date in db
            if let Err(err) = Strategies::new(strategy_request)
                .with_env(environment)
                .update_strategy()
                .await
            {
                dbg!(eprintln!("{}", err.message));
                return;
            };
        });
    }
}
