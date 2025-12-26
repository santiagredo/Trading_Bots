use std::collections::HashMap;

use models::{
    entities::assets::Model,
    structs::{AssetRequest, CacheAsset, LedgerRequest},
};
use sea_orm::prelude::Decimal;

use crate::{
    handler::{Assets, Ledgers, Senders, DBC},
    utils::{handle_user_err, Cache, Core, Data, Logic, Response},
};

impl Assets<Core> {
    // db
    pub async fn insert_asset_core(self) -> Result<Model, Response> {
        let env = self.environment;

        self.next_phase::<Logic>()
            .insert_asset_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .insert_asset_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn select_asset_core(self) -> Result<Option<Model>, Response> {
        let env = self.environment;

        self.next_phase::<Data>()
            .select_asset_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn select_assets_core(self) -> Result<Vec<Model>, Response> {
        let env = self.environment;

        self.next_phase::<Data>()
            .select_assets_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn update_asset_core(self) -> Result<Model, Response> {
        let env = self.environment;

        self.next_phase::<Logic>()
            .update_asset_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .update_asset_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn delete_asset_core(self) -> Result<u64, Response> {
        let env = self.environment;

        self.next_phase::<Logic>()
            .delete_asset_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .delete_asset_data(&DBC::db(&env).await?)
            .await
    }

    // cache
    pub async fn get_active_assets_core(self) -> Option<HashMap<i32, CacheAsset>> {
        Assets::<Cache>::get_active_assets_cache(&self.environment).await
    }

    pub async fn get_active_asset_core(self) -> Option<CacheAsset> {
        Assets::<Cache>::get_active_asset_cache(
            &self.environment,
            &self.model.id.unwrap_or_default(),
        )
        .await
    }

    pub async fn set_active_asset_core(self, is_remove: bool) -> Model {
        let env = self.environment;
        let model = Assets::into_model(self.model);

        Assets::<Cache>::set_active_asset_cache(&env, model, is_remove).await
    }

    pub async fn set_active_asset_value_core(
        self,
        value: Decimal,
        is_locked: bool,
        is_sell: bool,
    ) -> Result<(Model, Decimal), Response> {
        let Some(memory_asset) = Assets::<Cache>::set_active_asset_value_cache(
            &self.environment,
            &self.model.id.unwrap_or_default(),
            value,
            is_locked,
            is_sell,
        )
        .await
        else {
            return Err(Response::not_found("Memory asset".to_string()));
        };

        Ok(memory_asset)
    }

    pub async fn start_active_assets_core(self) -> Result<(), Response> {
        let env = self.environment;

        if !Assets::<Cache>::get_active_assets_status_cache(&env).await {
            let mut assets_request = Assets::default();
            assets_request.environment = env;

            let models = assets_request.select_assets().await?;

            Assets::<Cache>::set_active_assets_cache(&env, models).await;

            let senders = Senders::get_active_senders().await;
            let mut orders_receiver = senders.order_sender.subscribe();

            let join_handle = tokio::spawn(async move {
                while let Ok((environment, order)) = orders_receiver.recv().await {
                    // base asset
                    let mut base_asset = Assets::default().with_env(environment);
                    base_asset.model.id = Some(order.base_asset_id);

                    let (base_model, base_previous_balance) = base_asset
                        .set_active_asset_value(order.base_asset_amount, false, order.is_sell)
                        .await
                        .unwrap_or_default();

                    let base_asset_request = AssetRequest::from_model(&base_model);

                    let base_asset_ledger = LedgerRequest::from_asset(&base_model)
                        .from_order(&order)
                        .update_values(
                            false,
                            order.base_asset_amount,
                            base_previous_balance,
                            base_asset_request.free.unwrap_or_default(),
                        );

                    if let Err(err) = Assets::from_request(base_asset_request)
                        .with_env(environment)
                        .update_asset()
                        .await
                    {
                        dbg!(eprintln!("{}", err.message));
                        continue;
                    };

                    if let Err(err) = Ledgers::new(base_asset_ledger)
                        .with_env(environment)
                        .insert_ledger()
                        .await
                    {
                        dbg!(eprintln!("{}", err.message));
                        continue;
                    };

                    // quote asset
                    let mut quote_asset = Assets::default().with_env(environment);
                    quote_asset.model.id = Some(order.quote_asset_id);

                    let (quote_model, quote_previous_balance) = quote_asset
                        .set_active_asset_value(order.quote_asset_amount, false, !order.is_sell)
                        .await
                        .unwrap_or_default();

                    let quote_asset_request = AssetRequest::from_model(&quote_model);

                    let quote_asset_ledger = LedgerRequest::from_asset(&quote_model)
                        .from_order(&order)
                        .update_values(
                            false,
                            order.quote_asset_amount,
                            quote_previous_balance,
                            quote_asset_request.free.unwrap_or_default(),
                        );

                    if let Err(err) = Assets::from_request(quote_asset_request)
                        .with_env(environment)
                        .update_asset()
                        .await
                    {
                        dbg!(eprintln!("{}", err.message));
                        continue;
                    };

                    if let Err(err) = Ledgers::new(quote_asset_ledger)
                        .with_env(environment)
                        .insert_ledger()
                        .await
                    {
                        dbg!(eprintln!("{}", err.message));
                        continue;
                    };
                }
            });

            Assets::<Cache>::set_active_assets_join_handle_cache(&self.environment, join_handle)
                .await;
        }

        Ok(())
    }

    pub async fn stop_active_assets_core(self) -> Result<(), Response> {
        let env = self.environment;

        Assets::<Cache>::stop_active_assets_cache(&env)
            .await
            .map_err(handle_user_err)
    }
}
