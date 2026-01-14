use chrono::Local;
use function_name::named;
use models::{
    entities::ledgers::{ActiveModel, Column, Entity, Model},
    enums::OrderDirection,
    structs::{ErrorLogRequest, QueryOptions},
};
use sea_orm::{
    ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect,
};

use crate::{
    handler::{ErrorLogs, Ledgers},
    log_db_error,
    utils::{Data, Response},
};

impl Ledgers<Data> {
    #[named]
    pub async fn insert_ledger_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        let active_model_ledger = ActiveModel {
            id: ActiveValue::NotSet,
            order_id: ActiveValue::Set(self.model.order_id),
            record_type_id: ActiveValue::Set(self.model.record_type_id.unwrap_or_default()),
            creation_date: ActiveValue::Set(Local::now().naive_local()),
            asset_id: ActiveValue::Set(self.model.asset_id.unwrap_or_default()),
            free_amount: ActiveValue::Set(self.model.free_amount.unwrap_or_default()),
            free_previous_balance: ActiveValue::Set(
                self.model.free_previous_balance.unwrap_or_default(),
            ),
            free_new_balance: ActiveValue::Set(self.model.free_new_balance.unwrap_or_default()),
            locked_amount: ActiveValue::Set(self.model.locked_amount.unwrap_or_default()),
            locked_previous_balance: ActiveValue::Set(
                self.model.locked_previous_balance.unwrap_or_default(),
            ),
            locked_new_balance: ActiveValue::Set(self.model.locked_new_balance.unwrap_or_default()),
        };

        match Entity::insert(active_model_ledger)
            .exec_with_returning(db)
            .await
        {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    #[named]
    pub async fn select_ledger_data(
        self,
        db: &DatabaseConnection,
    ) -> Result<Option<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = self.model.id {
            condition = condition.add(Column::Id.eq(id))
        }

        if let Some(order_id) = self.model.order_id {
            condition = condition.add(Column::OrderId.eq(order_id))
        }

        if let Some(record_type_id) = self.model.record_type_id {
            condition = condition.add(Column::RecordTypeId.eq(record_type_id))
        }

        if let Some(creation_date) = self.model.creation_date {
            condition = condition.add(Column::CreationDate.eq(creation_date))
        }

        if let Some(asset_id) = self.model.asset_id {
            condition = condition.add(Column::AssetId.eq(asset_id))
        }

        match Entity::find()
            .filter(condition)
            .order_by(Column::CreationDate, sea_orm::Order::Desc)
            .one(db)
            .await
        {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    #[named]
    pub async fn select_ledgers_data(
        self,
        db: &DatabaseConnection,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = self.model.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(order_id) = self.model.order_id {
            condition = condition.add(Column::OrderId.eq(order_id));
        }

        if let Some(record_type_id) = self.model.record_type_id {
            condition = condition.add(Column::RecordTypeId.eq(record_type_id));
        }

        if let Some(creation_date) = self.model.creation_date {
            condition = condition.add(Column::CreationDate.eq(creation_date));
        }

        if let Some(asset_id) = self.model.asset_id {
            condition = condition.add(Column::AssetId.eq(asset_id));
        }

        let mut stmt = Entity::find().filter(condition);

        stmt = stmt.order_by(Column::Id, sea_orm::Order::Desc);

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
            "order_id" => Some(Column::OrderId),
            "record_type_id" => Some(Column::RecordTypeId),
            "creation_date" => Some(Column::CreationDate),
            "asset_id" => Some(Column::AssetId),
            "free_amount" => Some(Column::FreeAmount),
            "free_previous_balance" => Some(Column::FreePreviousBalance),
            "free_new_balance" => Some(Column::FreeNewBalance),
            "locked_amount" => Some(Column::LockedAmount),
            "locked_previous_balance" => Some(Column::LockedPreviousBalance),
            "locked_new_balance" => Some(Column::LockedNewBalance),
            _ => None,
        }
    }
}
