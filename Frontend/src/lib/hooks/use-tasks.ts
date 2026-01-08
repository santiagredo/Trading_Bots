import { useCallback, useMemo, useState } from "react";
import { Task } from "@/interfaces/entities/task";
import { taskService } from "@/lib/services/tasks";
import { RuntimeStatus } from "@/types/runtime-status";

export function useTasks(env: string) {
    const [tasks, setTasks] = useState<Task[]>([]);
    const [activeTasks, setActiveTasks] = useState<Task[]>([]);

    const uiTasks = useMemo(
        () => mergeTasks(tasks, activeTasks),
        [tasks, activeTasks]
    );

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
            setActiveTasks(result.data);
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
                    prev.map((t) => (t.id === result.data.id ? result.data : t))
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
        activeTasks,
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

function mergeTasks(db: Task[], memory: Task[]) {
    const memoryMap = new Map<number, Task>();

    for (const m of memory) {
        if (m.id != null) {
            memoryMap.set(m.id, m);
        }
    }

    return db.map((dbTask) => {
        const mem = dbTask.id ? memoryMap.get(dbTask.id) : undefined;

        let runtimeStatus: RuntimeStatus = "not_loaded";

        if (!mem) {
            runtimeStatus = "not_loaded";
        } else if (
            mem.nick !== dbTask.nick ||
            mem.description !== dbTask.description ||
            mem.is_active !== dbTask.is_active ||
            mem.cooldown !== dbTask.cooldown ||
            mem.delay !== dbTask.delay
        ) {
            runtimeStatus = "outdated";
        } else {
            runtimeStatus = "loaded";
        }

        return {
            ...dbTask,
            runtimeStatus,
        };
    });
}
