use crate::utils::handle_db_error;
use migration::async_trait::async_trait;
use models::entities::status::{ActiveModel, Column, Entity, Model};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, Condition, EntityTrait, QueryFilter};

use crate::utils::{DbRepo, Delete, Insert, MockRepo, Response, Select, Update};

//
// ==========================
// DbRepo IMPLEMENTATIONS
// ==========================
//

#[async_trait]
impl Insert<Model, Model> for DbRepo {
    async fn insert(&self, req: Model) -> Result<Model, Response> {
        let active_model = ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(req.name),
        };

        match active_model.insert(&self.data).await {
            Err(err) => Err(handle_db_error(&err)),
            Ok(val) => Ok(val),
        }
    }
}

#[async_trait]
impl Select<Model, Model> for DbRepo {
    async fn select(&self, req: Model) -> Result<Option<Model>, Response> {
        let mut condition = Condition::all();

        if req.id != 0 {
            condition = condition.add(Column::Id.eq(req.id));
        }

        match Entity::find().filter(condition).one(&self.data).await {
            Err(err) => Err(handle_db_error(&err)),
            Ok(val) => Ok(val),
        }
    }

    async fn select_many(
        &self,
        _req: Model,
        _query: Option<models::structs::QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        match Entity::find().all(&self.data).await {
            Err(err) => Err(handle_db_error(&err)),
            Ok(val) => Ok(val),
        }
    }
}

#[async_trait]
impl Update<Model, Model> for DbRepo {
    async fn update(&self, req: Model) -> Result<Model, Response> {
        let mut active_model = ActiveModel {
            id: ActiveValue::Unchanged(req.id),
            ..Default::default()
        };

        if !req.name.is_empty() {
            active_model.name = ActiveValue::Set(req.name);
        }

        match active_model.update(&self.data).await {
            Err(err) => Err(handle_db_error(&err)),
            Ok(val) => Ok(val),
        }
    }
}

#[async_trait]
impl Delete<Model> for DbRepo {
    async fn delete(&self, req: Model) -> Result<u64, Response> {
        match Entity::delete_by_id(req.id).exec(&self.data).await {
            Err(err) => Err(handle_db_error(&err)),
            Ok(val) => Ok(val.rows_affected),
        }
    }
}

//
// ==========================
// MockRepo IMPLEMENTATIONS
// ==========================
//

#[async_trait]
impl Insert<Model, Model> for MockRepo<Model> {
    async fn insert(&self, req: Model) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();
        data.push(req.clone());
        Ok(req)
    }
}

#[async_trait]
impl Select<Model, Model> for MockRepo<Model> {
    async fn select(&self, req: Model) -> Result<Option<Model>, Response> {
        let data = self.data.lock().unwrap();

        Ok(data.iter().find(|m| m.id == req.id).cloned())
    }

    async fn select_many(
        &self,
        _req: Model,
        _query: Option<models::structs::QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let data = self.data.lock().unwrap();
        Ok(data.clone())
    }
}

#[async_trait]
impl Update<Model, Model> for MockRepo<Model> {
    async fn update(&self, req: Model) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();

        let model = data
            .iter_mut()
            .find(|m| m.id == req.id)
            .ok_or(Response::not_found("Invalid mock update id".to_string()))?;

        if !req.name.is_empty() {
            model.name = req.name;
        }

        Ok(model.clone())
    }
}

#[async_trait]
impl Delete<Model> for MockRepo<Model> {
    async fn delete(&self, req: Model) -> Result<u64, Response> {
        let mut data = self.data.lock().unwrap();

        let before = data.len();
        data.retain(|m| m.id != req.id);

        Ok((before - data.len()) as u64)
    }
}
