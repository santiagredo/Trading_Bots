import { AccountInformation } from "@/interfaces/account-information";
import { Result } from "@/types/result";
import { invoke } from "@tauri-apps/api/core";

async function get_account(): Promise<Result<AccountInformation>> {
    try {
        const data = await invoke<AccountInformation>("get_account");

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

// service
export const binanceService = {
    get: (): Promise<Result<AccountInformation>> => get_account(),
};
