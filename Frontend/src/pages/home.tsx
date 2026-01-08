"use client";

import { useEffect, useState } from "react";
import { DashboardLayout } from "@/components/dashboard-layout";
import { PageHeader } from "@/components/page-header";
import { EngineOverview } from "@/components/engine-overview";
import Refresh from "@/components/ui/refresh";
import Loading from "@/components/ui/loading";
import { useToastContext } from "@/components/toast-provider";
import { useUserCommands } from "@/lib/hooks/use-user-commands";
import { UserCommand } from "@/types/user-commands";
import { EngineControlPanel } from "@/components/engine-control-panel";
import { useEngineConfiguration } from "@/lib/hooks/use-engine-configuration";
import { Environment } from "@/lib/hooks/use-environment";
import { CriticalMetrics } from "@/components/metrics";

export function HomePage() {
    const [refreshKey, setRefreshKey] = useState(0);

    const {
        healthCheck,
        engineRunning,
        lastRefresh,
        loading: healthLoading,
        error: healthError,
        loadHealthCheck,
    } = useEngineConfiguration();

    const {
        execute,
        loading: commandLoading,
        error: commandError,
    } = useUserCommands();

    const { error: toastError } = useToastContext();

    const handleCommand = async (env: Environment, command: UserCommand) => {
        const ok = await execute(env, command);

        if (ok) {
            await loadHealthCheck();
            setRefreshKey((k) => k + 1);
        }
    };

    const handleRefresh = async () => {
        await loadHealthCheck();
        setRefreshKey((k) => k + 1);
    };

    useEffect(() => {
        if (healthError) {
            toastError("Health Check error", healthError);
        }

        if (commandError) {
            toastError("Engine command error", commandError);
        }
    }, [commandError, toastError, healthError]);

    useEffect(() => {
        loadHealthCheck();
    }, [loadHealthCheck]);

    return (
        <DashboardLayout>
            <div className="flex h-full flex-col">
                <PageHeader
                    title="Dashboard Overview"
                    description="Monitor your crypto trading bot engine and performance"
                    actions={
                        <Refresh
                            onRefresh={handleRefresh}
                            isRefreshing={healthLoading}
                        />
                    }
                />

                <div className="flex-1 overflow-auto p-6">
                    {commandLoading ? (
                        <Loading />
                    ) : (
                        <div className="grid gap-6">
                            <EngineOverview
                                healthCheck={healthCheck}
                                engineRunning={engineRunning}
                                lastRefresh={lastRefresh}
                                loading={commandLoading}
                            />

                            <div className="grid grid-cols-2 gap-6">
                                <EngineControlPanel
                                    engineRunning={engineRunning}
                                    onCommand={handleCommand}
                                    loading={commandLoading}
                                    env="dev"
                                    healthCheck={healthCheck}
                                />

                                <EngineControlPanel
                                    engineRunning={engineRunning}
                                    onCommand={handleCommand}
                                    loading={commandLoading}
                                    env="prod"
                                    healthCheck={healthCheck}
                                />
                            </div>

                            <div className="grid grid-cols-2 gap-6">
                                <CriticalMetrics
                                    env={"dev"}
                                    refreshKey={refreshKey}
                                />

                                <CriticalMetrics
                                    env={"prod"}
                                    refreshKey={refreshKey}
                                />
                            </div>
                        </div>
                    )}
                </div>
            </div>
        </DashboardLayout>
    );
}
