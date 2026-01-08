import { Configuration } from "@/interfaces/configuration";
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
 * configuration
 * ========================= */

async function select_configuration(): Promise<Result<Configuration>> {
    try {
        const data = await invoke<Configuration>("select_configuration");
        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function insert_configuration(
    configuration: Configuration
): Promise<Result<Configuration>> {
    try {
        const data = await invoke<Configuration>("insert_configuration", {
            configuration,
        });
        return { ok: true, data };
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

    /* configuration */
    getConfiguration: (): Promise<Result<Configuration>> =>
        select_configuration(),

    saveConfiguration: (
        configuration: Configuration
    ): Promise<Result<Configuration>> => insert_configuration(configuration),

    /* health check */
    getHealthCheck: (): Promise<Result<CacheHealthCheck>> =>
        select_health_check(),
};
