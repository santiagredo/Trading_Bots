import { invoke } from "@tauri-apps/api/core";
import { Result } from "@/types/result";
import { StrategyOverview } from "@/interfaces/strategy-overview";
import { Strategy } from "@/interfaces/entities/strategy";

async function select_strategies_overview(
    env: string,
    query: Strategy
): Promise<Result<StrategyOverview[]>> {
    try {
        const data = await invoke<StrategyOverview[]>(
            "select_strategies_overview",
            {
                env,
                query,
            }
        );

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

export const strategyOverviewService = {
    select: (
        env: string,
        query: Strategy
    ): Promise<Result<StrategyOverview[]>> =>
        select_strategies_overview(env, query),
};
