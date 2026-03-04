import { useCallback, useMemo, useState } from "react";
import { IntegrationSetting } from "@/interfaces/integration-setting";
import { QueryOptions } from "@/interfaces/query-options";
import { integrationSettingService } from "../services/integrations-settings";

export function useIntegrationSettings(env: string) {
    /* =======================
     * database state
     * ======================= */

    const [settings, setSettings] = useState<IntegrationSetting[]>([]);

    /* =======================
     * memory/runtime state
     * ======================= */

    const [activeSettings, setActiveSettings] = useState<IntegrationSetting[]>(
        [],
    );

    /* =======================
     * merged ui state
     * ======================= */

    const uiSettings = useMemo(
        () => mergeSettings(settings, activeSettings),
        [settings, activeSettings],
    );

    /* =======================
     * ui state
     * ======================= */

    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    /* =======================
     * loaders - database
     * ======================= */

    const loadAll = useCallback(
        async (filters?: Partial<IntegrationSetting>, query?: QueryOptions) => {
            setLoading(true);
            setError(null);

            const result = await integrationSettingService.selectAll(
                env,
                filters,
                query,
            );

            if (result.ok) {
                setSettings(result.data);
            } else {
                setError(String(result.error));
            }

            setLoading(false);
        },
        [env],
    );

    const loadOne = useCallback(
        async (filters: Partial<IntegrationSetting>) => {
            setLoading(true);
            setError(null);

            const result = await integrationSettingService.select(env, filters);

            if (result.ok) {
                setSettings([result.data]);
            } else {
                setError(String(result.error));
            }

            setLoading(false);
            return result;
        },
        [env],
    );

    /* =======================
     * loaders - memory
     * ======================= */

    const loadActiveAll = useCallback(async () => {
        setLoading(true);
        setError(null);

        const result = await integrationSettingService.getMemoryAll(env);

        if (result.ok) {
            setActiveSettings(result.data);
        } else {
            setActiveSettings([]);
            setError(String(result.error));
        }

        setLoading(false);
    }, [env]);

    const loadActiveOne = useCallback(
        async (id: number) => {
            setLoading(true);
            setError(null);

            const result = await integrationSettingService.getMemory(env, id);

            if (result.ok) {
                setActiveSettings((prev) => {
                    const filtered = prev.filter((s) => s.id !== id);
                    return [...filtered, result.data];
                });
            } else {
                setError(String(result.error));
            }

            setLoading(false);
            return result;
        },
        [env],
    );

    /* =======================
     * mutations
     * ======================= */

    const update = useCallback(
        async (setting: IntegrationSetting) => {
            if (!setting.id) {
                setError("IntegrationSetting ID is required");
                return { ok: false, error: "Missing integration setting id" };
            }

            setLoading(true);
            setError(null);

            const result = await integrationSettingService.update(env, setting);

            if (result.ok) {
                setSettings((prev) =>
                    prev.map((s) =>
                        s.id === result.data.id ? result.data : s,
                    ),
                );
            } else {
                setError(String(result.error));
            }

            setLoading(false);
            return result;
        },
        [env],
    );

    /* =======================
     * combined loader
     * ======================= */

    const loadWithMemory = useCallback(async () => {
        await Promise.all([loadAll(), loadActiveAll()]);
    }, [loadAll, loadActiveAll]);

    /* =======================
     * public api
     * ======================= */

    return {
        settings,
        activeSettings,
        uiSettings,

        loading,
        error,

        loadAll,
        loadOne,

        loadActiveAll,
        loadActiveOne,

        update,

        loadWithMemory,
    };
}

/* =======================
 * helpers
 * ======================= */

function mergeSettings(db: IntegrationSetting[], memory: IntegrationSetting[]) {
    const memoryMap = new Map<number, IntegrationSetting>();

    for (const m of memory) {
        if (m.id != null) {
            memoryMap.set(m.id, m);
        }
    }

    return db.map((dbSetting) => {
        const mem = dbSetting.id ? memoryMap.get(dbSetting.id) : undefined;

        const runtimeLoaded = !!mem;

        return {
            ...dbSetting,
            runtimeLoaded,
        };
    });
}
