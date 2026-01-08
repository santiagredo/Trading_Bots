"use client";

import { useEffect } from "react";
import { DashboardLayout } from "@/components/dashboard-layout";
import { PageHeader } from "@/components/page-header";
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import Loading from "@/components/ui/loading";
import Refresh from "@/components/ui/refresh";
import { Database, Server, Clock } from "lucide-react";
import { useEngineConfiguration } from "@/lib/hooks/use-engine-configuration";

export default function HealthCheckPage() {
    const { healthCheck, loading, error, loadHealthCheck } =
        useEngineConfiguration();

    useEffect(() => {
        loadHealthCheck();
    }, [loadHealthCheck]);

    return (
        <DashboardLayout>
            <div className="flex h-full flex-col">
                <PageHeader
                    title="System Health"
                    description="Overall backend, cache, healthCheckbase and websocket health"
                    actions={
                        <Refresh
                            onRefresh={loadHealthCheck}
                            isRefreshing={loading}
                        />
                    }
                />

                <div className="flex-1 overflow-auto p-6">
                    {loading ? (
                        <Loading />
                    ) : error ? (
                        <Card>
                            <CardContent className="p-6 text-red-500">
                                Error loading health check: {error}
                            </CardContent>
                        </Card>
                    ) : healthCheck ? (
                        <div className="grid gap-6">
                            {/* High level stats */}
                            <div className="grid gap-4 md:grid-cols-4">
                                <Stat
                                    title="Uptime"
                                    value={healthCheck.uptime}
                                    icon={Clock}
                                />
                                <Stat
                                    title="Startup Date"
                                    value={healthCheck.startup_date}
                                    icon={Server}
                                />
                                <Stat
                                    title="DB (Dev)"
                                    value={healthCheck.db_dev_conn_is_valid}
                                    icon={Database}
                                    variant="status"
                                />
                                <Stat
                                    title="DB (Prod)"
                                    value={healthCheck.db_prod_conn_is_valid}
                                    icon={Database}
                                    variant="status"
                                />
                            </div>

                            {/* Cache - Dev */}
                            <Card>
                                <CardHeader>
                                    <CardTitle>Cache · Development</CardTitle>
                                    <CardDescription>
                                        Initialization and runtime state
                                    </CardDescription>
                                </CardHeader>
                                <CardContent className="space-y-2">
                                    <HealthRow
                                        label="Actions"
                                        value={
                                            healthCheck.cache_actions_dev_is_initialized
                                        }
                                    />
                                    <HealthRow
                                        label="Assets"
                                        value={
                                            healthCheck.cache_assets_dev_is_initialized
                                        }
                                    />
                                    <HealthRow
                                        label="Indicators"
                                        value={
                                            healthCheck.cache_indicators_dev_is_initialized
                                        }
                                    />
                                    <HealthRow
                                        label="Pairs"
                                        value={
                                            healthCheck.cache_pairs_dev_is_initialized
                                        }
                                    />
                                    <HealthRow
                                        label="Status"
                                        value={
                                            healthCheck.cache_status_dev_is_initialized
                                        }
                                    />
                                    <HealthRow
                                        label="Strategies"
                                        value={
                                            healthCheck.cache_strategies_dev_is_initialized
                                        }
                                    />
                                    <HealthRow
                                        label="Subscribed Indicators"
                                        value={
                                            healthCheck.cache_subscribed_indicators_dev_is_initialized
                                        }
                                    />
                                    <HealthRow
                                        label="Tasks"
                                        value={
                                            healthCheck.cache_tasks_dev_is_initialized
                                        }
                                    />
                                </CardContent>
                            </Card>

                            {/* Cache - Prod */}
                            <Card>
                                <CardHeader>
                                    <CardTitle>Cache · Production</CardTitle>
                                    <CardDescription>
                                        Initialization and runtime state
                                    </CardDescription>
                                </CardHeader>
                                <CardContent className="space-y-2">
                                    <HealthRow
                                        label="Actions"
                                        value={
                                            healthCheck.cache_actions_prod_is_initialized
                                        }
                                    />
                                    <HealthRow
                                        label="Assets"
                                        value={
                                            healthCheck.cache_assets_prod_is_initialized
                                        }
                                    />
                                    <HealthRow
                                        label="Indicators"
                                        value={
                                            healthCheck.cache_indicators_prod_is_initialized
                                        }
                                    />
                                    <HealthRow
                                        label="Pairs"
                                        value={
                                            healthCheck.cache_pairs_prod_is_initialized
                                        }
                                    />
                                    <HealthRow
                                        label="Status"
                                        value={
                                            healthCheck.cache_status_prod_is_initialized
                                        }
                                    />
                                    <HealthRow
                                        label="Strategies"
                                        value={
                                            healthCheck.cache_strategies_prod_is_initialized
                                        }
                                    />
                                    <HealthRow
                                        label="Subscribed Indicators"
                                        value={
                                            healthCheck.cache_subscribed_indicators_prod_is_initialized
                                        }
                                    />
                                    <HealthRow
                                        label="Tasks"
                                        value={
                                            healthCheck.cache_tasks_prod_is_initialized
                                        }
                                    />
                                </CardContent>
                            </Card>

                            {/* Websocket */}
                            <Card>
                                <CardHeader>
                                    <CardTitle>Binance WebSocket</CardTitle>
                                    <CardDescription>
                                        Connection lifecycle state
                                    </CardDescription>
                                </CardHeader>
                                <CardContent className="space-y-2">
                                    <HealthRow
                                        label="Abort Handle Exists"
                                        value={
                                            healthCheck.cache_ws_binance_abort_handle_is_some
                                        }
                                    />
                                    <HealthRow
                                        label="Abort Handle Finished"
                                        value={
                                            healthCheck.cache_ws_binance_abort_handle_is_finished
                                        }
                                    />
                                </CardContent>
                            </Card>
                        </div>
                    ) : null}
                </div>
            </div>
        </DashboardLayout>
    );
}

