import { useCallback, useState } from "react";

import { Configuration } from "@/interfaces/configuration";
import { CacheHealthCheck } from "@/interfaces/health-check";
import { configurationService } from "../services/configuration";
import { format_datetime } from "../utils";

export function useEngineConfiguration() {
    /* =======================
     * configuration state
     * ======================= */

    const [configuration, setConfiguration] = useState<Configuration>({});

    /* =======================
     * health / runtime state
     * ======================= */

    const [healthCheck, setHealthCheck] = useState<CacheHealthCheck | null>(
        null
    );

    const [engineRunning, setEngineRunning] = useState<boolean>(false);

    const [lastRefresh, setLastRefresh] = useState<string>(
        format_datetime(new Date())
    );

    /* =======================
     * ui state
     * ======================= */

    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    /* =======================
     * loaders
     * ======================= */

    const loadConfiguration = useCallback(async () => {
        setLoading(true);
        setError(null);

        const result = await configurationService.getConfiguration();

        if (result.ok) {
            setConfiguration(result.data);
        } else {
            setError(String(result.error));
        }

        setLoading(false);
    }, []);

    const loadHealthCheck = useCallback(async () => {
        setLoading(true);
        setError(null);

        const result = await configurationService.getHealthCheck();

        if (result.ok) {
            setHealthCheck(result.data);
            setEngineRunning(true);
        } else {
            setHealthCheck(null);
            setEngineRunning(false);
            setError(String(result.error));
        }

        setLastRefresh(format_datetime(new Date()));
        setLoading(false);
    }, []);

    /* =======================
     * mutations
     * ======================= */

    const saveConfiguration = useCallback(async (config: Configuration) => {
        setLoading(true);
        setError(null);

        const result = await configurationService.saveConfiguration(config);

        if (result.ok) {
            setConfiguration(result.data);
        } else {
            setError(String(result.error));
        }

        setLoading(false);
        return result;
    }, []);

    const saveEngineState = useCallback(async () => {
        setLoading(true);
        setError(null);

        const result = await configurationService.setRunning(engineRunning);

        if (!result.ok) {
            setError(String(result.error));
        }

        setLoading(false);
        return result;
    }, [engineRunning]);

    /* =======================
     * public api
     * ======================= */

    return {
        // configuration
        configuration,
        loadConfiguration,
        saveConfiguration,
        setConfiguration,

        // health / runtime
        healthCheck,
        loadHealthCheck,
        lastRefresh,

        engineRunning,
        setEngineRunning,
        saveEngineState,

        // ui
        loading,
        error,
    };
}
