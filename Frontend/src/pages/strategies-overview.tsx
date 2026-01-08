"use client";

import { useEffect } from "react";
import { DashboardLayout } from "@/components/dashboard-layout";
import { PageHeader } from "@/components/page-header";
import { Column, DataTable } from "@/components/data-table";
import { useEnvironment } from "@/lib/hooks/use-environment";
import { useStrategiesOverview } from "@/lib/hooks/use-strategies-overview";
import Loading from "@/components/ui/loading";
import Refresh from "@/components/ui/refresh";
import { useToastContext } from "@/components/toast-provider";
import { Strategy } from "@/interfaces/entities/strategy";

export default function StrategiesOverviewPage() {
    const { environment } = useEnvironment();
    const { error: toastError } = useToastContext();

    const { strategies, loading, error, loadStrategiesOverview } =
        useStrategiesOverview(environment);

    // =======================
    // initial load
    // =======================

    useEffect(() => {
        loadStrategiesOverview({} as Strategy);
    }, [loadStrategiesOverview]);

    // =======================
    // error toast
    // =======================

    useEffect(() => {
        if (error) {
            toastError(
                "Strategies overview error",
                `There was a problem: ${error}`
            );
        }
    }, [error, toastError]);

    // =======================
    // columns
    // =======================

    const columns: Column<any>[] = [
        {
            key: "strategy",
            label: "Strategy",
            render: (strategy) => <p>{strategy.name}</p>,
        },
        {
            key: "indicator",
            label: "Indicator",
            render: (indicator) => <p>{indicator.nick ?? indicator.symbol}</p>,
        },
        {
            key: "action",
            label: "Action",
            render: (action) => <p>{action.is_sell ? "SELL" : "BUY"}</p>,
        },
    ];

    // =======================
    // refresh
    // =======================

    const refresh = async () => {
        await loadStrategiesOverview({} as Strategy);
    };

    return (
        <DashboardLayout>
            <div className="flex h-full flex-col">
                <PageHeader
                    title="Strategies Overview"
                    description="Strategies with their indicators and actions"
                    showEnvironmentSelector
                    actions={
                        <Refresh onRefresh={refresh} isRefreshing={loading} />
                    }
                />

                <div className="flex-1 overflow-auto p-6">
                    {loading ? (
                        <Loading />
                    ) : (
                        <DataTable
                            columns={columns}
                            data={strategies}
                            searchPlaceholder="Search strategies..."
                            showEnvironment
                        />
                    )}
                </div>
            </div>
        </DashboardLayout>
    );
}
