use std::time::Instant;

use function_name::named;
use models::{
    enums::AccountInformationResponse,
    structs::{
        environments, AccountInformation, ExchangeInformation, IntegrationLogRequest, OrderRequest,
    },
};

use crate::{
    handler::{Binance, Configurations, IntegrationLogs, Orders},
    static_strings::{
        ACCOUNT_INFORMATION_ENDPOINT, EXCHANGE_INFORMATION_ENDPOINT, ORDERS_ENDPOINT,
        ORDERS_TEST_ENDPOINT, X_MBX_APIKEY,
    },
    utils::Core,
};

const INTEGRATION_NAME: &'static str = "BINANCE";

impl Binance<Core> {
    #[named]
    pub async fn get_account_core(self) -> Option<AccountInformation> {
        let environment = self.environment;
        let start = Instant::now();

        let mut log = IntegrationLogRequest {
            id: None,
            creation_date: None,
            integration_name: Some(INTEGRATION_NAME.to_string()),
            function_name: Some(function_name!().to_string()),
            url: Some(ACCOUNT_INFORMATION_ENDPOINT.to_string()),
            status_code: None,
            execution_time_ms: None,
            request: Some(format!("GET request: {ACCOUNT_INFORMATION_ENDPOINT}")),
            response: None,
            error_message: None,
        };

        let request = match Binance::get_account_logic(
            ACCOUNT_INFORMATION_ENDPOINT,
            Configurations::default()
                .select_configuration()
                .await
                .secret_pass
                .as_ref(),
            Configurations::default()
                .select_configuration()
                .await
                .api_key
                .as_ref(),
            X_MBX_APIKEY,
        ) {
            Err(err) => {
                let execution_time_ms = start.elapsed().as_millis();

                log.status_code = Some(500);
                log.execution_time_ms = Some(execution_time_ms.try_into().unwrap_or_default());
                log.error_message = Some(err);

                let _ = IntegrationLogs::new(&environment, log).insert_log().await;

                return None;
            }
            Ok(val) => val,
        };

        let result = self.next_phase().get_account_integration(request).await;

        let execution_time_ms = start.elapsed().as_millis();

        log.status_code = Some(
            result
                .as_ref()
                .map(|r| r.status().as_u16())
                .unwrap_or_else(|e| e.status().map_or(500, |s| s.as_u16()))
                .into(),
        );
        log.execution_time_ms = Some(execution_time_ms.try_into().unwrap_or_default());

        // HTTP -> String
        let outcome = match result {
            Err(err) => {
                log.error_message = Some(err.to_string());
                let _ = IntegrationLogs::new(&environment, log).insert_log().await;
                dbg!(err.to_string());
                return None;
            }
            Ok(response) => {
                let body = response.text().await.unwrap_or_default();
                log.response = Some(body.clone());
                body
            }
        };

        // String -> Result<AccountInformationResponse>
        let outcome = match serde_json::from_str::<AccountInformationResponse>(&outcome) {
            Err(err) => {
                log.error_message = Some(err.to_string());
                let _ = IntegrationLogs::new(&environment, log).insert_log().await;
                dbg!(err.to_string());
                return None;
            }
            Ok(val) => val,
        };

        match outcome {
            AccountInformationResponse::Error(err) => {
                log.error_message = Some(format!("code: {} - message: {}", err.code, err.msg));
                let _ = IntegrationLogs::new(&environment, log).insert_log().await;
                dbg!(format!("code: {} - message: {}", err.code, err.msg));
                return None;
            }
            AccountInformationResponse::AccountInformation(val) => {
                let _ = IntegrationLogs::new(&environment, log).insert_log().await;
                Some(val)
            }
        }
    }

