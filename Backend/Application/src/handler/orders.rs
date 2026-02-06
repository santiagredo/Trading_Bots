use models::{
    entities::{actions, orders::Model, pairs, strategies},
    structs::{OrderRequest, Ticker},
};

#[derive(Debug, Clone)]
pub struct Orders<R> {
    pub repo: R,
}

impl<R> Orders<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl Orders<()> {
    pub fn blank() -> Orders<()> {
        Self { repo: () }
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

    pub fn from_model(request: &mut OrderRequest, order: Model) {
        request.id = Some(order.id);
        request.status_id = Some(order.status_id);
        request.creation_date = Some(order.creation_date);
        request.update_date = Some(order.update_date);
        request.is_sell = Some(order.is_sell);
        request.strategy_id = Some(order.strategy_id);
        request.base_asset_id = Some(order.base_asset_id);
        request.base_asset_amount = Some(order.base_asset_amount);
        request.quote_asset_id = Some(order.quote_asset_id);
        request.quote_asset_amount = Some(order.quote_asset_amount);
        request.price_entry = Some(order.price_entry);
        request.price_target = Some(order.price_target);
        request.price_abort = Some(order.price_abort);
    }

    pub fn from_strategy(request: &mut OrderRequest, strategy: &strategies::Model) {
        request.status_id = Some(1);
        request.strategy_id = Some(strategy.id);
    }

    pub fn from_action(request: &mut OrderRequest, action: &actions::Model) {
        request.is_sell = Some(action.is_sell);
        let action_value = action.value;
        request.base_asset_amount = Some(action_value.clone());
    }

    pub fn from_pair(request: &mut OrderRequest, pair: &pairs::Model) {
        request.base_asset_id = Some(pair.base_asset_id);
        request.quote_asset_id = Some(pair.quote_asset_id);
    }

    pub fn from_ticker(request: &mut OrderRequest, ticker: &Ticker) {
        request.price_entry = Some(ticker.last_price);
        request.price_target = Some(ticker.last_price);

        let quote_asset_amount = &request.base_asset_amount.unwrap_or_default() * ticker.last_price;
        request.quote_asset_amount = Some(quote_asset_amount);
    }

    pub fn from_status(request: &mut OrderRequest, status_id: i32) {
        request.status_id = Some(status_id);
    }
}

#[test]
fn builds_order_request_from_strategy_action_pair_and_ticker() {
    use sea_orm::prelude::Decimal;

    let mut request = OrderRequest::default();

    let strategy = strategies::Model {
        id: 1,
        ..Default::default()
    };

    let action = actions::Model {
        is_sell: false,
        is_quote_asset: true,
        is_percentage: false,
        value: Decimal::from_f32_retain(0.001).unwrap_or_default(),
        ..Default::default()
    };

    let pair = pairs::Model {
        base_asset_id: 1,
        quote_asset_id: 2,
        ..Default::default()
    };

    let ticker = Ticker {
        last_price: Decimal::from(10000),
        ..Default::default()
    };

    Orders::from_strategy(&mut request, &strategy);
    Orders::from_action(&mut request, &action);
    Orders::from_pair(&mut request, &pair);
    Orders::from_ticker(&mut request, &ticker);

    assert_eq!(request.strategy_id, Some(1));
    assert_eq!(request.is_sell, Some(false));
    assert_eq!(request.base_asset_amount, Some(action.value));
    assert_eq!(
        request.quote_asset_amount.unwrap_or_default(),
        request.base_asset_amount.unwrap_or_default() * ticker.last_price
    );
}
