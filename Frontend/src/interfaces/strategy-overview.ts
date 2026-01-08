import { Indicator } from "./entities/indicator";
import { Strategy } from "./entities/strategy";
import type { Ticker } from "./ticker";
import { Pair } from "./entities/pair";
import { Asset } from "./entities/asset";
import { Status } from "./enums/status";
import { Action } from "./entities/action";

export interface StrategyOverview {
    strategy: Strategy;
    indicator: Indicator;
    action: Action;
    pair: Pair;
    base_asset: Asset;
    quote_asset: Asset;
    ticker: Ticker;
    order_status: Record<Status, number>;
}
