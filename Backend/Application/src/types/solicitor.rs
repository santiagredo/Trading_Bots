use std::marker::PhantomData;

use models::{
    entities::orders,
    structs::{AccountInformation, CoinPaprikaTicker, ExchangeInformation},
};
use tracing::error_span;

use crate::{
    integration::{
        binance_get_account_information, binance_get_exchange_information, binance_post_new_order,
        coinpaprika_get_tickers,
    },
    utils::Core,
};

pub struct Solicitor<Phase = Core> {
    phase: PhantomData<Phase>,
}

impl Solicitor {
    pub async fn binance_post_new_order(
        symbol: String,
        order: &mut orders::Model,
    ) -> Result<(), ()> {
        binance_post_new_order(symbol, order).await.map_err(|err| {
            error_span!("Error - Binance", error = ?err);
            println!("{err} \n");
        })
    }

    pub async fn binance_get_account_information() -> Result<AccountInformation, ()> {
        binance_get_account_information().await.map_err(|err| {
            error_span!("Error - Binance", error = ?err);
            println!("{err} \n");
        })
    }

    pub async fn binance_get_exchange_information() -> Option<ExchangeInformation> {
        binance_get_exchange_information()
            .await
            .map_err(|err| {
                error_span!("Error - Binance", error = ?err);
                println!("{err} \n");
            })
            .ok()
    }

    pub async fn coinpaprika_get_tickers() -> Result<Vec<CoinPaprikaTicker>, ()> {
        coinpaprika_get_tickers().await.map_err(|err| {
            error_span!("Error - CoinPaprika", error = ?err);
            println!("{err} \n");
        })
    }
}
