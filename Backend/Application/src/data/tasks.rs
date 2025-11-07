use function_name::named;
use models::{
    entities::tasks::{Column, Entity, Model},
    structs::ErrorLogRequest,
};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    handler::{ErrorLogs, Tasks},
    log_db_error,
    utils::{Data, Response},
};

impl Tasks<Data> {
    #[named]
    pub async fn select_tasks_data(self, db: &DatabaseConnection) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = self.model.id {
            condition = condition.add(Column::Id.eq(id))
        }

        if let Some(nick) = self.model.nick.as_ref() {
            condition = condition.add(Column::Nick.eq(nick.clone()))
        }

        if let Some(description) = self.model.description.as_ref() {
            condition = condition.add(Column::Description.eq(description.clone()))
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
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }
}
