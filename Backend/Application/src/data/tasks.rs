use chrono::Local;
use function_name::named;
use models::{
    entities::tasks::{self, Column, Entity, Model},
    enums::OrderDirection,
    structs::{ErrorLogRequest, QueryOptions},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter, QueryOrder, QuerySelect,
};

use crate::{
    handler::{ErrorLogs, Tasks},
    log_db_error,
    utils::{Data, Response},
};

impl Tasks<Data> {
    #[named]
    pub async fn select_tasks_data(
        self,
        db: &DatabaseConnection,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = self.model.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(nick) = self.model.nick.as_ref() {
            condition = condition.add(Column::Nick.eq(nick.clone()));
        }

        if let Some(description) = self.model.description.as_ref() {
            condition = condition.add(Column::Description.eq(description.clone()));
        }

        if let Some(is_active) = self.model.is_active {
            condition = condition.add(Column::IsActive.eq(is_active));
        }

        if let Some(cooldown) = self.model.cooldown {
            condition = condition.add(Column::Cooldown.eq(cooldown));
        }

        if let Some(delay) = self.model.delay {
            condition = condition.add(Column::Delay.eq(delay));
        }

        let mut stmt = Entity::find().filter(condition);

        stmt = stmt.order_by(Column::Id, sea_orm::Order::Asc);

        if let Some(q) = query {
            if let Some(limit) = q.limit {
                stmt = stmt.limit(limit);
            }

            if let Some(offset) = q.offset {
                stmt = stmt.offset(offset);
            }

            if let Some(order_by) = q.order_by.as_deref().and_then(Self::parse_order_column) {
                let direction = match q.order_direction.unwrap_or(OrderDirection::Desc) {
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
            "nick" => Some(Column::Nick),
            "is_active" => Some(Column::IsActive),
            "cooldown" => Some(Column::Cooldown),
            "delay" => Some(Column::Delay),
            "last_update" => Some(Column::LastUpdate),
            "last_execution" => Some(Column::LastExecution),
            _ => None,
        }
    }

    #[named]
    pub async fn update_task_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        let mut task = tasks::ActiveModel {
            id: ActiveValue::Unchanged(self.model.id.unwrap_or_default()),
            ..Default::default()
        };

        if let Some(is_active) = self.model.is_active {
            task.is_active = ActiveValue::Set(is_active);
        }

        if let Some(cooldown) = self.model.cooldown {
            task.cooldown = ActiveValue::Set(cooldown.into());
        }

        if let Some(delay) = self.model.delay {
            task.delay = ActiveValue::Set(delay.into());
        }

        if let Some(last_update) = self.model.last_update {
            task.last_update = ActiveValue::Set(last_update.into())
        } else {
            task.last_update = ActiveValue::Set(Local::now().naive_local().into())
        }

        if let Some(last_execution) = self.model.last_execution {
            task.last_execution = ActiveValue::Set(last_execution.into())
        }

        match task.update(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }
}
