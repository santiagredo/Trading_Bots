use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInformation {
    pub timezone: String,

    pub server_time: i64,

    pub rate_limits: Vec<RateLimit>,

    pub exchange_filters: Vec<Option<serde_json::Value>>,

    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub rate_limit_type: String,

    pub interval: String,

    pub interval_num: i64,

    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,

    pub status: Status,

    pub base_asset: String,

    pub base_asset_precision: i64,

    pub quote_asset: QuoteAsset,

    pub quote_precision: i64,

    pub quote_asset_precision: i64,

    pub base_commission_precision: i64,

    pub quote_commission_precision: i64,

    pub order_types: Vec<OrderType>,

    pub iceberg_allowed: bool,

    pub oco_allowed: bool,

    pub oto_allowed: bool,

    pub quote_order_qty_market_allowed: bool,

    pub allow_trailing_stop: bool,

    pub cancel_replace_allowed: bool,

    pub is_spot_trading_allowed: bool,

    pub is_margin_trading_allowed: bool,

    pub filters: Vec<Filter>,

    pub permissions: Vec<Option<serde_json::Value>>,

    pub permission_sets: Vec<Vec<PermissionSet>>,

    pub default_self_trade_prevention_mode: SelfTradePreventionMode,

    pub allowed_self_trade_prevention_modes: Vec<SelfTradePreventionMode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SelfTradePreventionMode {
    #[serde(rename = "EXPIRE_BOTH")]
    ExpireBoth,

    #[serde(rename = "EXPIRE_MAKER")]
    ExpireMaker,

    #[serde(rename = "EXPIRE_TAKER")]
    ExpireTaker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub filter_type: FilterType,

    pub min_price: Option<Decimal>,

    pub max_price: Option<Decimal>,

    pub tick_size: Option<Decimal>,

    pub min_qty: Option<Decimal>,

    pub max_qty: Option<Decimal>,

    pub step_size: Option<Decimal>,

    pub limit: Option<i64>,

    pub min_trailing_above_delta: Option<i32>,

    pub max_trailing_above_delta: Option<i32>,

    pub min_trailing_below_delta: Option<i32>,

    pub max_trailing_below_delta: Option<i32>,

    pub bid_multiplier_up: Option<Decimal>,

    pub bid_multiplier_down: Option<Decimal>,

    pub ask_multiplier_up: Option<Decimal>,

    pub ask_multiplier_down: Option<Decimal>,

    pub avg_price_mins: Option<i32>,

    pub min_notional: Option<Decimal>,

    pub apply_min_to_market: Option<bool>,

    pub max_notional: Option<Decimal>,

    pub apply_max_to_market: Option<bool>,

    pub max_num_orders: Option<i64>,

    pub max_num_algo_orders: Option<i64>,

    pub max_position: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FilterType {
    #[serde(rename = "ICEBERG_PARTS")]
    IcebergParts,

    #[serde(rename = "LOT_SIZE")]
    LotSize,

    #[serde(rename = "MARKET_LOT_SIZE")]
    MarketLotSize,

    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    MaxNumAlgoOrders,

    #[serde(rename = "MAX_NUM_ORDERS")]
    MaxNumOrders,

    #[serde(rename = "MAX_POSITION")]
    MaxPosition,

    Notional,

    #[serde(rename = "PERCENT_PRICE_BY_SIDE")]
    PercentPriceBySide,

    #[serde(rename = "PRICE_FILTER")]
    PriceFilter,

    #[serde(rename = "TRAILING_DELTA")]
    TrailingDelta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,

    #[serde(rename = "LIMIT_MAKER")]
    LimitMaker,

    Market,

    #[serde(rename = "STOP_LOSS")]
    StopLoss,

    #[serde(rename = "STOP_LOSS_LIMIT")]
    StopLossLimit,

    #[serde(rename = "TAKE_PROFIT")]
    TakeProfit,

    #[serde(rename = "TAKE_PROFIT_LIMIT")]
    TakeProfitLimit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PermissionSet {
    Leveraged,

    Margin,

    #[serde(rename = "MARGIN_001")]
    Margin001,

    Spot,

    #[serde(rename = "TRD_GRP_004")]
    TrdGrp004,

    #[serde(rename = "TRD_GRP_005")]
    TrdGrp005,

    #[serde(rename = "TRD_GRP_006")]
    TrdGrp006,

    #[serde(rename = "TRD_GRP_008")]
    TrdGrp008,

    #[serde(rename = "TRD_GRP_009")]
    TrdGrp009,

    #[serde(rename = "TRD_GRP_010")]
    TrdGrp010,

    #[serde(rename = "TRD_GRP_011")]
    TrdGrp011,

    #[serde(rename = "TRD_GRP_012")]
    TrdGrp012,

    #[serde(rename = "TRD_GRP_013")]
    TrdGrp013,

    #[serde(rename = "TRD_GRP_014")]
    TrdGrp014,

    #[serde(rename = "TRD_GRP_015")]
    TrdGrp015,

    #[serde(rename = "TRD_GRP_016")]
    TrdGrp016,

    #[serde(rename = "TRD_GRP_017")]
    TrdGrp017,

    #[serde(rename = "TRD_GRP_018")]
    TrdGrp018,

    #[serde(rename = "TRD_GRP_019")]
    TrdGrp019,

    #[serde(rename = "TRD_GRP_020")]
    TrdGrp020,

    #[serde(rename = "TRD_GRP_021")]
    TrdGrp021,

    #[serde(rename = "TRD_GRP_022")]
    TrdGrp022,

    #[serde(rename = "TRD_GRP_023")]
    TrdGrp023,

    #[serde(rename = "TRD_GRP_024")]
    TrdGrp024,

    #[serde(rename = "TRD_GRP_025")]
    TrdGrp025,

    #[serde(rename = "TRD_GRP_026")]
    TrdGrp026,

    #[serde(rename = "TRD_GRP_027")]
    TrdGrp027,

    #[serde(rename = "TRD_GRP_028")]
    TrdGrp028,

    #[serde(rename = "TRD_GRP_029")]
    TrdGrp029,

    #[serde(rename = "TRD_GRP_030")]
    TrdGrp030,

    #[serde(rename = "TRD_GRP_031")]
    TrdGrp031,

    #[serde(rename = "TRD_GRP_032")]
    TrdGrp032,

    #[serde(rename = "TRD_GRP_033")]
    TrdGrp033,

    #[serde(rename = "TRD_GRP_034")]
    TrdGrp034,

    #[serde(rename = "TRD_GRP_035")]
    TrdGrp035,

    #[serde(rename = "TRD_GRP_036")]
    TrdGrp036,

    #[serde(rename = "TRD_GRP_037")]
    TrdGrp037,

    #[serde(rename = "TRD_GRP_038")]
    TrdGrp038,

    #[serde(rename = "TRD_GRP_039")]
    TrdGrp039,

    #[serde(rename = "TRD_GRP_040")]
    TrdGrp040,

    #[serde(rename = "TRD_GRP_041")]
    TrdGrp041,

    #[serde(rename = "TRD_GRP_042")]
    TrdGrp042,

    #[serde(rename = "TRD_GRP_043")]
    TrdGrp043,

    #[serde(rename = "TRD_GRP_044")]
    TrdGrp044,

    #[serde(rename = "TRD_GRP_045")]
    TrdGrp045,

    #[serde(rename = "TRD_GRP_046")]
    TrdGrp046,

    #[serde(rename = "TRD_GRP_047")]
    TrdGrp047,

    #[serde(rename = "TRD_GRP_048")]
    TrdGrp048,

    #[serde(rename = "TRD_GRP_049")]
    TrdGrp049,

    #[serde(rename = "TRD_GRP_050")]
    TrdGrp050,

    #[serde(rename = "TRD_GRP_051")]
    TrdGrp051,

    #[serde(rename = "TRD_GRP_052")]
    TrdGrp052,

    #[serde(rename = "TRD_GRP_053")]
    TrdGrp053,

    #[serde(rename = "TRD_GRP_054")]
    TrdGrp054,

    #[serde(rename = "TRD_GRP_055")]
    TrdGrp055,

    #[serde(rename = "TRD_GRP_056")]
    TrdGrp056,

    #[serde(rename = "TRD_GRP_057")]
    TrdGrp057,

    #[serde(rename = "TRD_GRP_058")]
    TrdGrp058,

    #[serde(rename = "TRD_GRP_059")]
    TrdGrp059,

    #[serde(rename = "TRD_GRP_060")]
    TrdGrp060,

    #[serde(rename = "TRD_GRP_061")]
    TrdGrp061,

    #[serde(rename = "TRD_GRP_062")]
    TrdGrp062,

    #[serde(rename = "TRD_GRP_063")]
    TrdGrp063,

    #[serde(rename = "TRD_GRP_064")]
    TrdGrp064,

    #[serde(rename = "TRD_GRP_065")]
    TrdGrp065,

    #[serde(rename = "TRD_GRP_066")]
    TrdGrp066,

    #[serde(rename = "TRD_GRP_067")]
    TrdGrp067,

    #[serde(rename = "TRD_GRP_068")]
    TrdGrp068,

    #[serde(rename = "TRD_GRP_069")]
    TrdGrp069,

    #[serde(rename = "TRD_GRP_070")]
    TrdGrp070,

    #[serde(rename = "TRD_GRP_071")]
    TrdGrp071,

    #[serde(rename = "TRD_GRP_072")]
    TrdGrp072,

    #[serde(rename = "TRD_GRP_073")]
    TrdGrp073,

    #[serde(rename = "TRD_GRP_074")]
    TrdGrp074,

    #[serde(rename = "TRD_GRP_075")]
    TrdGrp075,

    #[serde(rename = "TRD_GRP_076")]
    TrdGrp076,

    #[serde(rename = "TRD_GRP_077")]
    TrdGrp077,

    #[serde(rename = "TRD_GRP_078")]
    TrdGrp078,

    #[serde(rename = "TRD_GRP_079")]
    TrdGrp079,

    #[serde(rename = "TRD_GRP_080")]
    TrdGrp080,

    #[serde(rename = "TRD_GRP_081")]
    TrdGrp081,

    #[serde(rename = "TRD_GRP_082")]
    TrdGrp082,

    #[serde(rename = "TRD_GRP_083")]
    TrdGrp083,

    #[serde(rename = "TRD_GRP_084")]
    TrdGrp084,

    #[serde(rename = "TRD_GRP_085")]
    TrdGrp085,

    #[serde(rename = "TRD_GRP_086")]
    TrdGrp086,

    #[serde(rename = "TRD_GRP_087")]
    TrdGrp087,

    #[serde(rename = "TRD_GRP_088")]
    TrdGrp088,

    #[serde(rename = "TRD_GRP_089")]
    TrdGrp089,

    #[serde(rename = "TRD_GRP_090")]
    TrdGrp090,

    #[serde(rename = "TRD_GRP_091")]
    TrdGrp091,

    #[serde(rename = "TRD_GRP_092")]
    TrdGrp092,

    #[serde(rename = "TRD_GRP_093")]
    TrdGrp093,

    #[serde(rename = "TRD_GRP_094")]
    TrdGrp094,

    #[serde(rename = "TRD_GRP_095")]
    TrdGrp095,

    #[serde(rename = "TRD_GRP_096")]
    TrdGrp096,

    #[serde(rename = "TRD_GRP_097")]
    TrdGrp097,

    #[serde(rename = "TRD_GRP_098")]
    TrdGrp098,

    #[serde(rename = "TRD_GRP_099")]
    TrdGrp099,

    #[serde(rename = "TRD_GRP_100")]
    TrdGrp100,

    #[serde(rename = "TRD_GRP_101")]
    TrdGrp101,

    #[serde(rename = "TRD_GRP_102")]
    TrdGrp102,

    #[serde(rename = "TRD_GRP_103")]
    TrdGrp103,

    #[serde(rename = "TRD_GRP_104")]
    TrdGrp104,

    #[serde(rename = "TRD_GRP_105")]
    TrdGrp105,

    #[serde(rename = "TRD_GRP_106")]
    TrdGrp106,

    #[serde(rename = "TRD_GRP_107")]
    TrdGrp107,

    #[serde(rename = "TRD_GRP_108")]
    TrdGrp108,

    #[serde(rename = "TRD_GRP_109")]
    TrdGrp109,

    #[serde(rename = "TRD_GRP_110")]
    TrdGrp110,

    #[serde(rename = "TRD_GRP_111")]
    TrdGrp111,

    #[serde(rename = "TRD_GRP_112")]
    TrdGrp112,

    #[serde(rename = "TRD_GRP_113")]
    TrdGrp113,

    #[serde(rename = "TRD_GRP_114")]
    TrdGrp114,

    #[serde(rename = "TRD_GRP_115")]
    TrdGrp115,

    #[serde(rename = "TRD_GRP_116")]
    TrdGrp116,

    #[serde(rename = "TRD_GRP_117")]
    TrdGrp117,

    #[serde(rename = "TRD_GRP_118")]
    TrdGrp118,

    #[serde(rename = "TRD_GRP_119")]
    TrdGrp119,

    #[serde(rename = "TRD_GRP_120")]
    TrdGrp120,

    #[serde(rename = "TRD_GRP_121")]
    TrdGrp121,

    #[serde(rename = "TRD_GRP_122")]
    TrdGrp122,

    #[serde(rename = "TRD_GRP_123")]
    TrdGrp123,

    #[serde(rename = "TRD_GRP_124")]
    TrdGrp124,

    #[serde(rename = "TRD_GRP_125")]
    TrdGrp125,

    #[serde(rename = "TRD_GRP_126")]
    TrdGrp126,

    #[serde(rename = "TRD_GRP_127")]
    TrdGrp127,

    #[serde(rename = "TRD_GRP_128")]
    TrdGrp128,

    #[serde(rename = "TRD_GRP_129")]
    TrdGrp129,

    #[serde(rename = "TRD_GRP_130")]
    TrdGrp130,

    #[serde(rename = "TRD_GRP_131")]
    TrdGrp131,

    #[serde(rename = "TRD_GRP_132")]
    TrdGrp132,

    #[serde(rename = "TRD_GRP_133")]
    TrdGrp133,

    #[serde(rename = "TRD_GRP_134")]
    TrdGrp134,

    #[serde(rename = "TRD_GRP_135")]
    TrdGrp135,

    #[serde(rename = "TRD_GRP_136")]
    TrdGrp136,

    #[serde(rename = "TRD_GRP_137")]
    TrdGrp137,

    #[serde(rename = "TRD_GRP_138")]
    TrdGrp138,

    #[serde(rename = "TRD_GRP_139")]
    TrdGrp139,

    #[serde(rename = "TRD_GRP_140")]
    TrdGrp140,

    #[serde(rename = "TRD_GRP_141")]
    TrdGrp141,

    #[serde(rename = "TRD_GRP_142")]
    TrdGrp142,

    #[serde(rename = "TRD_GRP_143")]
    TrdGrp143,

    #[serde(rename = "TRD_GRP_144")]
    TrdGrp144,

    #[serde(rename = "TRD_GRP_145")]
    TrdGrp145,

    #[serde(rename = "TRD_GRP_146")]
    TrdGrp146,

    #[serde(rename = "TRD_GRP_147")]
    TrdGrp147,

    #[serde(rename = "TRD_GRP_148")]
    TrdGrp148,

    #[serde(rename = "TRD_GRP_149")]
    TrdGrp149,

    #[serde(rename = "TRD_GRP_150")]
    TrdGrp150,

    #[serde(rename = "TRD_GRP_151")]
    TrdGrp151,

    #[serde(rename = "TRD_GRP_152")]
    TrdGrp152,

    #[serde(rename = "TRD_GRP_153")]
    TrdGrp153,

    #[serde(rename = "TRD_GRP_154")]
    TrdGrp154,

    #[serde(rename = "TRD_GRP_155")]
    TrdGrp155,

    #[serde(rename = "TRD_GRP_156")]
    TrdGrp156,

    #[serde(rename = "TRD_GRP_157")]
    TrdGrp157,

    #[serde(rename = "TRD_GRP_158")]
    TrdGrp158,

    #[serde(rename = "TRD_GRP_159")]
    TrdGrp159,

    #[serde(rename = "TRD_GRP_160")]
    TrdGrp160,

    #[serde(rename = "TRD_GRP_161")]
    TrdGrp161,

    #[serde(rename = "TRD_GRP_162")]
    TrdGrp162,

    #[serde(rename = "TRD_GRP_163")]
    TrdGrp163,

    #[serde(rename = "TRD_GRP_164")]
    TrdGrp164,

    #[serde(rename = "TRD_GRP_165")]
    TrdGrp165,

    #[serde(rename = "TRD_GRP_166")]
    TrdGrp166,

    #[serde(rename = "TRD_GRP_167")]
    TrdGrp167,

    #[serde(rename = "TRD_GRP_168")]
    TrdGrp168,

    #[serde(rename = "TRD_GRP_169")]
    TrdGrp169,

    #[serde(rename = "TRD_GRP_170")]
    TrdGrp170,

    #[serde(rename = "TRD_GRP_171")]
    TrdGrp171,

    #[serde(rename = "TRD_GRP_172")]
    TrdGrp172,

    #[serde(rename = "TRD_GRP_173")]
    TrdGrp173,

    #[serde(rename = "TRD_GRP_174")]
    TrdGrp174,

    #[serde(rename = "TRD_GRP_175")]
    TrdGrp175,

    #[serde(rename = "TRD_GRP_176")]
    TrdGrp176,

    #[serde(rename = "TRD_GRP_177")]
    TrdGrp177,

    #[serde(rename = "TRD_GRP_178")]
    TrdGrp178,

    #[serde(rename = "TRD_GRP_179")]
    TrdGrp179,

    #[serde(rename = "TRD_GRP_180")]
    TrdGrp180,

    #[serde(rename = "TRD_GRP_181")]
    TrdGrp181,

    #[serde(rename = "TRD_GRP_182")]
    TrdGrp182,

    #[serde(rename = "TRD_GRP_183")]
    TrdGrp183,

    #[serde(rename = "TRD_GRP_184")]
    TrdGrp184,

    #[serde(rename = "TRD_GRP_185")]
    TrdGrp185,

    #[serde(rename = "TRD_GRP_186")]
    TrdGrp186,

    #[serde(rename = "TRD_GRP_187")]
    TrdGrp187,

    #[serde(rename = "TRD_GRP_188")]
    TrdGrp188,

    #[serde(rename = "TRD_GRP_189")]
    TrdGrp189,

    #[serde(rename = "TRD_GRP_190")]
    TrdGrp190,

    #[serde(rename = "TRD_GRP_191")]
    TrdGrp191,

    #[serde(rename = "TRD_GRP_192")]
    TrdGrp192,

    #[serde(rename = "TRD_GRP_193")]
    TrdGrp193,

    #[serde(rename = "TRD_GRP_194")]
    TrdGrp194,

    #[serde(rename = "TRD_GRP_195")]
    TrdGrp195,

    #[serde(rename = "TRD_GRP_196")]
    TrdGrp196,

    #[serde(rename = "TRD_GRP_197")]
    TrdGrp197,

    #[serde(rename = "TRD_GRP_198")]
    TrdGrp198,

    #[serde(rename = "TRD_GRP_199")]
    TrdGrp199,

    #[serde(rename = "TRD_GRP_200")]
    TrdGrp200,

    #[serde(rename = "TRD_GRP_201")]
    TrdGrp201,

    #[serde(rename = "TRD_GRP_202")]
    TrdGrp202,

    #[serde(rename = "TRD_GRP_203")]
    TrdGrp203,

    #[serde(rename = "TRD_GRP_204")]
    TrdGrp204,

    #[serde(rename = "TRD_GRP_205")]
    TrdGrp205,

    #[serde(rename = "TRD_GRP_206")]
    TrdGrp206,

    #[serde(rename = "TRD_GRP_207")]
    TrdGrp207,

    #[serde(rename = "TRD_GRP_208")]
    TrdGrp208,

    #[serde(rename = "TRD_GRP_209")]
    TrdGrp209,

    #[serde(rename = "TRD_GRP_210")]
    TrdGrp210,

    #[serde(rename = "TRD_GRP_211")]
    TrdGrp211,

    #[serde(rename = "TRD_GRP_212")]
    TrdGrp212,

    #[serde(rename = "TRD_GRP_213")]
    TrdGrp213,

    #[serde(rename = "TRD_GRP_214")]
    TrdGrp214,

    #[serde(rename = "TRD_GRP_215")]
    TrdGrp215,

    #[serde(rename = "TRD_GRP_216")]
    TrdGrp216,

    #[serde(rename = "TRD_GRP_217")]
    TrdGrp217,

    #[serde(rename = "TRD_GRP_218")]
    TrdGrp218,

    #[serde(rename = "TRD_GRP_219")]
    TrdGrp219,

    #[serde(rename = "TRD_GRP_220")]
    TrdGrp220,

    #[serde(rename = "TRD_GRP_221")]
    TrdGrp221,

    #[serde(rename = "TRD_GRP_222")]
    TrdGrp222,

    #[serde(rename = "TRD_GRP_223")]
    TrdGrp223,

    #[serde(rename = "TRD_GRP_224")]
    TrdGrp224,

    #[serde(rename = "TRD_GRP_225")]
    TrdGrp225,

    #[serde(rename = "TRD_GRP_226")]
    TrdGrp226,

    #[serde(rename = "TRD_GRP_227")]
    TrdGrp227,

    #[serde(rename = "TRD_GRP_228")]
    TrdGrp228,

    #[serde(rename = "TRD_GRP_229")]
    TrdGrp229,

    #[serde(rename = "TRD_GRP_230")]
    TrdGrp230,

    #[serde(rename = "TRD_GRP_231")]
    TrdGrp231,

    #[serde(rename = "TRD_GRP_232")]
    TrdGrp232,

    #[serde(rename = "TRD_GRP_233")]
    TrdGrp233,

    #[serde(rename = "TRD_GRP_234")]
    TrdGrp234,

    #[serde(rename = "TRD_GRP_235")]
    TrdGrp235,

    #[serde(rename = "TRD_GRP_236")]
    TrdGrp236,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuoteAsset {
    #[serde(rename = "AEUR")]
    Aeur,

    #[serde(rename = "ARS")]
    Ars,

    #[serde(rename = "AUD")]
    Aud,

    #[serde(rename = "BIDR")]
    Bidr,

    #[serde(rename = "BKRW")]
    Bkrw,

    #[serde(rename = "BNB")]
    Bnb,

    #[serde(rename = "BRL")]
    Brl,

    #[serde(rename = "BTC")]
    Btc,

    #[serde(rename = "BUSD")]
    Busd,

    #[serde(rename = "BVND")]
    Bvnd,

    #[serde(rename = "COP")]
    Cop,

    #[serde(rename = "CZK")]
    Czk,

    #[serde(rename = "DAI")]
    Dai,

    #[serde(rename = "DOGE")]
    Doge,

    #[serde(rename = "DOT")]
    Dot,

    #[serde(rename = "ETH")]
    Eth,

    #[serde(rename = "EUR")]
    Eur,

    #[serde(rename = "EURI")]
    Euri,

    #[serde(rename = "FDUSD")]
    Fdusd,

    #[serde(rename = "GBP")]
    Gbp,

    #[serde(rename = "IDRT")]
    Idrt,

    #[serde(rename = "JPY")]
    Jpy,

    #[serde(rename = "MXN")]
    Mxn,

    #[serde(rename = "NGN")]
    Ngn,

    #[serde(rename = "PAX")]
    Pax,

    #[serde(rename = "PLN")]
    Pln,

    #[serde(rename = "RON")]
    Ron,

    #[serde(rename = "RUB")]
    Rub,

    #[serde(rename = "SOL")]
    Sol,

    #[serde(rename = "TRX")]
    Trx,

    #[serde(rename = "TRY")]
    Try,

    #[serde(rename = "TUSD")]
    Tusd,

    #[serde(rename = "UAH")]
    Uah,

    #[serde(rename = "USDC")]
    Usdc,

    #[serde(rename = "USDP")]
    Usdp,

    #[serde(rename = "USDS")]
    Usds,

    #[serde(rename = "USDT")]
    Usdt,

    #[serde(rename = "UST")]
    Ust,

    #[serde(rename = "VAI")]
    Vai,

    #[serde(rename = "XRP")]
    Xrp,

    #[serde(rename = "ZAR")]
    Zar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "BREAK")]
    Break,

    #[serde(rename = "TRADING")]
    Trading,
}
