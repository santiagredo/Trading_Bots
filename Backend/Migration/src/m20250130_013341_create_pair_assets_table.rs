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
                    .table(PairAssets::Table)
                    .if_not_exists()
                    .col(pk_auto(PairAssets::Id))
                    .col(integer(PairAssets::BaseAssetId))
                    .col(integer(PairAssets::QuoteAssetId))
                    .col(text(PairAssets::Symbol))
                    .col(date_time(PairAssets::UpdateDate).default(Expr::current_timestamp()))
                    .col(decimal(PairAssets::AllTimeHighPrice).default(0))
                    .col(date_time(PairAssets::AllTimeHighDate).default(Expr::current_timestamp()))
                    .col(decimal(PairAssets::PercentFromAllTimeHigh).default(0))
                    .col(decimal(PairAssets::FifteenMinutesPricePercentChange).default(0))
                    .col(decimal(PairAssets::ThirtyMinutesPricePercentChange).default(0))
                    .col(decimal(PairAssets::HourPricePercentChange).default(0))
                    .col(decimal(PairAssets::SixHoursPricePercentChange).default(0))
                    .col(decimal(PairAssets::TwelveHoursPricePercentChange).default(0))
                    .col(decimal(PairAssets::DayPricePercentChange).default(0))
                    .col(decimal(PairAssets::WeekPricePercentChange).default(0))
                    .col(decimal(PairAssets::MonthPricePercentChange).default(0))
                    .col(decimal(PairAssets::YearPricePercentChange).default(0))
                    .col(decimal(PairAssets::PriceFilterMinPrice).default(0))
                    .col(decimal(PairAssets::PriceFilterMaxPrice).default(0))
                    .col(decimal(PairAssets::PriceFilterTickSize).default(0))
                    .col(decimal(PairAssets::LotSizeMinQty).default(0))
                    .col(decimal(PairAssets::LotSizeMaxQty).default(0))
                    .col(decimal(PairAssets::LotSizeStepSize).default(0))
                    .col(integer(PairAssets::IcebergPartsLimit).default(0))
                    .col(decimal(PairAssets::MarketLotSizeMinQty).default(0))
                    .col(decimal(PairAssets::MarketLotSizeMaxQty).default(0))
                    .col(decimal(PairAssets::MarketLotSizeStepSize).default(0))
                    .col(integer(PairAssets::TrailingDeltaMinTrailingAboveDelta).default(0))
                    .col(integer(PairAssets::TrailingDeltaMaxTrailingAboveDelta).default(0))
                    .col(integer(PairAssets::TrailingDeltaMinTrailingBelowDelta).default(0))
                    .col(integer(PairAssets::TrailingDeltaMaxTrailingBelowDelta).default(0))
                    .col(decimal(PairAssets::PercentPriceBySideBidMultiplierUp).default(0))
                    .col(decimal(PairAssets::PercentPriceBySideBidMultiplierDown).default(0))
                    .col(decimal(PairAssets::PercentPriceBySideAskMultiplierUp).default(0))
                    .col(decimal(PairAssets::PercentPriceBySideAskMultiplierDown).default(0))
                    .col(integer(PairAssets::PercentPriceBySideAvgPriceMins).default(0))
                    .col(decimal(PairAssets::NotionalMinNotional).default(0))
                    .col(boolean(PairAssets::NotionalApplyMinToMarket).default(false))
                    .col(decimal(PairAssets::NotionalMaxNotional).default(0))
                    .col(boolean(PairAssets::NotionalApplyMaxToMarket).default(false))
                    .col(integer(PairAssets::NotionalAvgPriceMins).default(0))
                    .col(integer(PairAssets::MaxNumOrders).default(0))
                    .col(integer(PairAssets::MaxNumAlgoOrders).default(0))
                    .to_owned(),
            )
            .await?;

        // Foreign keys
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_pairassets_assets_base")
                    .from(PairAssets::Table, PairAssets::BaseAssetId)
                    .to(Assets::Table, Assets::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_pairassets_assets_quote")
                    .from(PairAssets::Table, PairAssets::QuoteAssetId)
                    .to(Assets::Table, Assets::Id)
                    .to_owned(),
            )
            .await?;

        // Example insert
        let insert_pair_assets = Query::insert()
            .into_table(PairAssets::Table)
            .columns([
                PairAssets::BaseAssetId,
                PairAssets::QuoteAssetId,
                PairAssets::Symbol,
            ])
            .values_panic([3.into(), 2.into(), "BTCUSDT".into()])
            .to_owned();

        manager.exec_stmt(insert_pair_assets).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PairAssets::Table).cascade().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PairAssets {
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
