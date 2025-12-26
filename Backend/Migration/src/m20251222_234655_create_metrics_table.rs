use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CriticalMetrics::Table)
                    .if_not_exists()
                    .col(pk_auto(CriticalMetrics::Id))
                    .col(date_time(CriticalMetrics::CreationDate))
                    // Counters
                    .col(big_unsigned(CriticalMetrics::ExecutionsOk))
                    .col(big_unsigned(CriticalMetrics::ExecutionsErr))
                    // Durations (stored as integer: ms/ns)
                    .col(big_unsigned(CriticalMetrics::TotalExecutionTime))
                    .col(big_unsigned(CriticalMetrics::MaxExecutionTime))
                    .col(big_unsigned(CriticalMetrics::SlowestDuration))
                    // Posting metrics
                    .col(big_unsigned(CriticalMetrics::ActivePosting))
                    .col(big_unsigned(CriticalMetrics::MaxActivePosting))
                    // Locks
                    .col(big_unsigned(CriticalMetrics::SkippedDueToLock))
                    // Dates
                    .col(
                        ColumnDef::new(CriticalMetrics::LastSuccess)
                            .date_time()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CriticalMetrics::LastError)
                            .date_time()
                            .null(),
                    )
                    // Errors
                    .col(big_unsigned(CriticalMetrics::ConsecutiveErrors))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(CriticalMetrics::Table)
                    .cascade()
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum CriticalMetrics {
    Table,
    Id,
    CreationDate,

    ExecutionsOk,
    ExecutionsErr,

    TotalExecutionTime,
    MaxExecutionTime,
    SlowestDuration,

    ActivePosting,
    MaxActivePosting,

    SkippedDueToLock,

    LastSuccess,
    LastError,

    ConsecutiveErrors,
}
