use std::marker::PhantomData;

use models::{
    entities::{
        actions,
        orders::{self, Model},
        pairs, strategies,
    },
    structs::{request::OrderRequest, Environments, QueryOptions, Ticker},
};

use crate::utils::{Response, Types};

#[derive(Debug, Default)]
pub struct Orders<Phase = Types> {
    pub phase: PhantomData<Phase>,
    pub environment: Environments,
    pub model: OrderRequest,
}

impl<Phase> Orders<Phase> {
    pub fn next_phase<Next>(self) -> Orders<Next> {
        Orders {
            phase: PhantomData::<Next>,
            environment: self.environment,
            model: self.model,
        }
    }
}

impl Orders {
    pub fn new(model: OrderRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            environment: Environments::DEV,
            model,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            model: OrderRequest {
                ..Default::default()
            },
            environment: Environments::DEV,
        }
    }

    pub fn with_env(self, environment: Environments) -> Self {
        Self {
            phase: self.phase,
            environment,
            model: self.model,
        }
    }

    pub fn from_request(order_request: OrderRequest) -> Model {
        Model {
            id: order_request.id.unwrap_or_default(),
            status_id: order_request.status_id.unwrap_or_default(),
            creation_date: order_request.creation_date.unwrap_or_default(),
            update_date: order_request.update_date.unwrap_or_default(),
            is_sell: order_request.is_sell.unwrap_or(false),
            strategy_id: order_request.strategy_id.unwrap_or_default(),
            base_asset_id: order_request.base_asset_id.unwrap_or_default(),
            base_asset_amount: order_request.base_asset_amount.unwrap_or_default(),
            quote_asset_id: order_request.quote_asset_id.unwrap_or_default(),
            quote_asset_amount: order_request.quote_asset_amount.unwrap_or_default(),
            price_entry: order_request.price_entry.unwrap_or_default(),
            price_target: order_request.price_target.unwrap_or_default(),
            price_abort: order_request.price_abort.unwrap_or_default(),
        }
    }

    pub fn from_model(mut self, order: &orders::Model) -> Self {
        let model = OrderRequest {
            id: Some(order.id),
            status_id: Some(order.status_id),
            creation_date: Some(order.creation_date),
            update_date: Some(order.update_date),
            is_sell: Some(order.is_sell),
            strategy_id: Some(order.strategy_id),
            base_asset_id: Some(order.base_asset_id),
            base_asset_amount: Some(order.base_asset_amount),
            quote_asset_id: Some(order.quote_asset_id),
            quote_asset_amount: Some(order.quote_asset_amount),
            price_entry: Some(order.price_entry),
            price_target: Some(order.price_target),
            price_abort: Some(order.price_abort),
        };

        self.model = model;
        self
    }

    pub fn from_strategy(mut self, strategy: &strategies::Model) -> Self {
        self.model.status_id = Some(1);
        self.model.strategy_id = Some(strategy.id);

        self
    }

    pub fn from_action(mut self, action: &actions::Model) -> Self {
        self.model.is_sell = Some(action.is_sell);

        let action_value = action.value;

        self.model.base_asset_amount = Some(action_value.clone());

        self
    }

    pub fn from_pair(mut self, pair: &pairs::Model) -> Self {
        self.model.base_asset_id = Some(pair.base_asset_id);
        self.model.quote_asset_id = Some(pair.quote_asset_id);

        self
    }

    pub fn from_ticker(mut self, ticker: &Ticker) -> Self {
        self.model.price_entry = Some(ticker.last_price);
        self.model.price_target = Some(ticker.last_price);

        self
    }

    pub fn from_status(mut self, status_id: i32) -> Self {
        self.model.status_id = Some(status_id);

        self
    }

    pub async fn insert_order(self) -> Result<Model, Response> {
        self.next_phase().insert_order_core().await
    }

    pub async fn select_order(self) -> Result<Model, Response> {
        self.next_phase().select_order_core().await
    }

    pub async fn select_orders(self, query: Option<QueryOptions>) -> Result<Vec<Model>, Response> {
        self.next_phase().select_orders_core(query).await
    }

    pub async fn update_order(self) -> Result<Model, Response> {
        self.next_phase().update_order_core().await
    }
}
