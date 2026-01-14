use chrono::Local;
use function_name::named;
use models::{
    entities::integrations::{ActiveModel, Column, Entity, Model},
    structs::ErrorLogRequest,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter,
};

use crate::{
    handler::{ErrorLogs, Integrations},
    log_db_error,
    utils::{Data, Response},
};

impl Integrations<Data> {
    #[named]
    pub async fn select_integrations_data(
        self,
        db: &DatabaseConnection,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if self.model.id.is_some() {
            condition = condition.add(Column::Id.eq(self.model.id.unwrap_or_default()));
        }

        if self.model.name.as_ref().is_some() {
            condition = condition.add(Column::Name.eq(self.model.name.clone().unwrap_or_default()));
        }

        if self.model.code.as_ref().is_some() {
            condition = condition.add(Column::Code.eq(self.model.code.clone().unwrap_or_default()));
        }

        if self.model.is_enabled.is_some() {
            condition =
                condition.add(Column::IsEnabled.eq(self.model.is_enabled.unwrap_or_default()));
        }

        match Entity::find().filter(condition).all(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    #[named]
    pub async fn update_integration_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        let mut active_model_integration = ActiveModel {
            id: ActiveValue::Unchanged(self.model.id.unwrap_or_default()),
            last_update_date: ActiveValue::Set(Local::now().naive_local().into()),
            ..Default::default()
        };

        if let Some(is_enabled) = self.model.is_enabled {
            active_model_integration.is_enabled = ActiveValue::Set(is_enabled);
        }

        match active_model_integration.update(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }
}
