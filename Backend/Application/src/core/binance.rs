use crate::{
    handler::{Binance, IntegrationLogs, Integrations, IntegrationsSettings, Orders},
    logic,
    static_strings::{ACCOUNT_INFORMATION_ENDPOINT, EXCHANGE_INFORMATION_ENDPOINT, X_MBX_APIKEY},
    utils::{EntityCache, RepoFactory, Response},
};
use function_name::named;
use models::{
    entities::{self, integration_log, integration_settings, integrations},
    enums::{AccountInformationResponse, BinanceRestResponse},
    structs::{
        AccountInformation, Environments, ExchangeInformation, IntegrationLogRequest,
        IntegrationRequest, IntegrationSettingRequest, OrderRequest,
    },
};
use std::time::Instant;

const INTEGRATION_NAME: &'static str = "BINANCE";

impl<R> Binance<R> {
    /* =====================================================
     * PUBLIC API – HIGH LEVEL
     * ===================================================== */

    #[named]
    pub async fn get_account(
        &self,
        api_key: String,
        secret_pass: String,
        factory: RepoFactory,
    ) -> Result<AccountInformation, Response> {
        let integration_log_repo = factory.repo::<IntegrationLogRequest, integration_log::Model>();

        let request = match logic::binance::build_account_request(
            ACCOUNT_INFORMATION_ENDPOINT,
            &secret_pass,
            &api_key,
            X_MBX_APIKEY,
        ) {
            Err(err) => {
                return Err(Response {
                    code: 500,
                    message: err,
                });
            }
            Ok(val) => val,
        };

        let start = Instant::now();

        let result = self.send_account_request(request).await;

        let execution_time_ms = start.elapsed().as_millis();

        let mut integration_log = IntegrationLogRequest::new(
            INTEGRATION_NAME.to_string(),
            function_name!().to_string(),
            ACCOUNT_INFORMATION_ENDPOINT.to_string(),
        );

        integration_log.with_execution_time_ms(execution_time_ms.try_into().unwrap_or(-1));

        let body =
            match logic::integration_logs::map_request_result(result, &mut integration_log).await {
                Err(err) => {
                    let _ = IntegrationLogs::new(integration_log_repo)
                        .insert(integration_log)
                        .await;

                    return Err(err);
                }
                Ok(val) => val,
            };

        let serde_result = serde_json::from_str::<AccountInformationResponse>(&body);

        let outcome =
            match logic::integration_logs::map_serde_result(serde_result, &mut integration_log) {
                Err(err) => {
                    let _ = IntegrationLogs::new(integration_log_repo)
                        .insert(integration_log)
                        .await;

                    return Err(err);
                }
                Ok(val) => val,
            };

        match outcome {
            AccountInformationResponse::Error(err) => {
                let message = format!("code: {} - message: {}", err.code, err.msg);
                integration_log.with_error_message(message.clone());
                dbg!(&message);

                let _ = IntegrationLogs::new(integration_log_repo)
                    .insert(integration_log)
                    .await;

                Err(Response { code: 500, message })
            }
            AccountInformationResponse::AccountInformation(val) => {
                let _ = IntegrationLogs::new(integration_log_repo)
                    .insert(integration_log)
                    .await;

                Ok(val)
            }
        }
    }

    #[named]
    pub async fn get_exchange_information(
        &self,
        factory: RepoFactory,
    ) -> Result<ExchangeInformation, Response> {
        let integration_log_repo = factory.repo::<IntegrationLogRequest, integration_log::Model>();

        let start = Instant::now();

        let result = self.send_exchange_information_request().await;

        let execution_time_ms = start.elapsed().as_millis();

        let mut integration_log = IntegrationLogRequest::new(
            INTEGRATION_NAME.to_string(),
            function_name!().to_string(),
            EXCHANGE_INFORMATION_ENDPOINT.to_string(),
        );

        integration_log.with_execution_time_ms(execution_time_ms.try_into().unwrap_or(-1));
        integration_log.with_request(format!("GET request: {EXCHANGE_INFORMATION_ENDPOINT}"));

        let body =
            match logic::integration_logs::map_request_result(result, &mut integration_log).await {
                Err(err) => {
                    let _ = IntegrationLogs::new(integration_log_repo)
                        .insert(integration_log)
                        .await;

                    return Err(err);
                }
                Ok(val) => {
                    integration_log.response = Some(Self::truncate_utf8(&val, 1000));

                    val
                }
            };

        let serde_result = serde_json::from_str::<ExchangeInformation>(&body);

        let outcome =
            match logic::integration_logs::map_serde_result(serde_result, &mut integration_log) {
                Err(err) => {
                    let _ = IntegrationLogs::new(integration_log_repo)
                        .insert(integration_log)
                        .await;

                    return Err(err);
                }
                Ok(val) => val,
            };

        Ok(outcome)
    }

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

