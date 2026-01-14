use chrono::Local;
use function_name::named;
use models::{
    entities::integration_settings::{ActiveModel, Column, Entity, Model},
    enums::OrderDirection,
    structs::{ErrorLogRequest, QueryOptions},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter, QueryOrder, QuerySelect,
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
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = self.model.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(integration_id) = self.model.integration_id {
            condition = condition.add(Column::IntegrationId.eq(integration_id));
        }

        if let Some(name) = self.model.name.clone() {
            condition = condition.add(Column::Name.eq(name));
        }

        if let Some(nick) = self.model.nick.clone() {
            condition = condition.add(Column::Nick.eq(nick));
        }

        let mut stmt = Entity::find().filter(condition);

        if let Some(q) = query {
            if let Some(limit) = q.limit {
                stmt = stmt.limit(limit);
            }

            if let Some(offset) = q.offset {
                stmt = stmt.offset(offset);
            }

            if let Some(order_by) = q.order_by.and_then(|c| Self::parse_order_column(&c)) {
                let direction = match q.order_direction.unwrap_or(OrderDirection::Asc) {
                    OrderDirection::Asc => sea_orm::Order::Asc,
                    OrderDirection::Desc => sea_orm::Order::Desc,
                };

                stmt = stmt.order_by(order_by, direction);
            }
        }

        match stmt.all(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    fn parse_order_column(value: &str) -> Option<Column> {
        match value {
            "id" => Some(Column::Id),
            "integration_id" => Some(Column::IntegrationId),
            "name" => Some(Column::Name),
            "nick" => Some(Column::Nick),
            "value" => Some(Column::Value),
            "creation_date" => Some(Column::CreationDate),
            "last_update_date" => Some(Column::LastUpdateDate),
            _ => None,
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
