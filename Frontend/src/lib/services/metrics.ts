import { invoke } from "@tauri-apps/api/core";
import { Metric } from "@/interfaces/entities/metric";
import { Result } from "@/types/result";

// db
async function select_metrics(env: string): Promise<Result<Metric[]>> {
    try {
        const data = await invoke<Metric[]>("select_metrics", {
            env,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

// runtime
async function get_active_metric(
    env: string,
): Promise<Result<Metric | null>> {
    try {
        const data = await invoke<Metric | null>("get_active_metric", {
            env,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

export const metricService = {
    // db
    selectAll: (env: string): Promise<Result<Metric[]>> => select_metrics(env),

    // runtime
    getActive: (env: string): Promise<Result<Metric | null>> =>
        get_active_metric(env),
};
