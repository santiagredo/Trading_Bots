import { useCallback, useMemo, useState } from "react";
import { CacheIndicators, Indicator } from "@/interfaces/entities/indicator";
import { indicatorService } from "@/lib/services/indicators";
import { RuntimeStatus } from "@/types/runtime-status";

export function useIndicators(env: string) {
    const [indicators, setIndicators] = useState<Indicator[]>([]);
    const [activeCache, setActiveCache] = useState<CacheIndicators | null>(
        null,
    );
    const [subscribedIndicators, setSubscribedIndicators] = useState<Record<
        string,
        number[]
    > | null>(null);

    // Derivado desde cache real
    const activeIndicators = useMemo<Indicator[]>(() => {
        if (!activeCache) return [];
        return Object.values(activeCache.models);
    }, [activeCache]);

    const runtimeState = activeCache?.status ?? "Stopped";

    const uiIndicators = useMemo(
        () =>
            mergeIndicators(indicators, activeIndicators, subscribedIndicators),
        [indicators, activeIndicators, subscribedIndicators],
    );

    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const loadAll = useCallback(async () => {
        setLoading(true);
        setError(null);

        const result = await indicatorService.selectAll(env);

        if (result.ok) {
            setIndicators(result.data);
        } else {
            setError(String(result.error));
        }

        setLoading(false);
    }, [env]);

    const loadActive = useCallback(async () => {
        setLoading(true);
        setError(null);

        const result = await indicatorService.getActiveAll(env);

        if (result.ok) {
            setActiveCache(result.data);
        } else {
            setError(String(result.error));
        }

        setLoading(false);
    }, [env]);

    const loadSubscribed = useCallback(async () => {
        setLoading(true);
        setError(null);

        const result = await indicatorService.getSubscribed(env);

        if (result.ok) {
            setSubscribedIndicators(result.data);
        } else {
            setError(String(result.error));
        }

        setLoading(false);
    }, [env]);

    const create = useCallback(
        async (indicator: Indicator) => {
            setLoading(true);
            setError(null);

            const result = await indicatorService.insert(env, indicator);

            if (result.ok) {
                setIndicators((prev) => [...prev, result.data]);
            } else {
                setError(String(result.error));
            }

            setLoading(false);
            return result;
        },
        [env],
    );

    const update = useCallback(
        async (indicator: Indicator) => {
            setLoading(true);
            setError(null);

            const result = await indicatorService.update(env, indicator);

            if (result.ok) {
                setIndicators((prev) =>
                    prev.map((i) =>
                        i.id === result.data.id ? result.data : i,
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
        async (indicator: Indicator) => {
            if (!indicator.id) {
                setError("Indicator ID is required to delete");
                return { ok: false, error: "Missing indicator id" };
            }

            setLoading(true);
            setError(null);

            const result = await indicatorService.delete(env, indicator);

            if (result.ok) {
                setIndicators((prev) =>
                    prev.filter((i) => i.id !== indicator.id),
                );

                setActiveCache((prev) => {
                    if (!prev) return prev;

                    const updatedModels = { ...prev.models };
                    delete updatedModels[String(indicator.id)];

                    return {
                        ...prev,
                        models: updatedModels,
                    };
                });
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

        const result = await indicatorService.startActive(env);

        if (result.ok) {
            await Promise.all([loadActive(), loadSubscribed()]);
        } else {
            setError(String(result.error));
        }

        setLoading(false);
    }, [env, loadActive, loadSubscribed]);

    const stopActive = useCallback(async () => {
        setLoading(true);
        setError(null);

        const result = await indicatorService.stopActive(env);

        if (result.ok) {
            setActiveCache(null);
            setSubscribedIndicators(null);
        } else {
            setError(String(result.error));
        }

        setLoading(false);
    }, [env]);

    const loadWithRuntime = useCallback(async () => {
        await Promise.all([loadAll(), loadActive(), loadSubscribed()]);
    }, [loadAll, loadActive, loadSubscribed]);

    return {
        indicators,
        activeIndicators,
        activeCache,
        subscribedIndicators,
        uiIndicators,
        runtimeState,
        loading,
        error,

        loadAll,
        loadActive,
        loadSubscribed,

        create,
        update,
        remove,

        startActive,
        stopActive,

        loadWithRuntime,
    };
}

// helpers

function mergeIndicators(
    db: Indicator[],
    memory: Indicator[],
    subscribed: Record<string, number[]> | null,
) {
    const memoryMap = new Map<number, Indicator>();

    for (const m of memory) {
        if (m.id != null) {
            memoryMap.set(m.id, m);
        }
    }

    return db.map((dbIndicator) => {
        const mem =
            dbIndicator.id != null ? memoryMap.get(dbIndicator.id) : undefined;

        let runtimeStatus: RuntimeStatus = "not_loaded";

        if (!mem) {
            runtimeStatus = "not_loaded";
        } else if (mem.last_update !== dbIndicator.last_update) {
            runtimeStatus = "outdated";
        } else {
            runtimeStatus = "loaded";
        }

        return {
            ...dbIndicator,
            runtimeStatus,
            is_subscribed:
                dbIndicator.id != null
                    ? isIndicatorSubscribed(dbIndicator.id, subscribed)
                    : false,
        };
    });
}

function isIndicatorSubscribed(
    indicatorId: number,
    subscribed: Record<string, number[]> | null,
): boolean {
    if (!subscribed) return false;

    return Object.values(subscribed).some((ids) => ids.includes(indicatorId));
}
