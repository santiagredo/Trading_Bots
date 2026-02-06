use crate::{
    handler::ErrorLogs,
    utils::{Repository, Response},
};
use models::{
    entities::error_log::Model,
    structs::{ErrorLogRequest, QueryOptions},
};

impl<R> ErrorLogs<R>
where
    R: Repository<ErrorLogRequest, Model>,
{
    pub async fn insert(self, req: ErrorLogRequest) -> Result<Model, Response> {
        self.repo.insert(req).await
    }

    pub async fn select(self, req: ErrorLogRequest) -> Result<Option<Model>, Response> {
        self.repo.select(req).await
    }

    pub async fn select_many(
        self,
        req: ErrorLogRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        self.repo.select_many(req, query).await
    }

    pub async fn delete(self, req: ErrorLogRequest) -> Result<u64, Response> {
        self.repo.delete(req).await
    }
}
