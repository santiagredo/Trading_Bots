import { IntegrationSetting } from "@/interfaces/integration-setting";
import { QueryOptions } from "@/interfaces/query-options";
import { Result } from "@/types/result";
import { invoke } from "@tauri-apps/api/core";

/* =========================
 * database
 * ========================= */

async function select(
    env: string,
    filters?: Partial<IntegrationSetting>,
): Promise<Result<IntegrationSetting>> {
    try {
        const data = await invoke<IntegrationSetting>(
            "select_integration_setting",
            {
                env,
                integrationSetting: filters ?? {},
            },
        );

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function selectAll(
    env: string,
    filters?: Partial<IntegrationSetting>,
    query?: QueryOptions,
): Promise<Result<IntegrationSetting[]>> {
    try {
        const data = await invoke<IntegrationSetting[]>(
            "select_integrations_settings",
            {
                env,
                integrationSetting: filters ?? {},
                query: query ?? {},
            },
        );

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function update(
    env: string,
    setting: IntegrationSetting,
): Promise<Result<IntegrationSetting>> {
    try {
        const data = await invoke<IntegrationSetting>(
            "update_integration_setting",
            {
                env,
                integrationSetting: setting,
            },
        );

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

/* =========================
 * memory (cache)
 * ========================= */

async function getMemory(
    env: string,
    id: number,
): Promise<Result<IntegrationSetting>> {
    try {
        const data = await invoke<IntegrationSetting>(
            "get_integration_setting",
            {
                env,
                integrationSetting: { id },
            },
        );

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function getMemoryAll(
    env: string,
): Promise<Result<IntegrationSetting[]>> {
    try {
        const data = await invoke<IntegrationSetting[]>(
            "get_integrations_settings",
            {
                env,
            },
        );

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

/* =========================
 * public service
 * ========================= */

export const integrationSettingService = {
    // database
    select,
    selectAll,
    update,

    // memory
    getMemory,
    getMemoryAll,
};
