use std::marker::PhantomData;

use models::structs::Environments;
use sea_orm::DatabaseConnection;

use crate::utils::{Core, Response, Types};

pub struct DBC<Phase = Types> {
    phase: PhantomData<Phase>,
}

impl DBC {
    pub async fn db(environment: &Environments) -> Result<DatabaseConnection, Response> {
        DBC::<Core>::get_database_core(environment).await
    }
}
