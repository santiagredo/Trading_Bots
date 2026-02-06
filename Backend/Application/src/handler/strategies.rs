use models::{entities::strategies::Model, structs::StrategyRequest};

#[derive(Debug, Clone)]
pub struct Strategies<R> {
    pub repo: R,
}

impl<R> Strategies<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl Strategies<()> {
    pub fn blank() -> Strategies<()> {
        Self { repo: () }
    }

    pub fn into_model(strategy: StrategyRequest) -> Model {
        Model {
            id: strategy.id.unwrap_or_default(),
            name: strategy.name.unwrap_or_default(),
            is_active: strategy.is_active.unwrap_or_default(),
            can_trade: strategy.can_trade.unwrap_or_default(),
            description: Some(strategy.description.unwrap_or_default()),
            last_execution: Some(strategy.last_execution.unwrap_or_default()),
            cooldown: Some(strategy.cooldown.unwrap_or_default()),
            error_cooldown: Some(strategy.error_cooldown.unwrap_or_default()),
            error_last_date: Some(strategy.error_last_date.unwrap_or_default()),
            last_update: Some(strategy.last_update.unwrap_or_default()),
        }
    }

    pub fn into_request(model: Model) -> StrategyRequest {
        StrategyRequest {
            id: Some(model.id),
            name: Some(model.name),
            is_active: Some(model.is_active),
            can_trade: Some(model.can_trade),
            description: model.description,
            last_execution: model.last_execution,
            cooldown: model.cooldown,
            error_cooldown: model.error_cooldown,
            error_last_date: model.error_last_date,
            last_update: model.last_update,
        }
    }
}
