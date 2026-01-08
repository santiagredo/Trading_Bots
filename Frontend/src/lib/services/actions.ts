import { Action } from "@/interfaces/entities/action";
import { Result } from "@/types/result";
import { invoke } from "@tauri-apps/api/core";

// db
async function insert_action(
    env: string,
    action: Action
): Promise<Result<Action>> {
    try {
        action.id = 0;

        const data = await invoke<Action>("insert_action", {
            env,
            action,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function select_action(
    env: string,
    action?: Action
): Promise<Result<Action>> {
    try {
        const data = await invoke<Action>("select_action", {
            env,
            action,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function select_actions(env: string): Promise<Result<Action[]>> {
    try {
        const data = await invoke<Action[]>("select_actions", {
            env,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function update_action(
    env: string,
    action: Action
): Promise<Result<Action>> {
    try {
        const data = await invoke<Action>("update_action", {
            env,
            action,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function delete_action(
    env: string,
    action: Action
): Promise<Result<number>> {
    try {
        const data = await invoke<number>("delete_action", {
            env,
            action,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

// cache
async function get_active_action(
    env: string,
    action?: Action
): Promise<Result<Action>> {
    try {
        const data = await invoke<Action>("get_active_action", {
            env,
            action,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function get_active_actions(env: string): Promise<Result<Action[]>> {
    try {
        const data = await invoke<Action[]>("get_active_actions", {
            env,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function start_active_actions(env: string): Promise<Result<void>> {
    try {
        await invoke("start_active_actions", {
            env,
        });

        return { ok: true, data: undefined };
    } catch (error) {
        return { ok: false, error };
    }
}

async function stop_active_actions(env: string): Promise<Result<void>> {
    try {
        await invoke("stop_active_actions", {
            env,
        });

        return { ok: true, data: undefined };
    } catch (error) {
        return { ok: false, error };
    }
}

// service
export const actionService = {
    // db
    insert: (env: string, action: Action): Promise<Result<Action>> =>
        insert_action(env, action),

    select: (env: string, action?: Action): Promise<Result<Action>> =>
        select_action(env, action),

    selectAll: (env: string): Promise<Result<Action[]>> => select_actions(env),

    update: (env: string, action: Action): Promise<Result<Action>> =>
        update_action(env, action),

    delete: (env: string, action: Action): Promise<Result<number>> =>
        delete_action(env, action),

    // cache
    getActive: (env: string, action?: Action): Promise<Result<Action>> =>
        get_active_action(env, action),

    getActiveAll: (env: string): Promise<Result<Action[]>> =>
        get_active_actions(env),

    startActive: (env: string): Promise<Result<void>> =>
        start_active_actions(env),

    stopActive: (env: string): Promise<Result<void>> =>
        stop_active_actions(env),
};
