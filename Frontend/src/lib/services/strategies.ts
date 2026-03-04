import { CacheStrategies, CacheStrategy, Strategy } from "@/interfaces/entities/strategy";
import { Result } from "@/types/result";
import { invoke } from "@tauri-apps/api/core";

// db
async function insert_strategy(
    env: string,
    strategy: Strategy
): Promise<Result<Strategy>> {
    try {
        strategy.id = 0;

        const data = await invoke<Strategy>("insert_strategy", {
            env,
            strategy,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function select_strategy(
    env: string,
    strategy?: Strategy
): Promise<Result<Strategy>> {
    try {
        const data = await invoke<Strategy>("select_strategy", {
            env,
            strategy,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function select_strategies(env: string): Promise<Result<Strategy[]>> {
    try {
        let data = await invoke<Strategy[]>("select_strategies", {
            env,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function update_strategy(
    env: string,
    strategy: Strategy
): Promise<Result<Strategy>> {
    try {
        const data = await invoke<Strategy>("update_strategy", {
            env,
            strategy,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function delete_strategy(
    env: string,
    strategy: Strategy
): Promise<Result<number>> {
    try {
        const data = await invoke<number>("delete_strategy", {
            env,
            strategy,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

// cache
async function get_active_strategy(
    env: string,
    strategy?: Strategy
): Promise<Result<CacheStrategy | null>> {
    try {
        const data = await invoke<CacheStrategy | null>("get_active_strategy", {
            env,
            strategy,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function get_active_strategies(
    env: string,
    strategy?: Strategy
): Promise<Result<CacheStrategies>> {
    try {
        const data = await invoke<CacheStrategies>(
            "get_active_strategies",
            { env, strategy }
        );

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function start_active_strategies(env: string): Promise<Result<void>> {
    try {
        await invoke("start_active_strategies", {
            env,
        });

        return { ok: true, data: undefined };
    } catch (error) {
        return { ok: false, error };
    }
}

async function stop_active_strategies(env: string): Promise<Result<void>> {
    try {
        await invoke("stop_active_strategies", {
            env,
        });

        return { ok: true, data: undefined };
    } catch (error) {
        return { ok: false, error };
    }
}

export const strategyService = {
    // db
    insert: (env: string, strategy: Strategy): Promise<Result<Strategy>> =>
        insert_strategy(env, strategy),

    select: (env: string, strategy?: Strategy): Promise<Result<Strategy>> =>
        select_strategy(env, strategy),

    selectAll: (env: string): Promise<Result<Strategy[]>> =>
        select_strategies(env),

    update: (env: string, strategy: Strategy): Promise<Result<Strategy>> =>
        update_strategy(env, strategy),

    delete: (env: string, strategy: Strategy): Promise<Result<number>> =>
        delete_strategy(env, strategy),

    // cache
    getActive: (env: string, strategy?: Strategy): Promise<Result<CacheStrategy | null>> =>
        get_active_strategy(env, strategy),

    getActiveAll: (
        env: string,
        strategy?: Strategy
    ): Promise<Result<CacheStrategies>> =>
        get_active_strategies(env, strategy),

    startActive: (env: string): Promise<Result<void>> =>
        start_active_strategies(env),

    stopActive: (env: string): Promise<Result<void>> =>
        stop_active_strategies(env),
};
