import { Asset } from "@/interfaces/entities/asset";
import { Result } from "@/types/result";
import { invoke } from "@tauri-apps/api/core";

export async function select_assets(env: string): Promise<Result<Asset[]>> {
    try {
        let data = await invoke<Asset[]>("select_assets", {
            env,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}
