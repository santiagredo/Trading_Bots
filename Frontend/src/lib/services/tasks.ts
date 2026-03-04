import { CacheTasks, Task } from "@/interfaces/entities/task";
import { Result } from "@/types/result";
import { invoke } from "@tauri-apps/api/core";

// ======================
// DB
// ======================

async function selectTask(env: string, task?: Task): Promise<Result<Task[]>> {
    try {
        const data = await invoke<Task[]>("select_task", {
            env,
            query: task,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function selectTasks(env: string, task?: Task): Promise<Result<Task[]>> {
    try {
        const data = await invoke<Task[]>("select_tasks", {
            env,
            query: task,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function updateTask(env: string, task: Task): Promise<Result<Task>> {
    try {
        const data = await invoke<Task>("update_task", { env, task });
        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

// ======================
// CACHE
// ======================

async function getActiveTasks(env: string): Promise<Result<CacheTasks>> {
    try {
        const data = await invoke<CacheTasks>("select_active_tasks", {
            env,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function startActiveTasks(env: string): Promise<Result<void>> {
    try {
        await invoke("start_active_tasks", { env });
        return { ok: true, data: undefined };
    } catch (error) {
        return { ok: false, error };
    }
}

async function stopActiveTasks(env: string): Promise<Result<void>> {
    try {
        await invoke("stop_active_tasks", { env });
        return { ok: true, data: undefined };
    } catch (error) {
        return { ok: false, error };
    }
}

// ======================
// EXPORT SERVICE
// ======================

export const taskService = {
    // DB
    select: selectTask,
    selectAll: selectTasks,
    update: updateTask,

    // Cache
    getActive: getActiveTasks,
    startActive: startActiveTasks,
    stopActive: stopActiveTasks,
};
