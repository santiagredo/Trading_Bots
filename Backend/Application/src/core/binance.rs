use models::{
    entities::orders,
    structs::{AccountInformation, ExchangeInformation},
};
use tracing::error_span;

use crate::{config::get_config, handler::Binance, utils::Core};

impl Binance<Core> {
    pub async fn get_account_core(self) -> Option<AccountInformation> {
        self.next_phase()
            .get_account_integration()
            .await
            .map_err(|err| {
                error_span!("Error - Binance", error = ?err);
                dbg!(eprint!("{err}"))
            })
            .ok()
    }

    pub async fn get_exchange_information_core(self) -> Option<ExchangeInformation> {
        self.next_phase()
            .get_exchange_information_integration()
            .await
            .map_err(|err| {
                error_span!("Error - Binance", error = ?err);
                dbg!(eprint!("{err}"));
            })
            .ok()
    }

    pub async fn post_new_order_core(
        self,
        symbol: String,
        order: &mut orders::Model,
    ) -> Result<(), ()> {
        self.next_phase()
            .post_new_order_integration(get_config().await.environment, symbol, order)
            .await
            .map_err(|err| {
                error_span!("Error - Binance", error = ?err);
                dbg!(eprint!("{err} \n"));
            })
    }
}
