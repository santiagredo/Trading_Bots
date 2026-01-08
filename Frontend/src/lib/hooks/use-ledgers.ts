import { useCallback, useState } from "react";
import { Ledger } from "@/interfaces/entities/ledger";
import { ledgerService } from "@/lib/services/ledgers";
import { Result } from "@/types/result";

export function useLedgers(env: string) {
    const [ledgers, setLedgers] = useState<Ledger[]>([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const loadAll = useCallback(
        async (_query?: Partial<Ledger>) => {
            setLoading(true);
            setError(null);

            const result = await ledgerService.selectAll(env, {});

            if (result.ok) {
                setLedgers(result.data);
            } else {
                setError(String(result.error));
            }

            setLoading(false);
        },
        [env]
    );

    const loadOne = useCallback(
        async (query?: Partial<Ledger>): Promise<Result<Ledger>> => {
            setLoading(true);
            setError(null);

            const result = await ledgerService.select(env, query);

            if (!result.ok) {
                setError(String(result.error));
            }

            setLoading(false);
            return result;
        },
        [env]
    );

    const create = useCallback(
        async (ledger: Ledger): Promise<Result<Ledger>> => {
            setLoading(true);
            setError(null);

            const result = await ledgerService.insert(env, ledger);

            if (result.ok) {
                setLedgers((prev) => [...prev, result.data]);
            } else {
                setError(String(result.error));
            }

            setLoading(false);
            return result;
        },
        [env]
    );

    return {
        ledgers,
        loading,
        error,

        loadAll,
        loadOne,
        create,
    };
}
