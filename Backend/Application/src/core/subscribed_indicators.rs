use crate::{
    handler::{Indicators, Senders, SubscribedIndicators},
    utils::{EntityCache, Response},
};
use models::{enums::WebsocketCommand, structs::Environments};

impl<R> SubscribedIndicators<R>
where
    R: Send + Sync,
{
    /* ===========================
     * LIFECYCLE
     * ===========================
     */

    pub async fn start(&self, environment: Environments) -> Result<(), Response> {
        // LOAD ACTIVE INDICATORS
        let active_indicators: Vec<String> = Indicators::blank()
            .get_all(environment)
            .await
            .unwrap_or_default()
            .models
            .into_iter()
            .map(|(_, model)| model.symbol)
            .collect();

        self.set_all(active_indicators)
            .await
            .map_err(Response::server_error)?;

        Ok(())
    }

    pub async fn stop(&self, environment: Environments, senders: Senders) -> Result<(), Response> {
        // LOAD ACTIVE INDICATORS
        let active_indicators: Vec<String> = Indicators::blank()
            .get_all(environment)
            .await
            .unwrap_or_default()
            .models
            .into_iter()
            .map(|(_, model)| model.symbol)
            .collect();

        for indicator in active_indicators {
            self.remove(indicator).await;
        }

        self.remove_entries().await;

        senders
            .command_sender
            .send(WebsocketCommand::Unsubscribe)
            .map_err(|err| Response::server_error(err.to_string()))?;

        Ok(())
    }

    pub async fn get_all_unique_symbols(&self) -> Result<Vec<String>, String> {
        let symbols: Vec<String> = self
            .get_all()
            .await
            .models
            .into_iter()
            .map(|(symbol, _)| symbol)
            .collect();

        Ok(symbols.into_iter().collect())
    }
}
