use crate::utils::Response;
use models::structs::Environments;
use sea_orm::DatabaseConnection;

pub struct DBC;

impl DBC {
    pub async fn db(environment: &Environments) -> Result<DatabaseConnection, Response> {
        DBC::get_database_core(environment).await
    }
}
