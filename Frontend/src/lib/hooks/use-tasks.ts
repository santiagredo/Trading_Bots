import { useCallback, useMemo, useState } from "react";
import { CacheTask, CacheTasks, Task } from "@/interfaces/entities/task";
import { taskService } from "@/lib/services/tasks";
import { RuntimeStatus } from "@/types/runtime-status";

export function useTasks(env: string) {
    const [tasks, setTasks] = useState<Task[]>([]);
    const [cache, setCache] = useState<CacheTasks | null>(null);

    const uiTasks = useMemo(() => mergeTasks(tasks, cache), [tasks, cache]);

    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const loadAll = useCallback(async () => {
        setLoading(true);
        setError(null);

        const result = await taskService.selectAll(env);

        if (result.ok) {
            setTasks(result.data);
        } else {
            setError(String(result.error));
        }

        setLoading(false);
    }, [env]);

    const loadActive = useCallback(async () => {
        setLoading(true);
        setError(null);

        const result = await taskService.getActive(env);

        if (result.ok) {
            setCache(result.data);
        } else {
            setError(String(result.error));
        }

        setLoading(false);
    }, [env]);

    const update = useCallback(
        async (task: Task) => {
            setLoading(true);
            setError(null);

            const result = await taskService.update(env, task);

            if (result.ok) {
                setTasks((prev) =>
                    prev.map((t) =>
                        t.id === result.data.id ? result.data : t,
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

    const startActive = useCallback(async () => {
        setLoading(true);
        setError(null);

        const result = await taskService.startActive(env);

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

        const result = await taskService.stopActive(env);

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
        tasks,
        cache,
        uiTasks,
        loading,
        error,

        loadAll,
        loadActive,
        update,

        startActive,
        stopActive,

        loadWithRuntime,
    };
}

// ======================================================
// Merge DB + Cache
// ======================================================

function mergeTasks(db: Task[], cache: CacheTasks | null) {
    if (!cache) {
        return db.map((t) => ({
            ...t,
            runtimeStatus: "not_loaded" as RuntimeStatus,
        }));
    }

    const memoryMap = new Map<number, CacheTask>();

    for (const key in cache.models) {
        const id = Number(key);
        memoryMap.set(id, cache.models[id]);
    }

    return db.map((dbTask) => {
        const mem = dbTask.id != null ? memoryMap.get(dbTask.id) : undefined;

        let runtimeStatus: RuntimeStatus = "not_loaded";

        if (!mem) {
            runtimeStatus = "not_loaded";
        } else if (
            mem.model.nick !== dbTask.nick ||
            mem.model.description !== dbTask.description ||
            mem.model.is_active !== dbTask.is_active ||
            mem.model.cooldown !== dbTask.cooldown ||
            mem.model.delay !== dbTask.delay
        ) {
            runtimeStatus = "outdated";
        } else {
            runtimeStatus = "loaded";
        }

        return {
            ...dbTask,
            runtimeStatus,
            taskState: mem?.state,
        };
    });
}
