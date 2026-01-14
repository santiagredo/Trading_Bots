use models::{
    entities::assets::Model,
    enums::LifecycleState,
    structs::{AssetRequest, CacheAssets, LedgerRequest},
};
use sea_orm::prelude::Decimal;
use tokio_util::sync::CancellationToken;

use crate::{
    handler::{Assets, Ledgers, Senders, DBC},
    utils::{handle_user_err, Cache, Core, Data, Logic, Response},
};

impl Assets<Core> {
    /* ===========================
     * DB
     * ===========================
     */

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

    /* ===========================
     * CACHE (READ)
     * ===========================
     */

    pub async fn get_assets_core(self) -> Option<CacheAssets> {
        Assets::<Cache>::get_assets_cache(self.environment).await
    }

    pub async fn get_asset_core(self) -> Option<Model> {
        Assets::<Cache>::get_asset_cache(self.environment, self.model.id.unwrap_or_default()).await
    }

    pub async fn get_assets_state_core(self) -> LifecycleState {
        Assets::<Cache>::get_assets_state_cache(self.environment).await
    }

    /* ===========================
     * CACHE (WRITE)
     * ===========================
     */

    pub async fn upsert_asset_core(self) -> Result<(), String> {
        let env = self.environment;
        let model = Assets::into_model(self.model);

        Assets::<Cache>::upsert_asset_cache(env, model).await
    }

    pub async fn remove_asset_core(self) -> Result<Option<Model>, String> {
        let env = self.environment;
        let id = self.model.id.unwrap_or_default();

        Assets::<Cache>::remove_asset_cache(env, id).await
    }

    pub async fn set_asset_value_core(
        self,
        value: Decimal,
        is_locked: bool,
        is_sell: bool,
    ) -> Result<(Model, Decimal), Response> {
        let Ok(result) = Assets::<Cache>::update_asset_balance_cache(
            self.environment,
            self.model.id.unwrap_or_default(),
            value,
            is_locked,
            is_sell,
        )
        .await
        else {
            return Err(Response {
                code: 404,
                message: "Memory asset not found".into(),
            });
        };

        Ok(result)
    }

    /* ===========================
     * START ACTIVE ASSETS
     * ===========================
     */

    pub async fn start_assets_core(self, token: &CancellationToken) -> Result<(), Response> {
        let env = self.environment;

        // STARTING
        if let Err(err) =
            Assets::<Cache>::set_status_cache(env, models::enums::LifecycleState::Starting).await
        {
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // Load from DB
        let models = match Assets::default().with_env(env).select_assets().await {
            Ok(m) => m,
            Err(err) => {
                let _ = Assets::<Cache>::reset_assets_cache(env).await;
                return Err(Response {
                    code: 500,
                    message: err.message,
                });
            }
        };

        // RUNNING
        if let Err(err) =
            Assets::<Cache>::set_status_cache(env, models::enums::LifecycleState::Running).await
        {
            let _ = Assets::<Cache>::reset_assets_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // Set assets
        if let Err(err) = Assets::<Cache>::set_assets_cache(env, models).await {
            let _ = Assets::<Cache>::reset_assets_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // Runtime
        let senders = Senders::get_senders().await;
        let mut orders_receiver = senders.order_sender.subscribe();
        let cancellation_token = token.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => break,
                    msg = orders_receiver.recv() => {
                        let Ok((environment, order)) = msg else { continue };

                        // BASE ASSET
                        let mut base_asset = Assets::default().with_env(environment);
                        base_asset.model.id = Some(order.base_asset_id);

                        let Ok((base_model, base_prev)) = base_asset
                            .set_asset_value(
                                order.base_asset_amount,
                                false,
                                order.is_sell,
                            )
                            .await else { continue };

                        let base_req = AssetRequest::from_model(&base_model);
                        let base_ledger = LedgerRequest::from_asset(&base_model)
                            .from_order(&order)
                            .update_values(
                                false,
                                order.base_asset_amount,
                                base_prev,
                                base_req.free.unwrap_or_default(),
                            );

                        if Assets::from_request(base_req)
                            .with_env(environment)
                            .update_asset()
                            .await
                            .is_err()
                        {
                            continue;
                        }

                        let _ = Ledgers::new(base_ledger)
                            .with_env(environment)
                            .insert_ledger()
                            .await;

                        // QUOTE ASSET
                        let mut quote_asset = Assets::default().with_env(environment);
                        quote_asset.model.id = Some(order.quote_asset_id);

                        let Ok((quote_model, quote_prev)) = quote_asset
                            .set_asset_value(
                                order.quote_asset_amount,
                                false,
                                !order.is_sell,
                            )
                            .await else { continue };

                        let quote_req = AssetRequest::from_model(&quote_model);
                        let quote_ledger = LedgerRequest::from_asset(&quote_model)
                            .from_order(&order)
                            .update_values(
                                false,
                                order.quote_asset_amount,
                                quote_prev,
                                quote_req.free.unwrap_or_default(),
                            );

                        if Assets::from_request(quote_req)
                            .with_env(environment)
                            .update_asset()
                            .await
                            .is_err()
                        {
                            continue;
                        }

                        let _ = Ledgers::new(quote_ledger)
                            .with_env(environment)
                            .insert_ledger()
                            .await;

                        // READY
                    }
                }
            }
        });

        Ok(())
    }

    /* ===========================
     * STOP ACTIVE ASSETS
     * ===========================
     */

    pub async fn stop_assets_core(self) -> Result<(), Response> {
        let env = self.environment;

        // STOPPING
        if let Err(err) =
            Assets::<Cache>::set_status_cache(env, models::enums::LifecycleState::Stopping).await
        {
            let _ = Assets::<Cache>::reset_assets_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // Remove assets
        if let Err(err) = Assets::<Cache>::remove_assets_cache(env).await {
            let _ = Assets::<Cache>::reset_assets_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // OFF
        if let Err(err) =
            Assets::<Cache>::set_status_cache(env, models::enums::LifecycleState::Off).await
        {
            let _ = Assets::<Cache>::reset_assets_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        Ok(())
    }

    pub async fn reset_assets_core(self) -> Result<(), Response> {
        let environment = self.environment;

        if let Err(err) = Assets::<Cache>::reset_assets_cache(environment).await {
            return Err(Response {
                code: 500,
                message: err,
            });
        };

        Ok(())
    }
}
