use std::time::Instant;

use function_name::named;
use models::structs::{
    AccountInformation, ExchangeInformation, IntegrationLogRequest, OrderRequest,
};
use tracing::error_span;

use crate::{
    config::get_config,
    handler::{Binance, IntegrationLogs, Orders},
    static_strings::{ORDERS_ENDPOINT, ORDERS_TEST_ENDPOINT},
    utils::Core,
};

const INTEGRATION_NAME: &'static str = "BINANCE";

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

    #[named]
    pub async fn post_new_order_core(
        self,
        symbol: String,
        order: &mut OrderRequest,
    ) -> Result<(), String> {
        let order_model = Orders::from_request(order.clone());
        let mut order_request = Binance::build_binance_order_request_logic(symbol, order_model);
        let environment = get_config().await.environment;
        let endpoint = match environment {
            crate::environments::Environments::PRO => ORDERS_ENDPOINT,
            _ => ORDERS_TEST_ENDPOINT,
        };

        // let span = warn_span!("Binance - Order - Request", ?order_request);

        async move {
            let start = Instant::now();

            let binance_response = self
                .next_phase()
                .post_new_order_integration(endpoint, &mut order_request)
                .await
                .map_err(|err| {
                    error_span!("Error - Binance", error = ?err);
                    dbg!(eprint!("{err} \n"));
                    err
                })?;

            let mut integreation_log_request = IntegrationLogRequest {
                integration_name: Some(INTEGRATION_NAME.to_string()),
                function_name: Some(function_name!().to_string()),
                url: Some(endpoint.to_string()),
                status_code: Some(binance_response.status().as_u16().into()),
                execution_time_ms: Some(start.elapsed().as_millis().try_into().unwrap_or(-1)),
                ..Default::default()
            };
            integreation_log_request.request =
                Some(serde_json::to_string(&order_request).unwrap_or_default());

            let binance_response = binance_response
                .text()
                .await
                .map_err(|err| err.to_string())?;
            integreation_log_request.response = Some(binance_response.clone());

            // warn_span!("Binance - Order - Response", %binance_response);

            let result = match Binance::post_new_order_logic(
                get_config().await.environment,
                binance_response,
                order,
            )
            .await
            {
                Err(err) => {
                    // error_span!("Error - Binance", error = ?err);
                    integreation_log_request.error_message = Some(err.clone());
                    dbg!(eprint!("{err} \n"));
                    Err(err)
                }
                Ok(val) => Ok(val),
            };

            let _ = IntegrationLogs::new(integreation_log_request)
                .insert_log()
                .await;

            result
        }
        // .instrument(span)
        .await
    }
}