    #[named]
    pub async fn post_new_order(
        &self,
        factory: RepoFactory,
        symbol: String,
        order: &mut OrderRequest,
        api_key: String,
        secret_pass: String,
        endpoint: &str,
    ) -> Result<(), Response> {
        let integration_log_repo = factory.repo::<IntegrationLogRequest, integration_log::Model>();

        let mut integration_log = IntegrationLogRequest::new(
            INTEGRATION_NAME.to_string(),
            function_name!().to_string(),
            endpoint.to_string(),
        );

        let order_model = Orders::from_request(order.clone());
        let mut order_request =
            match logic::binance::build_order(symbol, order_model, &api_key, &secret_pass) {
                Err(err) => {
                    integration_log.with_status_code(err.code.try_into().unwrap_or(400));
                    integration_log.with_error_message(err.message.clone());

                    let _ = IntegrationLogs::new(integration_log_repo)
                        .insert(integration_log)
                        .await;

                    return Err(err);
                }
                Ok(val) => val,
            };

        let request = match logic::binance::build_order_request(
            endpoint,
            &secret_pass,
            &api_key,
            X_MBX_APIKEY,
            &mut order_request,
        ) {
            Err(err) => {
                integration_log.with_status_code(err.code.try_into().unwrap_or(500));
                integration_log.with_error_message(err.message.clone());

                let _ = IntegrationLogs::new(integration_log_repo)
                    .insert(integration_log)
                    .await;

                return Err(err);
            }
            Ok(val) => val,
        };

        let start = Instant::now();

        let result = self.send_new_order_request(request).await;

        let execution_time_ms = start.elapsed().as_millis();

        integration_log.with_execution_time_ms(execution_time_ms.try_into().unwrap_or(-1));
        integration_log.with_request(serde_json::to_string(&order_request).unwrap_or_default());

        let body =
            match logic::integration_logs::map_request_result(result, &mut integration_log).await {
                Err(err) => {
                    let _ = IntegrationLogs::new(integration_log_repo)
                        .insert(integration_log)
                        .await;

                    return Err(err);
                }
                Ok(val) => val,
            };

        // in non production environtments: {} means a successful order
        if body == "{}".to_string() {
            return Ok(());
        }

        let serde_result = serde_json::from_str::<BinanceRestResponse>(&body);

        let outcome =
            match logic::integration_logs::map_serde_result(serde_result, &mut integration_log) {
                Err(err) => {
                    let _ = IntegrationLogs::new(integration_log_repo)
                        .insert(integration_log)
                        .await;

                    return Err(err);
                }
                Ok(val) => val,
            };

        logic::binance::map_new_order(outcome, order)
    }

    /* =====================================================
     * RESOLVERS (Cache / DB orchestration)
     * ===================================================== */

    pub async fn resolve_binance_integration(
        &self,
        environment: Environments,
        factory: RepoFactory,
    ) -> Result<entities::integrations::Model, Response> {
        let integrations_repo = factory.repo::<IntegrationRequest, integrations::Model>();
        let cache_integrations = Integrations::new(integrations_repo.clone())
            .get_all(environment)
            .await;

        let integration = match cache_integrations {
            Some(val) => val
                .models
                .values()
                .find(|int| int.code == "BINANCE")
                .cloned(),

            None => {
                // let mut req = ;
                let req = IntegrationRequest {
                    code: Some("BINANCE".to_string()),
                    ..Default::default()
                };

                Integrations::new(integrations_repo).select(req).await?
            }
        };

        integration.ok_or(Response {
            code: 404,
            message: "Binance integration not found in cache or db".to_string(),
        })
    }

    pub async fn resolve_binance_credentials(
        &self,
        factory: RepoFactory,
        environment: Environments,
    ) -> Result<(String, String), Response> {
        let binance = self
            .resolve_binance_integration(environment, factory.clone())
            .await?;

        let req = IntegrationSettingRequest {
            integration_id: Some(binance.id),
            ..Default::default()
        };

        let integrations_settings_repo =
            factory.repo::<IntegrationSettingRequest, integration_settings::Model>();
        let settings = IntegrationsSettings::new(integrations_settings_repo)
            .resolve_integration_settings(environment, req)
            .await?;

        let api_key = logic::integrations_settings::resolve_setting_value(&settings, "api_key")
            .map_err(Response::bad_request)?;
        let secret = logic::integrations_settings::resolve_setting_value(&settings, "secret_pass")
            .map_err(Response::bad_request)?;

        Ok((api_key, secret))
    }

    pub async fn resolve_account(
        &self,
        factory: RepoFactory,
        environment: Environments,
    ) -> Result<AccountInformation, Response> {
        let (api_key, secret) = self
            .resolve_binance_credentials(factory.clone(), environment)
            .await?;
        self.get_account(api_key, secret, factory).await
    }
}
