"use client";

import { useEffect } from "react";
import { DashboardLayout } from "@/components/dashboard-layout";
import { PageHeader } from "@/components/page-header";
import { Column, DataTable } from "@/components/data-table";
import { useEnvironment } from "@/lib/hooks/use-environment";
import { useLogs } from "@/lib/hooks/use-logs";
import Loading from "@/components/ui/loading";
import Refresh from "@/components/ui/refresh";
import { format_datetime } from "@/lib/utils";
import { Badge } from "@/components/ui/badge";
import { useToastContext } from "@/components/toast-provider";
import { ErrorLog, IntegrationLog } from "@/interfaces/entities/logs";

export default function LogsPage() {
    const { environment } = useEnvironment();
    const { error: toastError } = useToastContext();

    const {
        errorLogs,
        integrationLogs,
        errorLogLoading,
        integrationLogLoading,
        error,
        loadErrorLogs,
        loadIntegrationLogs,
    } = useLogs(environment);

    const loading = errorLogLoading && integrationLogLoading;

    // =======================
    // initial load
    // =======================

    useEffect(() => {
        loadErrorLogs();
        loadIntegrationLogs();
    }, [loadErrorLogs, loadIntegrationLogs]);

    // =======================
    // error toast
    // =======================

    useEffect(() => {
        if (error) {
            toastError("Logs error", `There was a problem: ${error}`);
        }
    }, [error, toastError]);

    // =======================
    // columns – error logs
    // =======================

    const errorLogColumns: Column<ErrorLog>[] = [
        {
            key: "id",
            label: "ID",
            render: (id) => <p className="font-medium">{id}</p>,
        },
        {
            key: "error_type",
            label: "Type",
            render: (value) => <Badge variant="destructive">{value}</Badge>,
        },
        {
            key: "creation_date",
            label: "Created",
            render: (date) => (date ? format_datetime(date.toString()) : "-"),
        },
        {
            key: "request",
            label: "Request",
            render: (value) => (
                <p className="font-mono truncate max-w-65">{value}</p>
            ),
        },
        {
            key: "error_details",
            label: "Details",
            render: (value) => (
                <p className="font-mono truncate max-w-65">{value}</p>
            ),
        },
        {
            key: "file_path",
            label: "File",
            render: (value) => (
                <p className="font-mono truncate max-w-60">{value}</p>
            ),
        },
        {
            key: "line_number",
            label: "Line",
            render: (value) => <p className="font-mono">{value}</p>,
        },
        {
            key: "function_name",
            label: "Function",
            render: (value) => (
                <p className="font-mono truncate max-w-50">{value}</p>
            ),
        },
    ];

    // =======================
    // columns – integration logs
    // =======================

    const integrationLogColumns: Column<IntegrationLog>[] = [
        {
            key: "id",
            label: "ID",
            render: (id) => <p className="font-medium">{id}</p>,
        },
        {
            key: "creation_date",
            label: "Created",
            render: (date) => (date ? format_datetime(date.toString()) : "-"),
        },
        {
            key: "integration_name",
            label: "Integration",
            render: (value) => <Badge variant="outline">{value}</Badge>,
        },
        {
            key: "url",
            label: "URL",
            render: (value) => (
                <p className="font-mono truncate max-w-65">{value}</p>
            ),
        },
        {
            key: "status_code",
            label: "Status",
            render: (value) => {
                const status = Number(value);

                return (
                    <Badge variant={status >= 400 ? "destructive" : "default"}>
                        {value}
                    </Badge>
                );
            },
        },
        {
            key: "execution_time_ms",
            label: "Exec Time",
            render: (value) => <p className="font-mono">{value} ms</p>,
        },
        {
            key: "request",
            label: "Request",
            render: (value) => {
                const text = value?.toString() ?? "";
                const trimmed =
                    text.length > 100 ? `${text.slice(0, 100)}…` : text;

                return <p className="font-mono truncate max-w-65">{trimmed}</p>;
            },
        },
        {
            key: "response",
            label: "Response",
            render: (value) => {
                const text = value?.toString() ?? "";
                const trimmed =
                    text.length > 100 ? `${text.slice(0, 100)}…` : text;

                return <p className="font-mono truncate max-w-65">{trimmed}</p>;
            },
        },
        {
            key: "error_message",
            label: "Error message",
            render: (value) => {
                const text = value?.toString() ?? "";
                const trimmed =
                    text.length > 100 ? `${text.slice(0, 100)}…` : text;

                return <p className="font-mono truncate max-w-65">{trimmed}</p>;
            },
        },
    ];

    // =======================
    // refresh
    // =======================

    const refreshAll = async () => {
        await Promise.all([loadErrorLogs(), loadIntegrationLogs()]);
    };

    return (
        <DashboardLayout>
            <div className="flex h-full flex-col">
                <PageHeader
                    title="Logs"
                    description="System error and integration activity logs"
                    showEnvironmentSelector
                    actions={
                        <Refresh
                            onRefresh={refreshAll}
                            isRefreshing={loading}
                        />
                    }
                />

                <div className="flex-1 overflow-auto p-6 space-y-8">
                    {loading ? (
                        <Loading />
                    ) : (
                        <>
                            {errorLogLoading ? (
                                <Loading />
                            ) : (
                                <div>
                                    <span>Error Logs</span>
                                    <DataTable
                                        columns={errorLogColumns}
                                        data={errorLogs}
                                        searchPlaceholder="Search error logs..."
                                        showEnvironment
                                    />
                                </div>
                            )}

                            {integrationLogLoading ? (
                                <Loading />
                            ) : (
                                <div>
                                    <span>Integration Logs</span>
                                    <DataTable
                                        columns={integrationLogColumns}
                                        data={integrationLogs}
                                        searchPlaceholder="Search integration logs..."
                                        showEnvironment
                                    />
                                </div>
                            )}
                        </>
                    )}
                </div>
            </div>
        </DashboardLayout>
    );
}
