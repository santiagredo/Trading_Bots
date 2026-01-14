use chrono::Local;
use function_name::named;
use models::{
    entities::integrations::{ActiveModel, Column, Entity, Model},
    enums::OrderDirection,
    structs::{ErrorLogRequest, QueryOptions},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter, QueryOrder, QuerySelect,
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
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = self.model.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(name) = self.model.name.clone() {
            condition = condition.add(Column::Name.eq(name));
        }

        if let Some(code) = self.model.code.clone() {
            condition = condition.add(Column::Code.eq(code));
        }

        if let Some(is_enabled) = self.model.is_enabled {
            condition = condition.add(Column::IsEnabled.eq(is_enabled));
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
            "name" => Some(Column::Name),
            "code" => Some(Column::Code),
            "is_enabled" => Some(Column::IsEnabled),
            "creation_date" => Some(Column::CreationDate),
            "last_update_date" => Some(Column::LastUpdateDate),
            _ => None,
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
