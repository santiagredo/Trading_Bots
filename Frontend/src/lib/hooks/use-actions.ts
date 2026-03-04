import { useCallback, useMemo, useState } from "react";
import { Action, CacheActions } from "@/interfaces/entities/action";
import { actionService } from "@/lib/services/actions";
import { RuntimeStatus } from "@/types/runtime-status";

export function useActions(env: string) {
    const [actions, setActions] = useState<Action[]>([]);
    const [activeCache, setActiveCache] = useState<CacheActions | null>(null);

    const activeActions = useMemo<Action[]>(() => {
        if (!activeCache) return [];
        return Object.values(activeCache.models);
    }, [activeCache]);

    const runtimeState = activeCache?.status ?? "Stopped";

    const uiActions = useMemo(
        () => mergeActions(actions, activeActions),
        [actions, activeActions],
    );

    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    // =======================
    // loaders
    // =======================

    const loadAll = useCallback(async () => {
        setLoading(true);
        setError(null);

        const result = await actionService.selectAll(env);

        if (result.ok) {
            setActions(result.data);
        } else {
            setError(String(result.error));
        }

        setLoading(false);
    }, [env]);

    const loadActive = useCallback(async () => {
        setLoading(true);
        setError(null);

        const result = await actionService.getActiveAll(env);

        if (result.ok) {
            setActiveCache(result.data);
        } else {
            setError(String(result.error));
        }

        setLoading(false);
    }, [env]);

    // =======================
    // mutations
    // =======================

    const create = useCallback(
        async (action: Action) => {
            setLoading(true);
            setError(null);

            const result = await actionService.insert(env, action);

            if (result.ok) {
                setActions((prev) => [...prev, result.data]);
            } else {
                setError(String(result.error));
            }

            setLoading(false);
            return result;
        },
        [env],
    );

    const update = useCallback(
        async (action: Action) => {
            setLoading(true);
            setError(null);

            const result = await actionService.update(env, action);

            if (result.ok) {
                setActions((prev) =>
                    prev.map((a) =>
                        a.id === result.data.id ? result.data : a,
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
        async (action: Action) => {
            if (!action.id) {
                setError("Action ID is required to delete");
                return { ok: false, error: "Missing action id" };
            }

            setLoading(true);
            setError(null);

            const result = await actionService.delete(env, action);

            if (result.ok) {
                setActions((prev) => prev.filter((a) => a.id !== action.id));

                setActiveCache((prev) => {
                    if (!prev) return prev;

                    const updatedModels = { ...prev.models };
                    delete updatedModels[String(action.id)];

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

    // =======================
    // runtime
    // =======================

    const startActive = useCallback(async () => {
        setLoading(true);
        setError(null);

        const result = await actionService.startActive(env);

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

        const result = await actionService.stopActive(env);

        if (result.ok) {
            setActiveCache(null);
        } else {
            setError(String(result.error));
        }

        setLoading(false);
    }, [env]);

    const loadWithRuntime = useCallback(async () => {
        await Promise.all([loadAll(), loadActive()]);
    }, [loadAll, loadActive]);

    return {
        actions,
        activeActions,
        activeCache,
        runtimeState,
        uiActions,
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

// =======================
// helpers
// =======================

function mergeActions(db: Action[], memory: Action[]) {
    const memoryMap = new Map<number, Action>();

    for (const m of memory) {
        if (m.id != null) {
            memoryMap.set(m.id, m);
        }
    }

    return db.map((dbAction) => {
        const mem =
            dbAction.id != null ? memoryMap.get(dbAction.id) : undefined;

        let runtimeStatus: RuntimeStatus = "not_loaded";

        if (!mem) {
            runtimeStatus = "not_loaded";
        } else if (mem.last_update !== dbAction.last_update) {
            runtimeStatus = "outdated";
        } else {
            runtimeStatus = "loaded";
        }

        return {
            ...dbAction,
            runtimeStatus,
        };
    });
}