function Stat({
    title,
    value,
    icon: Icon,
    variant = "info",
}: {
    title: string;
    value: string;
    icon: React.ElementType;
    variant?: "info" | "status";
}) {
    const status = variant === "status" ? resolveStatus(value) : null;

    return (
        <Card>
            <CardContent className="flex items-center gap-4 p-4">
                <div
                    className={`rounded-xl p-2 ${
                        status ? status.iconBg : "bg-muted"
                    }`}
                >
                    <Icon
                        className={`h-5 w-5 ${
                            status ? status.iconColor : "text-muted-foreground"
                        }`}
                    />
                </div>

                <div className="flex flex-col">
                    <span className="text-sm text-muted-foreground">
                        {title}
                    </span>

                    {status ? (
                        <span
                            className={`font-mono text-sm font-semibold ${status.textColor}`}
                        >
                            {status.label}
                        </span>
                    ) : (
                        <span className="font-mono text-sm">{value}</span>
                    )}
                </div>
            </CardContent>
        </Card>
    );
}

function StatusBadge({ value }: { value: string }) {
    const ok = value === "true" || value === "ok" || value === "initialized";

    return (
        <Badge variant={ok ? "default" : "destructive"}>
            {ok ? "OK" : "ERROR"}
        </Badge>
    );
}

function HealthRow({ label, value }: { label: string; value: string }) {
    return (
        <div className="flex items-center justify-between text-sm">
            <span className="text-muted-foreground">{label}</span>
            <StatusBadge value={value} />
        </div>
    );
}

function resolveStatus(value: string) {
    const ok = value === "true" || value === "ok" || value === "initialized";

    return {
        ok,
        label: ok ? "OK" : "ERROR",
        iconBg: ok ? "bg-emerald-500/10" : "bg-red-500/10",
        iconColor: ok ? "text-emerald-500" : "text-red-500",
        textColor: ok ? "text-emerald-500" : "text-red-500",
    };
}
