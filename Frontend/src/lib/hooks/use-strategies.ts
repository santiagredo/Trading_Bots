import { useCallback, useMemo, useState } from "react";
import {
    CacheStrategy,
    CacheStrategies,
    Strategy,
} from "@/interfaces/entities/strategy";
import { strategyService } from "@/lib/services/strategies";
import { StrategyUI } from "@/interfaces/strategy-ui";
import { RuntimeStatus } from "@/types/runtime-status";

export function useStrategies(env: string) {
    const [strategies, setStrategies] = useState<Strategy[]>([]);
    const [runtime, setRuntime] = useState<CacheStrategies | null>(null);

    const activeStrategies = runtime?.models ?? {};

    const uiStrategies = useMemo(
        () => mergeStrategies(strategies, activeStrategies),
        [strategies, activeStrategies],
    );

    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const loadAll = useCallback(async () => {
        setLoading(true);
        setError(null);

        const result = await strategyService.selectAll(env);

        if (result.ok) {
            setStrategies(result.data);
        } else {
            setError(String(result.error));
        }

        setLoading(false);
    }, [env]);

    const loadActive = useCallback(
        async (filter?: Strategy) => {
            setLoading(true);
            setError(null);

            const result = await strategyService.getActiveAll(env, filter);

            if (result.ok) {
                setRuntime(result.data ?? null);
            } else {
                setError(String(result.error));
            }

            setLoading(false);
        },
        [env],
    );

    const create = useCallback(
        async (strategy: Strategy) => {
            setLoading(true);
            setError(null);

            const result = await strategyService.insert(env, strategy);

            if (result.ok) {
                setStrategies((prev) => [...prev, result.data]);
            } else {
                setError(String(result.error));
            }

            setLoading(false);
            return result;
        },
        [env],
    );

    const update = useCallback(
        async (strategy: Strategy) => {
            setLoading(true);
            setError(null);

            const result = await strategyService.update(env, strategy);

            if (result.ok) {
                setStrategies((prev) =>
                    prev.map((s) =>
                        s.id === result.data.id ? result.data : s,
                    ),
                );
            } else {
                setError(String(result.error));
            }

            setLoading(false);
            return result;
        },
        [env],
    );

    const remove = useCallback(
        async (strategy: Strategy) => {
            if (!strategy.id) {
                setError("Strategy ID is required to delete");
                return { ok: false, error: "Missing strategy id" };
            }

            setLoading(true);
            setError(null);

            const result = await strategyService.delete(env, strategy);

            if (result.ok) {
                setStrategies((prev) =>
                    prev.filter((s) => s.id !== strategy.id),
                );
            } else {
                setError(String(result.error));
            }

            setLoading(false);
            return result;
        },
        [env],
    );

    const startActive = useCallback(async () => {
        setLoading(true);
        setError(null);

        const result = await strategyService.startActive(env);

        if (result.ok) {
            await loadActive();
        } else {
            setError(String(result.error));
        }

        setLoading(false);
    }, [env, loadActive]);

    const stopActive = useCallback(async () => {
        setLoading(true);
        setError(null);

        const result = await strategyService.stopActive(env);

        if (result.ok) {
            await loadActive();
        } else {
            setError(String(result.error));
        }

        setLoading(false);
    }, [env, loadActive]);

    const loadWithRuntime = useCallback(async () => {
        await Promise.all([loadAll(), loadActive()]);
    }, [loadAll, loadActive]);

    return {
        strategies,
        runtime, 
        activeStrategies, 
        uiStrategies,
        loading,
        error,

        loadAll,
        loadActive,

        create,
        update,
        remove,

        startActive,
        stopActive,

        loadWithRuntime,
    };
}

function mergeStrategies(
    db: Strategy[],
    memory: Record<number, CacheStrategy> | undefined,
): StrategyUI[] {
    const memoryMap = new Map<number, CacheStrategy>();

    const memoryArray = memory ? Object.values(memory) : [];

    for (const m of memoryArray) {
        if (m.model?.id != null) {
            memoryMap.set(m.model.id, m);
        }
    }

    return db.map((dbStrategy) => {
        const mem = dbStrategy.id ? memoryMap.get(dbStrategy.id) : undefined;

        let runtimeStatus: RuntimeStatus = "not_loaded";
        let state = undefined;
        let last_error_message: string | null | undefined = undefined;
        let last_update_date: string | undefined = undefined;

        if (mem) {
            state = mem.state;
            last_error_message = mem.last_error_message ?? null;
            last_update_date = mem.last_update_date;

            runtimeStatus =
                mem.model?.last_update !== dbStrategy.last_update
                    ? "outdated"
                    : "loaded";
        }

        return {
            ...dbStrategy,
            runtimeStatus,
            state,
            last_error_message,
            last_update_date,
        } as StrategyUI;
    });
}
