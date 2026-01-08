"use client";

import { DashboardLayout } from "@/components/dashboard-layout";
import { PageHeader } from "@/components/page-header";
import { useEnvironment } from "@/lib/hooks/use-environment";
import { useMetrics } from "@/lib/hooks/use-metrics";
import Loading from "@/components/ui/loading";
import Refresh from "@/components/ui/refresh";
import { Column, DataTable } from "@/components/data-table";
import { useEffect } from "react";
import { Metric } from "@/interfaces/entities/metric";
import { useToastContext } from "@/components/toast-provider";

export default function MetricsPage() {
    const { environment } = useEnvironment();
    const { metrics, loading, error, loadAll } = useMetrics(environment);
    const { error: toastError } = useToastContext();

    useEffect(() => {
        loadAll();
    }, [loadAll]);

    useEffect(() => {
        if (error) {
            toastError("Metrics error", `There was a problem: ${error}`);
        }
    }, [error, toastError]);

    const columns: Column<Metric>[] = [
        {
            key: "id",
            label: "ID",
            render: (id) => <p className="font-medium">{id}</p>,
        },
        {
            key: "executions_ok",
            label: "OK",
            render: (value) => (
                <span className="font-mono text-success">{value}</span>
            ),
        },
        {
            key: "executions_err",
            label: "Errors",
            render: (value) => <span className={`font-mono `}>{value}</span>,
        },
        {
            key: "consecutive_errors",
            label: "Consecutive errors",
            render: (value) => <span>{value}</span>,
        },
        {
            key: "active_posting",
            label: "Active posting",
            render: (value) => <span className="font-mono">{value}</span>,
        },
        {
            key: "max_active_posting",
            label: "Max active posting",
            render: (value) => (
                <span className="font-mono text-muted-foreground">{value}</span>
            ),
        },
        {
            key: "max_execution_time",
            label: "Max time (ms)",
            render: (value) => <span className="font-mono">{value}</span>,
        },
        {
            key: "slowest_duration",
            label: "Slowest (ms)",
            render: (value) => (
                <span className="font-mono text-warning">{value}</span>
            ),
        },
        {
            key: "skipped_due_to_lock",
            label: "Skipped",
            render: (value) => <span className={`font-mono `}>{value}</span>,
        },
        {
            key: "last_success",
            label: "Last Success",
            render: (value) =>
                value ? (
                    <span className="font-mono">
                        {new Date(value).toLocaleString()}
                    </span>
                ) : (
                    <span className="text-muted-foreground">—</span>
                ),
        },
        {
            key: "last_error",
            label: "Last Error",
            render: (value) =>
                value ? (
                    <span className="font-mono text-destructive">
                        {new Date(value).toLocaleString()}
                    </span>
                ) : (
                    <span className="text-muted-foreground">—</span>
                ),
        },
        {
            key: "creation_date",
            label: "Created",
            render: (value) => <span className="font-mono">{value}</span>,
        },
    ];

    return (
        <DashboardLayout>
            <div className="flex h-full flex-col">
                <PageHeader
                    title="Metrics"
                    description="Runtime and historical performance metrics"
                    showEnvironmentSelector
                    actions={
                        <Refresh onRefresh={loadAll} isRefreshing={loading} />
                    }
                />

                <div className="flex-1 overflow-auto p-6">
                    {loading ? (
                        <Loading />
                    ) : (
                        <DataTable
                            columns={columns}
                            data={metrics}
                            searchPlaceholder="Search ledger entries..."
                            showEnvironment
                        />
                    )}
                </div>
            </div>
        </DashboardLayout>
    );
}
