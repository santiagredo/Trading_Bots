use function_name::named;
use models::{
    entities::assets::{self, ActiveModel, Column, Entity, Model},
    structs::ErrorLogRequest,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter,
};

use crate::{
    handler::{Assets, ErrorLogs},
    log_db_error,
    utils::{Data, Response},
};

impl Assets<Data> {
    #[named]
    pub async fn insert_asset_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        let active_model_asset = ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(self.model.name.clone().unwrap_or_default()),
            ticker: ActiveValue::Set(self.model.ticker.clone().unwrap_or_default()),
            free: ActiveValue::Set(self.model.free.unwrap_or_default()),
            locked: ActiveValue::Set(self.model.locked.unwrap_or_default()),
        };

        match Entity::insert(active_model_asset)
            .exec_with_returning(db)
            .await
        {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    #[named]
    pub async fn select_asset_data(
        self,
        db: &DatabaseConnection,
    ) -> Result<Option<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = self.model.id {
            condition = condition.add(Column::Id.eq(id))
        }

        if let Some(name) = self.model.name.clone() {
            condition = condition.add(Column::Name.eq(name))
        }

        if let Some(ticker) = self.model.ticker.clone() {
            condition = condition.add(Column::Name.eq(ticker))
        }

        match Entity::find().filter(condition).one(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    #[named]
    pub async fn select_assets_data(self, db: &DatabaseConnection) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = self.model.id {
            condition = condition.add(Column::Id.eq(id))
        }

        if let Some(name) = self.model.name.clone() {
            condition = condition.add(Column::Name.eq(name))
        }

        if let Some(ticker) = self.model.ticker.clone() {
            condition = condition.add(Column::Name.eq(ticker))
        }

        match Entity::find().filter(condition).all(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    #[named]
    pub async fn update_asset_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        let mut active_model_asset = assets::ActiveModel {
            id: ActiveValue::Unchanged(self.model.id.unwrap_or_default()),
            ..Default::default()
        };

        if self.model.name.is_some() {
            active_model_asset.name = ActiveValue::Set(self.model.name.clone().unwrap_or_default());
        }

        if self.model.ticker.is_some() {
            active_model_asset.ticker =
                ActiveValue::set(self.model.ticker.clone().unwrap_or_default());
        }
        if self.model.free.is_some() {
            active_model_asset.free = ActiveValue::set(self.model.free.unwrap_or_default());
        }

        if self.model.locked.is_some() {
            active_model_asset.locked = ActiveValue::set(self.model.locked.unwrap_or_default());
        }

        match active_model_asset.update(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    #[named]
    pub async fn delete_asset_data(self, db: &DatabaseConnection) -> Result<u64, Response> {
        match Entity::delete_by_id(self.model.id.unwrap_or_default())
            .exec(db)
            .await
        {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val.rows_affected),
        }
    }
}
