import { CacheHealthCheck } from "@/interfaces/health-check";
import { Result } from "@/types/result";
import { invoke } from "@tauri-apps/api/core";

/* =========================
 * engine state
 * ========================= */

async function set_engine_running(run: boolean): Promise<Result<null>> {
    try {
        await invoke("set_engine_running", { run });
        return { ok: true, data: null };
    } catch (error) {
        return { ok: false, error };
    }
}

/* =========================
 * health check
 * ========================= */

async function select_health_check(): Promise<Result<CacheHealthCheck>> {
    try {
        const data = await invoke<CacheHealthCheck>("select_health_check");
        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

/* =========================
 * public service
 * ========================= */

export const configurationService = {
    /* engine */
    setRunning: (run: boolean): Promise<Result<null>> =>
        set_engine_running(run),

    /* health check */
    getHealthCheck: (): Promise<Result<CacheHealthCheck>> =>
        select_health_check(),
};
