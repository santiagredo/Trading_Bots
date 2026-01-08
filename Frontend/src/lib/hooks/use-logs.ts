import { useCallback, useState } from "react";
import { logService } from "@/lib/services/logs";
import { ErrorLog, IntegrationLog } from "@/interfaces/entities/logs";

export function useLogs(env: string) {
    const [errorLogs, setErrorLogs] = useState<ErrorLog[]>([]);
    const [integrationLogs, setIntegrationLogs] = useState<IntegrationLog[]>(
        []
    );

    const [errorLogLoading, setErrorLogLoading] = useState(false);
    const [integrationLogLoading, setintegrationLogLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    // =======================
    // loaders
    // =======================

    const loadErrorLogs = useCallback(async () => {
        setErrorLogLoading(true);
        setError(null);

        const result = await logService.error.selectAll(env);

        if (result.ok) {
            setErrorLogs(result.data);
        } else {
            setError(String(result.error));
        }

        setErrorLogLoading(false);
    }, [env]);

    const loadIntegrationLogs = useCallback(async () => {
        setintegrationLogLoading(true);
        setError(null);

        const result = await logService.integration.selectAll(env);

        if (result.ok) {
            setIntegrationLogs(result.data);
        } else {
            setError(String(result.error));
        }

        setintegrationLogLoading(false);
    }, [env]);

    return {
        // state
        errorLogs,
        integrationLogs,
        errorLogLoading,
        integrationLogLoading,
        error,

        // loaders
        loadErrorLogs,
        loadIntegrationLogs,
    };
}
