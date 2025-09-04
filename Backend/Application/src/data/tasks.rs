use models::entities::tasks::{Column, Entity, Model};
use sea_orm::{Condition, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use tracing::error_span;

use crate::{
    handler::Tasks,
    utils::{handle_db_error, Data, Response},
};

impl Tasks<Data> {
    pub async fn select_tasks_data(self, db: &DatabaseConnection) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = self.model.id {
            condition = condition.add(Column::Id.eq(id))
        }

        if let Some(nick) = self.model.nick {
            condition = condition.add(Column::Nick.eq(nick))
        }

        if let Some(description) = self.model.description {
            condition = condition.add(Column::Description.eq(description))
        }

        if let Some(is_active) = self.model.is_active {
            condition = condition.add(Column::IsActive.eq(is_active))
        }

        if let Some(cooldown) = self.model.cooldown {
            condition = condition.add(Column::Cooldown.eq(cooldown))
        }

        if let Some(delay) = self.model.delay {
            condition = condition.add(Column::Delay.eq(delay))
        }

        match Entity::find().filter(condition).all(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(handle_db_error(&err));
            }
            Ok(val) => Ok(val),
        }
    }
}
