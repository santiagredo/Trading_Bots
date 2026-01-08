import { Pair } from "@/interfaces/entities/pair";
import { Result } from "@/types/result";
import { invoke } from "@tauri-apps/api/core";

export async function select_pairs(env: string): Promise<Result<Pair[]>> {
    try {
        let data = await invoke<Pair[]>("select_pairs", {
            env,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}