    #[named]
    pub async fn get_exchange_information_core(self) -> Option<ExchangeInformation> {
        let environment = self.environment;
        let start = Instant::now();

        let result = self
            .next_phase()
            .get_exchange_information_integration()
            .await;

        let execution_time_ms = start.elapsed().as_millis();

        let mut log = IntegrationLogRequest {
            id: None,
            creation_date: None,
            integration_name: Some(INTEGRATION_NAME.to_string()),
            function_name: Some(function_name!().to_string()),
            url: Some(EXCHANGE_INFORMATION_ENDPOINT.to_string()),
            status_code: Some(
                result
                    .as_ref()
                    .map(|r| r.status().as_u16())
                    .unwrap_or_else(|e| e.status().map_or(500, |s| s.as_u16()))
                    .into(),
            ),
            execution_time_ms: Some(execution_time_ms.try_into().unwrap_or_default()),
            request: Some(format!("GET request: {EXCHANGE_INFORMATION_ENDPOINT}")),
            response: None,
            error_message: None,
        };

        // HTTP -> String
        let outcome: Result<String, String> = match result {
            Ok(response) => {
                let body = response.text().await.unwrap_or_default();
                log.response = Some(body.clone());
                Ok(body)
            }
            Err(err) => {
                log.error_message = Some(err.to_string());
                Err(err.to_string())
            }
        };

        // String -> Result<ExchangeInformation>
        let outcome = outcome.and_then(|body| {
            serde_json::from_str::<ExchangeInformation>(&body).map_err(|err| {
                log.error_message = Some(err.to_string());
                err.to_string()
            })
        });

        // Persist log
        let _ = IntegrationLogs::new(&environment, log).insert_log().await;

        // Final return
        match outcome {
            Err(err) => {
                dbg!(err);
                None
            }
            Ok(val) => Some(val),
        }
    }

    #[named]
    pub async fn post_new_order_core(
        self,
        symbol: String,
        order: &mut OrderRequest,
    ) -> Result<(), String> {
        let environment = self.environment;
        let start = Instant::now();

        let endpoint = match environment {
            environments::Environments::PROD => ORDERS_ENDPOINT,
            _ => ORDERS_TEST_ENDPOINT,
        };

        let order_model = Orders::from_request(order.clone());
        let mut order_request = Binance::build_binance_order_request_logic(symbol, order_model);

        let mut log = IntegrationLogRequest {
            id: None,
            creation_date: None,
            integration_name: Some(INTEGRATION_NAME.to_string()),
            function_name: Some(function_name!().to_string()),
            url: Some(endpoint.to_string()),
            status_code: None,
            execution_time_ms: None,
            request: Some(serde_json::to_string(&order_request).unwrap_or_default()),
            response: None,
            error_message: None,
        };

        let request = match Binance::post_new_order_logic(
            ACCOUNT_INFORMATION_ENDPOINT,
            Configurations::default()
                .select_configuration()
                .await
                .secret_pass
                .as_ref(),
            Configurations::default()
                .select_configuration()
                .await
                .api_key
                .as_ref(),
            X_MBX_APIKEY,
            &mut order_request,
        ) {
            Err(err) => {
                let execution_time_ms = start.elapsed().as_millis();

                log.status_code = Some(500);
                log.execution_time_ms = Some(execution_time_ms.try_into().unwrap_or_default());
                log.error_message = Some(err.to_string());

                let _ = IntegrationLogs::new(&environment, log).insert_log().await;

                return Err(err);
            }
            Ok(val) => val,
        };

        async move {
            let result = self.next_phase().post_new_order_integration(request).await;

            let execution_time_ms = start.elapsed().as_millis();

            log.status_code = Some(
                result
                    .as_ref()
                    .map(|r| r.status().as_u16())
                    .unwrap_or_else(|e| e.status().map_or(500, |s| s.as_u16()))
                    .into(),
            );
            log.execution_time_ms = Some(execution_time_ms.try_into().unwrap_or_default());

            // HTTP -> String
            let outcome = match result {
                Err(err) => {
                    log.error_message = Some(err.to_string());
                    let _ = IntegrationLogs::new(&environment, log).insert_log().await;
                    dbg!(err.to_string());
                    return Err(err.to_string());
                }
                Ok(response) => {
                    let body = response.text().await.unwrap_or_default();
                    log.response = Some(body.clone());
                    body
                }
            };

            match Binance::map_new_order_logic(environment, outcome, order).await {
                Err(err) => {
                    log.error_message = Some(err.to_string());
                    let _ = IntegrationLogs::new(&environment, log).insert_log().await;
                    dbg!(err.to_string());
                    Err(err)
                }
                Ok(val) => {
                    let _ = IntegrationLogs::new(&environment, log).insert_log().await;
                    Ok(val)
                }
            }
        }
        .await
    }
}
