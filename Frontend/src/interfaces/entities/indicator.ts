import { LifecycleState } from "@/types/life-cycle-state";

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

export interface CacheIndicators {
    models: Record<string, Indicator>;
    startup_date: string;
    last_update_date: string;
    status: LifecycleState;
}
