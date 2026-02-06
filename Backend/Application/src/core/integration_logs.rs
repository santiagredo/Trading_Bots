use crate::{
    handler::IntegrationLogs,
    utils::{Repository, Response},
};
use models::{
    entities::integration_log::Model,
    structs::{IntegrationLogRequest, QueryOptions},
};

impl<R> IntegrationLogs<R>
where
    R: Repository<IntegrationLogRequest, Model>,
{
    pub async fn insert(&self, req: IntegrationLogRequest) -> Result<Model, Response> {
        self.repo.insert(req).await
    }

    pub async fn select(&self, req: IntegrationLogRequest) -> Result<Option<Model>, Response> {
        self.repo.select(req).await
    }

    pub async fn select_many(
        &self,
        req: IntegrationLogRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        self.repo.select_many(req, query).await
    }

    pub async fn delete(&self, req: IntegrationLogRequest) -> Result<u64, Response> {
        self.repo.delete(req).await
    }
}
