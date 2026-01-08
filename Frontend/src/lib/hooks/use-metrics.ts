import { useCallback, useState } from "react";
import { CriticalMetric, Metric } from "@/interfaces/entities/metric";
import { metricService } from "@/lib/services/metrics";

export function useMetrics(env: string) {
    const [metrics, setMetrics] = useState<Metric[]>([]);
    const [activeMetric, setActiveMetric] = useState<CriticalMetric | null>(
        null
    );

    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const loadAll = useCallback(async () => {
        setLoading(true);
        setError(null);

        const result = await metricService.selectAll(env);

        if (result.ok) {
            setMetrics(result.data);
        } else {
            setError(String(result.error));
        }

        setLoading(false);
    }, [env]);

    const loadActive = useCallback(async () => {
        setLoading(true);
        setError(null);

        const result = await metricService.getActive(env);

        if (result.ok) {
            setActiveMetric(result.data);
        } else {
            setError(String(result.error));
        }

        setLoading(false);
    }, [env]);

    const loadWithRuntime = useCallback(async () => {
        await Promise.all([loadAll(), loadActive()]);
    }, [loadAll, loadActive]);

    return {
        metrics,
        activeMetric,
        loading,
        error,

        loadAll,
        loadActive,
        loadWithRuntime,
    };
}
