import { invoke } from "@tauri-apps/api/core";
import { Ledger } from "@/interfaces/entities/ledger";
import { Result } from "@/types/result";

// db
async function insert_ledger(
    env: string,
    ledger: Ledger
): Promise<Result<Ledger>> {
    try {
        ledger.id = 0;

        const data = await invoke<Ledger>("insert_ledger", {
            env,
            ledger,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function select_ledger(
    env: string,
    ledger?: Partial<Ledger>
): Promise<Result<Ledger>> {
    try {
        const data = await invoke<Ledger>("select_ledger", {
            env,
            query: ledger,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function select_ledgers(
    env: string,
    ledger?: Partial<Ledger>
): Promise<Result<Ledger[]>> {
    try {
        const data = await invoke<Ledger[]>("select_ledgers", {
            env,
            query: ledger,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

export const ledgerService = {
    // db
    insert: (env: string, ledger: Ledger): Promise<Result<Ledger>> =>
        insert_ledger(env, ledger),

    select: (env: string, ledger?: Partial<Ledger>): Promise<Result<Ledger>> =>
        select_ledger(env, ledger),

    selectAll: (
        env: string,
        ledger?: Partial<Ledger>
    ): Promise<Result<Ledger[]>> => select_ledgers(env, ledger),
};
