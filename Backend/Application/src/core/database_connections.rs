use crate::{
    handler::{Configurations, DBC},
    utils::Response,
};
use models::structs::{DatabaseManager, Environments};
use sea_orm::DatabaseConnection;

impl DBC {
    pub async fn get_database_core(
        environment: &Environments,
    ) -> Result<DatabaseConnection, Response> {
        if let Some(database_manager) = DBC::get_database_cache().await {
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
            dev: DBC::set_database_data(&configuration.dev_database_url).await?,
            prod: DBC::set_database_data(&configuration.prod_database_url).await?,
        };

        let database_manager = DBC::set_database_cache(database_manager).await;

        match environment {
            Environments::PROD => Ok(database_manager.prod),
            _ => Ok(database_manager.dev),
        }
    }
}
