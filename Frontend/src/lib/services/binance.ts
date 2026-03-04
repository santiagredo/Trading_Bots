import { AccountInformation } from "@/interfaces/account-information";
import { Result } from "@/types/result";
import { invoke } from "@tauri-apps/api/core";

async function get_account(env: string): Promise<Result<AccountInformation>> {
    try {
        const data = await invoke<AccountInformation>("get_account", { env });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

// service
export const binanceService = {
    get: (env: string): Promise<Result<AccountInformation>> => get_account(env),
};
