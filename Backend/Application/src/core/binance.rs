use models::structs::{AccountInformation, ExchangeInformation, OrderRequest};
use tracing::{error_span, warn_span, Instrument};

use crate::{
    config::get_config,
    handler::{Binance, Orders},
    utils::Core,
};

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
        order: &mut OrderRequest,
    ) -> Result<(), String> {
        let order_model = Orders::from_request(order.clone());
        let order_request = Binance::build_binance_order_request_logic(symbol, order_model);

        let span = warn_span!("Binance - Order - Request", ?order_request);

        async move {
            let binance_response = self
                .next_phase()
                .post_new_order_integration(get_config().await.environment, order_request)
                .await
                .map_err(|err| {
                    error_span!("Error - Binance", error = ?err);
                    dbg!(eprint!("{err} \n"));
                    err
                })?;

            warn_span!("Binance - Order - Response", ?binance_response);

            match Binance::post_new_order_logic(
                get_config().await.environment,
                binance_response,
                order,
            )
            .await
            {
                Err(err) => {
                    error_span!("Error - Binance", error = ?err);
                    dbg!(eprint!("{err} \n"));
                    Err(err)
                }
                Ok(val) => Ok(val),
            }
        }
        .instrument(span)
        .await
    }
}
