export interface Order {
    id: number;
    status_id: number;
    creation_date: string;
    update_date: string;
    is_sell: boolean;
    strategy_id: number;
    base_asset_id: number;
    base_asset_amount: string;
    quote_asset_id: number;
    quote_asset_amount: string;
    price_entry: string;
    price_target: string;
    price_abort: string;
}
