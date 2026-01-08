import { RuntimeStatus } from "@/types/runtime-status";
import { Strategy } from "./entities/strategy";

export interface StrategyUI extends Strategy {
    runtimeStatus: RuntimeStatus;
    is_posting?: boolean;
    last_error_date?: string | null;
    last_error_message?: string | null;
}
