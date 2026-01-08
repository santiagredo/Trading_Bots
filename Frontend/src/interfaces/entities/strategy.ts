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
    is_posting?: boolean;
    model?: Strategy;
    last_error_date?: string | null;
    last_error_message?: string | null;
}
