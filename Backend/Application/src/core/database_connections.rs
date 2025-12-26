use models::structs::{DatabaseManager, Environments};
use sea_orm::DatabaseConnection;

use crate::{
    handler::{Configurations, DBC},
    utils::{Cache, Core, Data, Response},
};

impl DBC<Core> {
    pub async fn get_database_core(
        environment: &Environments,
    ) -> Result<DatabaseConnection, Response> {
        if let Some(database_manager) = DBC::<Cache>::get_database_cache().await {
            let conn = match environment {
                Environments::PROD => database_manager.prod,
                _ => database_manager.dev,
            };

            if conn.ping().await.is_ok() {
                return Ok(conn);
            }
        }

        let configuration = Configurations::default().select_configuration().await;

        let database_manager = DatabaseManager {
            dev: DBC::<Data>::set_database_data(&configuration.dev_database_url).await?,
            prod: DBC::<Data>::set_database_data(&configuration.prod_database_url).await?,
        };

        let database_manager = DBC::<Cache>::set_database_cache(database_manager).await;

        match environment {
            Environments::PROD => Ok(database_manager.prod),
            _ => Ok(database_manager.dev),
        }
    }
}
