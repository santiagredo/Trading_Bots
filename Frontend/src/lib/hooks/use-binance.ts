import { AccountInformation } from "@/interfaces/account-information";
import { useCallback, useState } from "react";
import { binanceService } from "../services/binance";

export function useBinance() {
    const [accountInformation, setAccountInformation] =
        useState<AccountInformation | null>(null);

    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    // =======================
    // loaders
    // =======================

    const load = useCallback(async () => {
        setLoading(true);
        setError(null);

        const result = await binanceService.get();

        if (result.ok) {
            setAccountInformation(result.data);
        } else {
            setError(String(result.error));
        }

        setLoading(false);
    }, []);

    return {
        accountInformation,
        loading,
        error,
        load,
    };
}
