export interface Indicator {
    id?: number;
    strategy_id?: number;
    is_active?: boolean;
    symbol?: string;
    nick?: string;
    direction?: string;
    is_percentage?: boolean;
    value?: number;
    last_update?: string;
}
