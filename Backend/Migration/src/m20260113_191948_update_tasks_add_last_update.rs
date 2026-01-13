use sea_orm_migration::prelude::*;

use crate::{
    m20250121_024310_create_assets_table::Assets, m20250904_011953_create_tasks_table::Tasks,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Tasks::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(TasksAlter::LastUpdate).date_time().null(),
                    )
                    .to_owned(),
            )
            .await
            .unwrap();

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Assets::Table)
                    .drop_column(TasksAlter::LastUpdate)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum TasksAlter {
    LastUpdate,
}
