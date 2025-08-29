use crate::m20250121_024310_create_assets_table::Assets;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Pairs::Table)
                    .if_not_exists()
                    .col(pk_auto(Pairs::Id))
                    .col(integer(Pairs::BaseAssetId))
                    .col(integer(Pairs::QuoteAssetId))
                    .col(text(Pairs::Symbol))
                    .col(date_time(Pairs::UpdateDate).default(Expr::current_timestamp()))
                    .col(decimal(Pairs::AllTimeHighPrice).default(0))
                    .col(date_time(Pairs::AllTimeHighDate).default(Expr::current_timestamp()))
                    .col(decimal(Pairs::PercentFromAllTimeHigh).default(0))
                    .col(decimal(Pairs::FifteenMinutesPricePercentChange).default(0))
                    .col(decimal(Pairs::ThirtyMinutesPricePercentChange).default(0))
                    .col(decimal(Pairs::HourPricePercentChange).default(0))
                    .col(decimal(Pairs::SixHoursPricePercentChange).default(0))
                    .col(decimal(Pairs::TwelveHoursPricePercentChange).default(0))
                    .col(decimal(Pairs::DayPricePercentChange).default(0))
                    .col(decimal(Pairs::WeekPricePercentChange).default(0))
                    .col(decimal(Pairs::MonthPricePercentChange).default(0))
                    .col(decimal(Pairs::YearPricePercentChange).default(0))
                    .col(decimal(Pairs::PriceFilterMinPrice).default(0))
                    .col(decimal(Pairs::PriceFilterMaxPrice).default(0))
                    .col(decimal(Pairs::PriceFilterTickSize).default(0))
                    .col(decimal(Pairs::LotSizeMinQty).default(0))
                    .col(decimal(Pairs::LotSizeMaxQty).default(0))
                    .col(decimal(Pairs::LotSizeStepSize).default(0))
                    .col(integer(Pairs::IcebergPartsLimit).default(0))
                    .col(decimal(Pairs::MarketLotSizeMinQty).default(0))
                    .col(decimal(Pairs::MarketLotSizeMaxQty).default(0))
                    .col(decimal(Pairs::MarketLotSizeStepSize).default(0))
                    .col(integer(Pairs::TrailingDeltaMinTrailingAboveDelta).default(0))
                    .col(integer(Pairs::TrailingDeltaMaxTrailingAboveDelta).default(0))
                    .col(integer(Pairs::TrailingDeltaMinTrailingBelowDelta).default(0))
                    .col(integer(Pairs::TrailingDeltaMaxTrailingBelowDelta).default(0))
                    .col(decimal(Pairs::PercentPriceBySideBidMultiplierUp).default(0))
                    .col(decimal(Pairs::PercentPriceBySideBidMultiplierDown).default(0))
                    .col(decimal(Pairs::PercentPriceBySideAskMultiplierUp).default(0))
                    .col(decimal(Pairs::PercentPriceBySideAskMultiplierDown).default(0))
                    .col(integer(Pairs::PercentPriceBySideAvgPriceMins).default(0))
                    .col(decimal(Pairs::NotionalMinNotional).default(0))
                    .col(boolean(Pairs::NotionalApplyMinToMarket).default(false))
                    .col(decimal(Pairs::NotionalMaxNotional).default(0))
                    .col(boolean(Pairs::NotionalApplyMaxToMarket).default(false))
                    .col(integer(Pairs::NotionalAvgPriceMins).default(0))
                    .col(integer(Pairs::MaxNumOrders).default(0))
                    .col(integer(Pairs::MaxNumAlgoOrders).default(0))
                    .to_owned(),
            )
            .await?;

        // Foreign keys
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_pairassets_assets_base")
                    .from(Pairs::Table, Pairs::BaseAssetId)
                    .to(Assets::Table, Assets::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_pairassets_assets_quote")
                    .from(Pairs::Table, Pairs::QuoteAssetId)
                    .to(Assets::Table, Assets::Id)
                    .to_owned(),
            )
            .await?;

        // Example insert
        let insert_pairs = Query::insert()
            .into_table(Pairs::Table)
            .columns([
                Pairs::BaseAssetId,
                Pairs::QuoteAssetId,
                Pairs::Symbol,
            ])
            .values_panic([3.into(), 2.into(), "BTCUSDT".into()])
            .to_owned();

        manager.exec_stmt(insert_pairs).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Pairs::Table).cascade().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Pairs {
    Table,
    Id,
    BaseAssetId,
    QuoteAssetId,
    Symbol,
    UpdateDate,
    AllTimeHighPrice,
    AllTimeHighDate,
    PercentFromAllTimeHigh,
    FifteenMinutesPricePercentChange,
    ThirtyMinutesPricePercentChange,
    HourPricePercentChange,
    SixHoursPricePercentChange,
    TwelveHoursPricePercentChange,
    DayPricePercentChange,
    WeekPricePercentChange,
    MonthPricePercentChange,
    YearPricePercentChange,
    PriceFilterMinPrice,
    PriceFilterMaxPrice,
    PriceFilterTickSize,
    LotSizeMinQty,
    LotSizeMaxQty,
    LotSizeStepSize,
    IcebergPartsLimit,
    MarketLotSizeMinQty,
    MarketLotSizeMaxQty,
    MarketLotSizeStepSize,
    TrailingDeltaMinTrailingAboveDelta,
    TrailingDeltaMaxTrailingAboveDelta,
    TrailingDeltaMinTrailingBelowDelta,
    TrailingDeltaMaxTrailingBelowDelta,
    PercentPriceBySideBidMultiplierUp,
    PercentPriceBySideBidMultiplierDown,
    PercentPriceBySideAskMultiplierUp,
    PercentPriceBySideAskMultiplierDown,
    PercentPriceBySideAvgPriceMins,
    NotionalMinNotional,
    NotionalApplyMinToMarket,
    NotionalMaxNotional,
    NotionalApplyMaxToMarket,
    NotionalAvgPriceMins,
    MaxNumOrders,
    MaxNumAlgoOrders,
}
