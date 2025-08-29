use models::entities::assets::{self, ActiveModel, Column, Entity, Model};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter,
};
use tracing::error_span;

use crate::{
    types::Assets,
    utils::{handle_db_error, Data, Response},
};

impl Assets<Data> {
    pub async fn insert_asset_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        let active_model_asset = ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(self.model.name.unwrap_or_default()),
            ticker: ActiveValue::Set(self.model.ticker.unwrap_or_default()),
            free: ActiveValue::Set(self.model.free.unwrap_or_default()),
            locked: ActiveValue::Set(self.model.locked.unwrap_or_default()),
        };

        match Entity::insert(active_model_asset)
            .exec_with_returning(db)
            .await
        {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(handle_db_error(&err));
            }
            Ok(val) => Ok(val),
        }
    }

    pub async fn select_asset_data(
        self,
        db: &DatabaseConnection,
    ) -> Result<Option<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = self.model.id {
            condition = condition.add(Column::Id.eq(id))
        }

        if let Some(name) = self.model.name {
            condition = condition.add(Column::Name.eq(name))
        }

        if let Some(ticker) = self.model.ticker {
            condition = condition.add(Column::Name.eq(ticker))
        }

        match Entity::find().filter(condition).one(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(handle_db_error(&err));
            }
            Ok(val) => Ok(val),
        }
    }

    pub async fn select_assets_data(self, db: &DatabaseConnection) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = self.model.id {
            condition = condition.add(Column::Id.eq(id))
        }

        if let Some(name) = self.model.name {
            condition = condition.add(Column::Name.eq(name))
        }

        if let Some(ticker) = self.model.ticker {
            condition = condition.add(Column::Name.eq(ticker))
        }

        match Entity::find().filter(condition).all(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(handle_db_error(&err));
            }
            Ok(val) => Ok(val),
        }
    }

    pub async fn update_asset_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        let mut active_model_asset = assets::ActiveModel {
            id: ActiveValue::Unchanged(self.model.id.unwrap_or_default()),
            ..Default::default()
        };

        if self.model.name.is_some() {
            active_model_asset.name = ActiveValue::Set(self.model.name.unwrap_or_default());
        }

        if self.model.ticker.is_some() {
            active_model_asset.ticker = ActiveValue::set(self.model.ticker.unwrap_or_default());
        }
        if self.model.free.is_some() {
            active_model_asset.free = ActiveValue::set(self.model.free.unwrap_or_default());
        }

        if self.model.locked.is_some() {
            active_model_asset.locked = ActiveValue::set(self.model.locked.unwrap_or_default());
        }

        match active_model_asset.update(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(handle_db_error(&err));
            }
            Ok(val) => Ok(val),
        }
    }

    pub async fn delete_asset_data(self, db: &DatabaseConnection) -> Result<u64, Response> {
        match Entity::delete_by_id(self.model.id.unwrap_or_default())
            .exec(db)
            .await
        {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(handle_db_error(&err));
            }
            Ok(val) => Ok(val.rows_affected),
        }
    }
}
