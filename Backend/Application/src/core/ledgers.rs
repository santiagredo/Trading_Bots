use crate::{
    handler::Ledgers,
    logic,
    utils::{handle_user_err, Repository, Response},
};
use models::{
    entities::ledgers::Model,
    structs::{LedgerRequest, QueryOptions},
};

/* ======================================================
 * CRUD / DB
 * ======================================================
 */

impl<R> Ledgers<R>
where
    R: Repository<LedgerRequest, Model>,
{
    pub async fn insert(&self, req: LedgerRequest) -> Result<Model, Response> {
        logic::ledgers::validate_insert(&req).map_err(handle_user_err)?;
        self.repo.insert(req).await
    }

    pub async fn select(&self, req: LedgerRequest) -> Result<Option<Model>, Response> {
        self.repo.select(req).await
    }

    pub async fn select_many(
        &self,
        req: LedgerRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        self.repo.select_many(req, query).await
    }

    pub async fn update(&self, req: LedgerRequest) -> Result<Model, Response> {
        // logic::ledgers::validate_update(&req).map_err(handle_user_err)?;
        self.repo.update(req).await
    }

    pub async fn delete(&self, req: LedgerRequest) -> Result<u64, Response> {
        // logic::ledgers::validate_delete(&req).map_err(handle_user_err)?;
        self.repo.delete(req).await
    }
}
