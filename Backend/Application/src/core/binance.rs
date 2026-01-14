use std::time::Instant;

use function_name::named;
use models::{
    entities,
    enums::AccountInformationResponse,
    structs::{
        environments, AccountInformation, ExchangeInformation, IntegrationLogRequest, OrderRequest,
    },
};

use crate::{
    handler::{Binance, IntegrationLogs, Integrations, IntegrationsSettings, Orders},
    static_strings::{
        ACCOUNT_INFORMATION_ENDPOINT, EXCHANGE_INFORMATION_ENDPOINT, ORDERS_ENDPOINT,
        ORDERS_TEST_ENDPOINT, X_MBX_APIKEY,
    },
    utils::{handle_user_err, Core, Response},
};

const INTEGRATION_NAME: &'static str = "BINANCE";

impl Binance<Core> {
    /* =====================================================
     * PUBLIC API – HIGH LEVEL (Core entrypoints)
     * ===================================================== */

    #[named]
    pub async fn get_account_core(
        self,
        api_key: String,
        secret_pass: String,
    ) -> Result<AccountInformation, Response> {
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

        if secret_pass.is_empty() {
            return Err(handle_user_err(format!(
                "Binance secret pass can't be empty"
            )));
        }

        if secret_pass.is_empty() {
            return Err(handle_user_err(format!("Binance api key can't be empty")));
        }

        let request = match Binance::get_account_logic(
            ACCOUNT_INFORMATION_ENDPOINT,
            &secret_pass,
            &api_key,
            X_MBX_APIKEY,
        ) {
            Err(err) => {
                let execution_time_ms = start.elapsed().as_millis();

                let code = 500;

                log.status_code = Some(code);
                log.execution_time_ms = Some(execution_time_ms.try_into().unwrap_or_default());
                log.error_message = Some(err.clone());

                let _ = IntegrationLogs::new(&environment, log).insert_log().await;

                return Err(Response {
                    code: code as u16,
                    message: err,
                });
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

        let outcome = match result {
            Err(err) => {
                let code = err.status().map_or(500, |s| s.as_u16());

                log.error_message = Some(err.to_string());
                let _ = IntegrationLogs::new(&environment, log).insert_log().await;

                let err = err.to_string();
                dbg!(&err);

                return Err(Response { code, message: err });
            }
            Ok(response) => {
                let body = response.text().await.unwrap_or_default();
                log.response = Some(body.clone());
                body
            }
        };

        let outcome = match serde_json::from_str::<AccountInformationResponse>(&outcome) {
            Err(err) => {
                let code = 500;

                log.error_message = Some(err.to_string());
                let _ = IntegrationLogs::new(&environment, log).insert_log().await;

                let err = err.to_string();
                dbg!(&err);

                return Err(Response { code, message: err });
            }
            Ok(val) => val,
        };

        match outcome {
            AccountInformationResponse::Error(err) => {
                log.error_message = Some(format!("code: {} - message: {}", err.code, err.msg));
                let _ = IntegrationLogs::new(&environment, log).insert_log().await;

                let message = format!("code: {} - message: {}", err.code, err.msg);
                dbg!(&message);

                Err(Response { code: 500, message })
            }
            AccountInformationResponse::AccountInformation(val) => {
                let _ = IntegrationLogs::new(&environment, log).insert_log().await;
                Ok(val)
            }
        }
    }

    pub async fn get_account_manually_core(self) -> Result<AccountInformation, Response> {
        let (api_key, secret) = self.resolve_binance_credentials_core().await?;
        self.get_account_core(api_key, secret).await
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

        let outcome: Result<String, String> = match result {
            Ok(response) => {
                let body = response.text().await.unwrap_or_default();
                log.response = Some(Self::truncate_utf8(&body, 1000));
                Ok(body)
            }
            Err(err) => {
                log.error_message = Some(err.to_string());
                Err(err.to_string())
            }
        };

        let outcome = outcome.and_then(|body| {
            serde_json::from_str::<ExchangeInformation>(&body).map_err(|err| {
                log.error_message = Some(err.to_string());
                err.to_string()
            })
        });

        let _ = IntegrationLogs::new(&environment, log).insert_log().await;

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
        api_key: String,
        secret_pass: String,
    ) -> Result<(), Response> {
        let environment = self.environment;
        let start = Instant::now();

        if secret_pass.is_empty() {
            return Err(handle_user_err(format!(
                "Binance secret pass can't be empty"
            )));
        }

        if secret_pass.is_empty() {
            return Err(handle_user_err(format!("Binance api key can't be empty")));
        }

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
            &secret_pass,
            &api_key,
            X_MBX_APIKEY,
            &mut order_request,
        ) {
            Err(err) => {
                let execution_time_ms = start.elapsed().as_millis();

                log.status_code = Some(500);
                log.execution_time_ms = Some(execution_time_ms.try_into().unwrap_or_default());
                log.error_message = Some(err.to_string());

                let _ = IntegrationLogs::new(&environment, log).insert_log().await;

                return Err(Response {
                    code: 500,
                    message: err,
                });
            }
            Ok(val) => val,
        };

        async move {
            let result = self.next_phase().post_new_order_integration(request).await;

            let execution_time_ms = start.elapsed().as_millis();

            let status_code = Some(
                result
                    .as_ref()
                    .map(|r| r.status().as_u16())
                    .unwrap_or_else(|e| e.status().map_or(500, |s| s.as_u16()))
                    .into(),
            );
            log.status_code = status_code;
            log.execution_time_ms = Some(execution_time_ms.try_into().unwrap_or_default());

            let outcome = match result {
                Err(err) => {
                    log.error_message = Some(err.to_string());
                    let _ = IntegrationLogs::new(&environment, log).insert_log().await;
                    dbg!(err.to_string());

                    return Err(Response {
                        code: status_code.unwrap_or(500) as u16,
                        message: err.to_string(),
                    });
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

                    Err(Response {
                        code: 500,
                        message: err,
                    })
                }
                Ok(val) => {
                    let _ = IntegrationLogs::new(&environment, log).insert_log().await;
                    Ok(val)
                }
            }
        }
        .await
    }

    /* =====================================================
     * RESOLVERS (Cache / DB orchestration)
     * ===================================================== */

    pub async fn resolve_binance_integration_core(
        &self,
    ) -> Result<entities::integrations::Model, Response> {
        let cache_integrations = Integrations::default()
            .with_env(self.environment)
            .get_integrations()
            .await;

        let integration = match cache_integrations {
            Some(val) => val
                .models
                .values()
                .find(|int| int.code == "BINANCE")
                .cloned(),

            None => {
                let mut req = Integrations::default().with_env(self.environment);
                req.model.code = Some("BINANCE".to_string());

                let db_integrations = req.select_integrations().await?;

                db_integrations
                    .into_iter()
                    .find(|int| int.code == "BINANCE")
            }
        };

        integration.ok_or(Response {
            code: 404,
            message: "Binance integration not found in cache or db".to_string(),
        })
    }

    pub async fn resolve_binance_credentials_core(&self) -> Result<(String, String), Response> {
        let binance = self.resolve_binance_integration_core().await?;

        let mut settings_request = IntegrationsSettings::default().with_env(self.environment);
        settings_request.model.integration_id = Some(binance.id);

        let settings = settings_request.resolve_integration_settings().await?;

        let api_key = IntegrationsSettings::resolve_setting_value(&settings, "api_key")?;
        let secret = IntegrationsSettings::resolve_setting_value(&settings, "secret_pass")?;

        Ok((api_key, secret))
    }

    /* =====================================================
     * INTERNAL HELPERS
     * ===================================================== */

    fn truncate_utf8(s: &str, max_chars: usize) -> String {
        let mut end = s.len();
        let mut count = 0;

        for (idx, _) in s.char_indices() {
            if count == max_chars {
                end = idx;
                break;
            }
            count += 1;
        }

        s[..end].to_string()
    }
}
