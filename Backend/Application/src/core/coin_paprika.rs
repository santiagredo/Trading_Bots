use std::time::Instant;

use function_name::named;
use models::structs::{CoinPaprikaTicker, IntegrationLogRequest};

use crate::{
    handler::{CoinPaprika, IntegrationLogs},
    static_strings::COINPAPRIKA_TICKERS_ENDPOINT,
    utils::Core,
};

const INTEGRATION_NAME: &'static str = "COINPAPRIKA";

impl CoinPaprika<Core> {
    #[named]
    pub async fn get_tickers_core(self) -> Result<Vec<CoinPaprikaTicker>, String> {
        let environment = self.environment;
        let start = Instant::now();

        let result = self.next_phase().get_tickers_integration().await;

        let execution_time_ms = start.elapsed().as_millis();

        let mut log = IntegrationLogRequest {
            id: None,
            creation_date: None,
            integration_name: Some(INTEGRATION_NAME.to_string()),
            function_name: Some(function_name!().to_string()),
            url: Some(COINPAPRIKA_TICKERS_ENDPOINT.to_string()),
            status_code: Some(
                result
                    .as_ref()
                    .map(|r| r.status().as_u16())
                    .unwrap_or_else(|e| e.status().map_or(500, |s| s.as_u16()))
                    .into(),
            ),
            execution_time_ms: Some(execution_time_ms.try_into().unwrap()),
            request: Some(format!("GET request: {COINPAPRIKA_TICKERS_ENDPOINT}")),
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

        // String -> Vec<CoinPaprikaTicker>
        let outcome = outcome.and_then(|body| {
            serde_json::from_str::<Vec<CoinPaprikaTicker>>(&body).map_err(|err| {
                log.error_message = Some(err.to_string());
                err.to_string()
            })
        });

        // Persist log
        let _ = IntegrationLogs::new(&environment, log).insert_log().await;

        // Final return
        outcome
    }
}
