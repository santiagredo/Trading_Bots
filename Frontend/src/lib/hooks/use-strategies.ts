import { useCallback, useMemo, useState } from "react";
import { CacheStrategy, Strategy } from "@/interfaces/entities/strategy";
import { strategyService } from "@/lib/services/strategies";
import { StrategyUI } from "@/interfaces/strategy-ui";
import { RuntimeStatus } from "@/types/runtime-status";

export function useStrategies(env: string) {
    const [strategies, setStrategies] = useState<Strategy[]>([]);
    const [activeStrategies, setActiveStrategies] = useState<
        Record<string, CacheStrategy>
    >({});

    const uiStrategies = useMemo(
        () => mergeStrategies(strategies, activeStrategies),
        [strategies, activeStrategies]
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
                setActiveStrategies(result.data);
            } else {
                setError(String(result.error));
            }

            setLoading(false);
        },
        [env]
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
        [env]
    );

    const update = useCallback(
        async (strategy: Strategy) => {
            setLoading(true);
            setError(null);

            const result = await strategyService.update(env, strategy);

            if (result.ok) {
                setStrategies((prev) =>
                    prev.map((s) => (s.id === result.data.id ? result.data : s))
                );
            } else {
                setError(String(result.error));
            }

            setLoading(false);
            return result;
        },
        [env]
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
                    prev.filter((s) => s.id !== strategy.id)
                );
            } else {
                setError(String(result.error));
            }

            setLoading(false);
            return result;
        },
        [env]
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
    memory: Record<string, CacheStrategy> | undefined
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
        let is_posting: boolean | undefined = undefined;
        let last_error_date: string | null | undefined = undefined;
        let last_error_message: string | null | undefined = undefined;

        if (mem) {
            is_posting = mem.is_posting;
            last_error_date = mem.last_error_date;
            last_error_message = mem.last_error_message;

            runtimeStatus =
                mem.model?.last_update !== dbStrategy.last_update
                    ? "outdated"
                    : "loaded";
        }

        return {
            ...dbStrategy,
            runtimeStatus,
            is_posting,
            last_error_date,
            last_error_message,
        } as StrategyUI;
    });
}
