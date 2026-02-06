use crate::utils::handle_db_error;
use crate::{handler::Tasks, utils::Response};
use chrono::Local;
use models::structs::TaskRequest;
use models::{
    entities::tasks::{self, Column, Entity, Model},
    enums::OrderDirection,
    structs::QueryOptions,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter, QueryOrder, QuerySelect,
};

impl Tasks {
    // #[named]
    pub async fn select_tasks_data(
        self,
        db: &DatabaseConnection,
        task: TaskRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = task.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(nick) = task.nick.as_ref() {
            condition = condition.add(Column::Nick.eq(nick.clone()));
        }

        if let Some(description) = task.description.as_ref() {
            condition = condition.add(Column::Description.eq(description.clone()));
        }

        if let Some(is_active) = task.is_active {
            condition = condition.add(Column::IsActive.eq(is_active));
        }

        if let Some(cooldown) = task.cooldown {
            condition = condition.add(Column::Cooldown.eq(cooldown));
        }

        if let Some(delay) = task.delay {
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
            Err(err) => {
                // let _ = ErrorLogs::new(self.clone())
                //     .insert(log_trait_db_error!(err, req))
                //     .await;

                Err(handle_db_error(&err))
            }
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

    // #[named]
    pub async fn update_task_data(
        self,
        db: &DatabaseConnection,
        task: TaskRequest,
    ) -> Result<Model, Response> {
        let mut active_task = tasks::ActiveModel {
            id: ActiveValue::Unchanged(task.id.unwrap_or_default()),
            ..Default::default()
        };

        if let Some(is_active) = task.is_active {
            active_task.is_active = ActiveValue::Set(is_active);
        }

        if let Some(cooldown) = task.cooldown {
            active_task.cooldown = ActiveValue::Set(cooldown.into());
        }

        if let Some(delay) = task.delay {
            active_task.delay = ActiveValue::Set(delay.into());
        }

        if let Some(last_update) = task.last_update {
            active_task.last_update = ActiveValue::Set(last_update.into())
        } else {
            active_task.last_update = ActiveValue::Set(Local::now().naive_local().into())
        }

        if let Some(last_execution) = task.last_execution {
            active_task.last_execution = ActiveValue::Set(last_execution.into())
        }

        match active_task.update(db).await {
            Err(err) => {
                // let _ = ErrorLogs::new(self.clone())
                //     .insert(log_trait_db_error!(err, req))
                //     .await;

                Err(handle_db_error(&err))
            }
            Ok(val) => Ok(val),
        }
    }
}
