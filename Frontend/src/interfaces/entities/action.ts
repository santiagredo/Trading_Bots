import { LifecycleState } from "@/types/life-cycle-state";

export interface Action {
    id?: number;
    strategy_id?: number;
    is_active?: boolean;
    is_sell?: boolean;
    is_quote_asset?: boolean;
    is_percentage?: boolean;
    value?: number;
    pair_id?: number;
    last_update?: string;
}

export interface CacheActions {
    models: Record<string, Action>;
    startup_date: string;
    last_update_date: string;
    status: LifecycleState;
}
