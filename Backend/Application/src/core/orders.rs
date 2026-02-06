use crate::{
    handler::Orders,
    logic,
    utils::{handle_user_err, Repository, Response},
};

use models::{
    entities::orders::Model,
    structs::{OrderRequest, QueryOptions},
};

/* ======================================================
 * CRUD / DB
 * ======================================================
 */

impl<R> Orders<R>
where
    R: Repository<OrderRequest, Model>,
{
    pub async fn insert(&self, req: OrderRequest) -> Result<Model, Response> {
        logic::orders::validate_insert(&req).map_err(handle_user_err)?;
        self.repo.insert(req).await
    }

    pub async fn select(&self, req: OrderRequest) -> Result<Option<Model>, Response> {
        logic::orders::validate_select(&req).map_err(handle_user_err)?;
        self.repo.select(req).await
    }

    pub async fn select_many(
        &self,
        req: OrderRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        self.repo.select_many(req, query).await
    }

    pub async fn update(&self, req: OrderRequest) -> Result<Model, Response> {
        logic::orders::validate_update(&req).map_err(handle_user_err)?;
        self.repo.update(req).await
    }

    pub async fn delete(&self, req: OrderRequest) -> Result<u64, Response> {
        // logic::orders::validate_delete(&req).map_err(handle_user_err)?;
        self.repo.delete(req).await
    }
}
