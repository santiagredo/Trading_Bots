import { Task } from "@/interfaces/entities/task";
import { Result } from "@/types/result";
import { invoke } from "@tauri-apps/api/core";

async function select_task(env: string, task?: Task): Promise<Result<Task[]>> {
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

async function select_tasks(env: string, task?: Task): Promise<Result<Task[]>> {
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

async function update_task(env: string, task: Task): Promise<Result<Task>> {
    try {
        const data = await invoke<Task>("update_task", { env, task });
        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function select_active_tasks(env: string): Promise<Result<Task[]>> {
    try {
        const data = await invoke<Task[] | null>("select_active_tasks", {
            env,
        });

        return { ok: true, data: data ?? [] };
    } catch (error) {
        return { ok: false, error };
    }
}

async function start_active_tasks(env: string): Promise<Result<void>> {
    try {
        const data = await invoke<void>("start_active_tasks", { env });
        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function stop_active_tasks(env: string): Promise<Result<void>> {
    try {
        await invoke("stop_active_tasks", { env });
        return { ok: true, data: undefined };
    } catch (error) {
        return { ok: false, error };
    }
}

export const taskService = {
    // db
    select: (env: string, task?: Task): Promise<Result<Task[]>> =>
        select_task(env, task),

    selectAll: (env: string, task?: Task): Promise<Result<Task[]>> =>
        select_tasks(env, task),

    update: (env: string, task: Task): Promise<Result<Task>> =>
        update_task(env, task),

    // cache
    getActive: (env: string): Promise<Result<Task[]>> =>
        select_active_tasks(env),

    startActive: (env: string): Promise<Result<void>> =>
        start_active_tasks(env),

    stopActive: (env: string): Promise<Result<void>> => stop_active_tasks(env),
};
