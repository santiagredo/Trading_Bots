import { useCallback, useState } from "react";
import { StrategyOverview } from "@/interfaces/strategy-overview";
import { Strategy } from "@/interfaces/entities/strategy";
import { strategyOverviewService } from "../services/strategies-overview";

export function useStrategiesOverview(env: string) {
    const [strategies, setStrategies] = useState<StrategyOverview[]>([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    // =======================
    // loader
    // =======================

    const loadStrategiesOverview = useCallback(
        async (query: Strategy) => {
            setLoading(true);
            setError(null);

            const result = await strategyOverviewService.select(env, query);

            if (result.ok) {
                setStrategies(result.data);
            } else {
                setError(String(result.error));
            }

            setLoading(false);
        },
        [env]
    );

    return {
        // state
        strategies,
        loading,
        error,

        // loader
        loadStrategiesOverview,
    };
}
