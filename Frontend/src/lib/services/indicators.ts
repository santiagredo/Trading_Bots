import { Indicator } from "@/interfaces/entities/indicator";
import { Result } from "@/types/result";
import { invoke } from "@tauri-apps/api/core";

// db
async function insert_indicator(
    env: string,
    indicator: Indicator
): Promise<Result<Indicator>> {
    try {
        indicator.id = 0;

        const data = await invoke<Indicator>("insert_indicator", {
            env,
            indicator,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function select_indicator(
    env: string,
    indicator?: Indicator
): Promise<Result<Indicator>> {
    try {
        const data = await invoke<Indicator>("select_indicator", {
            env,
            indicator,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function select_indicators(env: string): Promise<Result<Indicator[]>> {
    try {
        const data = await invoke<Indicator[]>("select_indicators", {
            env,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function update_indicator(
    env: string,
    indicator: Indicator
): Promise<Result<Indicator>> {
    try {
        const data = await invoke<Indicator>("update_indicator", {
            env,
            indicator,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function delete_indicator(
    env: string,
    indicator: Indicator
): Promise<Result<number>> {
    try {
        const data = await invoke<number>("delete_indicator", {
            env,
            indicator,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

// cache
async function get_active_indicator(
    env: string,
    indicator?: Indicator
): Promise<Result<Indicator>> {
    try {
        const data = await invoke<Indicator>("get_active_indicator", {
            env,
            indicator,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function get_active_indicators(
    env: string
): Promise<Result<Indicator[]>> {
    try {
        const data = await invoke<Indicator[]>("get_active_indicators", {
            env,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function get_subscribed_indicators(
    env: string
): Promise<Result<Record<string, number[]> | null>> {
    try {
        const data = await invoke<Record<string, number[]>>(
            "get_subscribed_indicators",
            {
                env,
            }
        );

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function start_active_indicators(env: string): Promise<Result<void>> {
    try {
        await invoke("start_active_indicators", {
            env,
        });

        return { ok: true, data: undefined };
    } catch (error) {
        return { ok: false, error };
    }
}

async function stop_active_indicators(env: string): Promise<Result<void>> {
    try {
        await invoke("stop_active_indicators", {
            env,
        });

        return { ok: true, data: undefined };
    } catch (error) {
        return { ok: false, error };
    }
}

export const indicatorService = {
    // db
    insert: (env: string, indicator: Indicator): Promise<Result<Indicator>> =>
        insert_indicator(env, indicator),

    select: (env: string, indicator?: Indicator): Promise<Result<Indicator>> =>
        select_indicator(env, indicator),

    selectAll: (env: string): Promise<Result<Indicator[]>> =>
        select_indicators(env),

    update: (env: string, indicator: Indicator): Promise<Result<Indicator>> =>
        update_indicator(env, indicator),

    delete: (env: string, indicator: Indicator): Promise<Result<number>> =>
        delete_indicator(env, indicator),

    // cache
    getActive: (
        env: string,
        indicator?: Indicator
    ): Promise<Result<Indicator>> => get_active_indicator(env, indicator),

    getActiveAll: (env: string): Promise<Result<Indicator[]>> =>
        get_active_indicators(env),

    getSubscribed: (
        env: string
    ): Promise<Result<Record<string, number[]> | null>> =>
        get_subscribed_indicators(env),

    startActive: (env: string): Promise<Result<void>> =>
        start_active_indicators(env),

    stopActive: (env: string): Promise<Result<void>> =>
        stop_active_indicators(env),
};
