use chrono::Local;
use function_name::named;
use models::{
    entities::integration_settings::{ActiveModel, Column, Entity, Model},
    structs::ErrorLogRequest,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter,
};

use crate::{
    handler::{ErrorLogs, IntegrationsSettings},
    log_db_error,
    utils::{Data, Response},
};

impl IntegrationsSettings<Data> {
    #[named]
    pub async fn select_integrations_settings_data(
        self,
        db: &DatabaseConnection,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if self.model.id.is_some() {
            condition = condition.add(Column::Id.eq(self.model.id.unwrap_or_default()));
        }

        if self.model.integration_id.is_some() {
            condition = condition
                .add(Column::IntegrationId.eq(self.model.integration_id.unwrap_or_default()));
        }

        if self.model.name.as_ref().is_some() {
            condition = condition.add(Column::Name.eq(self.model.name.clone().unwrap_or_default()));
        }

        if self.model.nick.as_ref().is_some() {
            condition = condition.add(Column::Nick.eq(self.model.nick.clone().unwrap_or_default()));
        }

        match Entity::find().filter(condition).all(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    #[named]
    pub async fn update_integration_setting_data(
        self,
        db: &DatabaseConnection,
    ) -> Result<Model, Response> {
        let mut active_model_integration_setting = ActiveModel {
            id: ActiveValue::Unchanged(self.model.id.unwrap_or_default()),
            last_update_date: ActiveValue::Set(Local::now().naive_local().into()),
            ..Default::default()
        };

        if let Some(value) = self.model.value.as_ref() {
            active_model_integration_setting.value = ActiveValue::Set(value.clone());
        }

        match active_model_integration_setting.update(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }
}
