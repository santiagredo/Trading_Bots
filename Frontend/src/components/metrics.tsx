"use client";

import { useEffect } from "react";
import { TrendingUp, AlertTriangle, Activity, Clock } from "lucide-react";

import { useMetrics } from "@/lib/hooks/use-metrics";
import { StatCard } from "@/components/stat-card";
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from "@/components/ui/card";
import { durationToMs } from "@/interfaces/entities/metric";

interface CriticalMetricsProps {
    env: string;
    refreshKey: number;
}

export function CriticalMetrics({ env, refreshKey }: CriticalMetricsProps) {
    const {
        activeMetric: metric,
        loading,
        error,
        loadWithRuntime,
    } = useMetrics(env);

    useEffect(() => {
        if (env) {
            loadWithRuntime();
        }
    }, [env, refreshKey, loadWithRuntime]);

    if (loading) {
        return (
            <p className="text-sm text-muted-foreground">Loading metrics…</p>
        );
    }

    if (error) {
        return (
            <p className="text-sm text-destructive">
                Failed to load metrics: {error}
            </p>
        );
    }

    if (!metric) {
        return (
            <p className="text-sm text-muted-foreground">
                No active metric available
            </p>
        );
    }

    return (
        <div className="grid gap-6">
            {/* Top stats */}
            <div className="grid gap-4 md:grid-cols-4">
                <StatCard
                    title="Executions OK"
                    value={metric.executions_ok}
                    description="Successful runs"
                    icon={TrendingUp}
                />

                <StatCard
                    title="Executions Error"
                    value={metric.executions_err}
                    description="Failed runs"
                    icon={AlertTriangle}
                />

                <StatCard
                    title="Active Posting"
                    value={metric.active_posting}
                    description={`Max ${metric.max_active_posting}`}
                    icon={Activity}
                />

                <StatCard
                    title="Max Exec Time"
                    value={`${durationToMs(metric.max_execution_time)} ms`}
                    description="Worst execution"
                    icon={Clock}
                />
            </div>

            {/* Detail cards */}
            <div className="grid gap-6 md:grid-cols-2">
                <Card>
                    <CardHeader>
                        <CardTitle>Execution Metrics</CardTitle>
                        <CardDescription>
                            Aggregated execution performance
                        </CardDescription>
                    </CardHeader>

                    <CardContent className="space-y-4">
                        <MetricRow
                            label="Total Execution Time"
                            value={`${durationToMs(
                                metric.total_execution_time
                            )} ms`}
                        />
                        <MetricRow
                            label="Slowest Duration"
                            value={`${durationToMs(
                                metric.slowest_duration
                            )} ms`}
                        />
                        <MetricRow
                            label="Skipped Due To Lock"
                            value={metric.skipped_due_to_lock}
                        />
                        <MetricRow
                            label="Consecutive Errors"
                            value={metric.consecutive_errors}
                            danger
                        />
                    </CardContent>
                </Card>

                <Card>
                    <CardHeader>
                        <CardTitle>Runtime State</CardTitle>
                        <CardDescription>
                            Last known execution state
                        </CardDescription>
                    </CardHeader>

                    <CardContent className="space-y-4">
                        <MetricRow
                            label="Last Success"
                            value={
                                metric.last_success
                                    ? new Date(
                                          metric.last_success
                                      ).toLocaleString()
                                    : "—"
                            }
                        />
                        <MetricRow
                            label="Last Error"
                            value={
                                metric.last_error
                                    ? new Date(
                                          metric.last_error
                                      ).toLocaleString()
                                    : "—"
                            }
                        />
                    </CardContent>
                </Card>
            </div>
        </div>
    );
}

function MetricRow({
    label,
    value,
    danger,
}: {
    label: string;
    value: React.ReactNode;
    danger?: boolean;
}) {
    return (
        <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">{label}</span>
            <span
                className={`font-mono text-sm font-medium ${
                    danger ? "text-destructive" : ""
                }`}
            >
                {value}
            </span>
        </div>
    );
}
