import { invoke } from "@tauri-apps/api/core";
import { Result } from "@/types/result";
import { ErrorLog, IntegrationLog } from "@/interfaces/entities/logs";

// db
async function select_error_logs(env: string): Promise<Result<ErrorLog[]>> {
    try {
        const data = await invoke<ErrorLog[]>("select_error_logs", {
            env,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function select_integration_logs(
    env: string
): Promise<Result<IntegrationLog[]>> {
    try {
        const data = await invoke<IntegrationLog[]>("select_integration_logs", {
            env,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

// service
export const logService = {
    error: {
        selectAll: (env: string): Promise<Result<ErrorLog[]>> =>
            select_error_logs(env),
    },

    integration: {
        selectAll: (env: string): Promise<Result<IntegrationLog[]>> =>
            select_integration_logs(env),
    },
};
