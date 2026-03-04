import { LifecycleState } from "@/types/life-cycle-state";

export interface Strategy {
    id?: number;
    name?: string;
    is_active?: boolean;
    can_trade?: boolean;
    description?: string | null;
    last_execution?: string | null;
    cooldown?: number | null;
    error_last_date?: string | null;
    error_cooldown?: number | null;
    last_update?: string | null;
}

export interface CacheStrategy {
    model: Strategy;
    state: LifecycleState;
    last_update_date: string;
    last_error_message?: string | null;
}

export interface CacheStrategies {
    models: Record<number, CacheStrategy>;
    startup_date: string;
    last_update_date: string;
    status: LifecycleState;
}
