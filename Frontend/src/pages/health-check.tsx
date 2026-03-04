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
import { Server, Clock } from "lucide-react";
import { useEngineConfiguration } from "@/lib/hooks/use-engine-configuration";
import { format_datetime } from "@/lib/utils";
import {
    LifecycleState,
    mapRuntimeState,
    mapSocketState,
    SocketState,
} from "@/types/life-cycle-state";
import { DatabaseStatusCard } from "@/components/ui/database-status-card";

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
                                    value={format_datetime(
                                        healthCheck.engine_startup_date,
                                    )}
                                    icon={Clock}
                                />
                                <Stat
                                    title="Startup Date"
                                    value={healthCheck.engine_startup_date}
                                    icon={Server}
                                />

                                <DatabaseStatusCard healthCheck={healthCheck} />
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
                                        value={mapRuntimeState(
                                            healthCheck.cache_actions_dev_status,
                                        )}
                                    />

                                    <HealthRow
                                        label="Assets"
                                        value={mapRuntimeState(
                                            healthCheck.cache_assets_dev_status,
                                        )}
                                    />

                                    <HealthRow
                                        label="Indicators"
                                        value={mapRuntimeState(
                                            healthCheck.cache_indicators_dev_status,
                                        )}
                                    />

                                    <HealthRow
                                        label="Integrations"
                                        value={mapRuntimeState(
                                            healthCheck.cache_integrations_dev_status,
                                        )}
                                    />

                                    <HealthRow
                                        label="Integrations settings"
                                        value={mapRuntimeState(
                                            healthCheck.cache_integrations_settings_dev_status,
                                        )}
                                    />

                                    <HealthRow
                                        label="Metrics"
                                        value={mapRuntimeState(
                                            healthCheck.cache_metrics_dev_status,
                                        )}
                                    />

                                    <HealthRow
                                        label="Order status"
                                        value={mapRuntimeState(
                                            healthCheck.cache_order_status_dev_status,
                                        )}
                                    />

                                    <HealthRow
                                        label="Pairs"
                                        value={mapRuntimeState(
                                            healthCheck.cache_pairs_dev_status,
                                        )}
                                    />

                                    <HealthRow
                                        label="Strategies"
                                        value={mapRuntimeState(
                                            healthCheck.cache_strategies_dev_status,
                                        )}
                                    />

                                    <HealthRow
                                        label="Subscribed Indicators"
                                        value={mapRuntimeState(
                                            healthCheck.cache_subscribed_indicators_status,
                                        )}
                                    />

                                    <HealthRow
                                        label="Tasks"
                                        value={mapRuntimeState(
                                            healthCheck.cache_tasks_dev_status,
                                        )}
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
                                        value={mapRuntimeState(
                                            healthCheck.cache_actions_prod_status,
                                        )}
                                    />

                                    <HealthRow
                                        label="Assets"
                                        value={mapRuntimeState(
                                            healthCheck.cache_assets_prod_status,
                                        )}
                                    />

                                    <HealthRow
                                        label="Indicators"
                                        value={mapRuntimeState(
                                            healthCheck.cache_indicators_prod_status,
                                        )}
                                    />

                                    <HealthRow
                                        label="Integrations"
                                        value={mapRuntimeState(
                                            healthCheck.cache_integrations_prod_status,
                                        )}
                                    />

                                    <HealthRow
                                        label="Integrations settings"
                                        value={mapRuntimeState(
                                            healthCheck.cache_integrations_settings_prod_status,
                                        )}
                                    />

                                    <HealthRow
                                        label="Metrics"
                                        value={mapRuntimeState(
                                            healthCheck.cache_metrics_prod_status,
                                        )}
                                    />

                                    <HealthRow
                                        label="Order status"
                                        value={mapRuntimeState(
                                            healthCheck.cache_order_status_prod_status,
                                        )}
                                    />

                                    <HealthRow
                                        label="Pairs"
                                        value={mapRuntimeState(
                                            healthCheck.cache_pairs_prod_status,
                                        )}
                                    />

                                    <HealthRow
                                        label="Strategies"
                                        value={mapRuntimeState(
                                            healthCheck.cache_strategies_prod_status,
                                        )}
                                    />

                                    <HealthRow
                                        label="Subscribed Indicators"
                                        value={mapRuntimeState(
                                            healthCheck.cache_subscribed_indicators_status,
                                        )}
                                    />

                                    <HealthRow
                                        label="Tasks"
                                        value={mapRuntimeState(
                                            healthCheck.cache_tasks_prod_status,
                                        )}
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
                                    <SocketHealthRow
                                        label="Websocket"
                                        value={
                                            healthCheck.cache_websocket_status
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
    const status =
        variant === "status" ? resolveStatus(mapRuntimeState(value)) : null;

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

function resolveStatus(value: LifecycleState) {
    switch (value) {
        case "Running":
            return {
                ok: true,
                label: "RUNNING",
                iconBg: "bg-emerald-500/10",
                iconColor: "text-emerald-500",
                textColor: "text-emerald-500",
            };

        case "Starting":
            return {
                ok: false,
                label: "STARTING",
                iconBg: "bg-amber-500/10",
                iconColor: "text-amber-500",
                textColor: "text-amber-500",
            };

        case "Stopping":
            return {
                ok: false,
                label: "STOPPING",
                iconBg: "bg-orange-500/10",
                iconColor: "text-orange-500",
                textColor: "text-orange-500",
            };

        case "Off":
        default:
            return {
                ok: false,
                label: "OFF",
                iconBg: "bg-red-500/10",
                iconColor: "text-red-500",
                textColor: "text-red-500",
            };
    }
}

function StatusBadge({ value }: { value: LifecycleState }) {
    const status = resolveStatus(value);

    return (
        <Badge
            variant={
                value === "Off"
                    ? "destructive"
                    : value === "Running"
                      ? "default"
                      : "secondary"
            }
        >
            {status.label}
        </Badge>
    );
}

function HealthRow({ label, value }: { label: string; value: LifecycleState }) {
    return (
        <div className="flex items-center justify-between text-sm">
            <span className="text-muted-foreground">{label}</span>
            <StatusBadge value={value} />
        </div>
    );
}

function resolveSocketStatus(value: SocketState) {
    switch (value) {
        case "Connected":
            return { label: "Connected", variant: "default" };

        case "Connecting":
        case "Reconnecting":
            return { label: value, variant: "secondary" };

        case "ShuttingDown":
            return { label: "Shutting Down", variant: "secondary" };

        case "Closed":
        case "Disconnected":
        default:
            return { label: value ?? "Disconnected", variant: "destructive" };
    }
}

function SocketStatusBadge({ value }: { value: SocketState }) {
    const status = resolveSocketStatus(value);

    return <Badge variant={status.variant as any}>{status.label}</Badge>;
}

function SocketHealthRow({
    label,
    value,
}: {
    label: string;
    value: string | undefined; // viene crudo del backend
}) {
    const mapped = mapSocketState(value);

    return (
        <div className="flex items-center justify-between text-sm">
            <span className="text-muted-foreground">{label}</span>
            <SocketStatusBadge value={mapped} />
        </div>
    );
}
