"use client";

import { useEffect } from "react";
import { DashboardLayout } from "@/components/dashboard-layout";
import { PageHeader } from "@/components/page-header";
import { DataTable, Column } from "@/components/data-table";
import { Badge } from "@/components/ui/badge";
import Loading from "@/components/ui/loading";
import Refresh from "@/components/ui/refresh";
import { useEnvironment } from "@/lib/hooks/use-environment";
import { useToastContext } from "@/components/toast-provider";
import { useLedgers } from "@/lib/hooks/use-ledgers";
import { Ledger } from "@/interfaces/entities/ledger";

export default function LedgersPage() {
    const { environment } = useEnvironment();
    const { error: toastError } = useToastContext();

    const { ledgers, loading, error, loadAll } = useLedgers(environment);

    // initial load + env change
    useEffect(() => {
        loadAll();
    }, [loadAll]);

    // error toast
    useEffect(() => {
        if (error) {
            toastError("Ledgers error", `There was a problem: ${error}`);
        }
    }, [error, toastError]);

    const columns: Column<Ledger>[] = [
        {
            key: "id",
            label: "ID",
            render: (id) => <p className="font-medium">{id}</p>,
        },
        {
            key: "record_type_id",
            label: "Type",
            render: (value) => <Badge variant="outline">{String(value)}</Badge>,
        },
        {
            key: "asset_id",
            label: "Asset",
            render: (value) => <p className="font-medium">{value}</p>,
        },
        {
            key: "free_amount",
            label: "Amount",
            render: (value) => {
                const amount = Number(value);
                return (
                    <span
                        className={`font-mono ${
                            amount >= 0 ? "text-success" : "text-destructive"
                        }`}
                    >
                        {amount >= 0 ? "+" : ""}
                        {amount.toFixed(4)}
                    </span>
                );
            },
        },
        {
            key: "free_new_balance",
            label: "Balance",
            render: (value) => (
                <span className="font-mono">{Number(value).toFixed(4)}</span>
            ),
        },
        {
            key: "creation_date",
            label: "Timestamp",
            render: (value) => <span className="font-mono">{value}</span>,
        },
    ];

    return (
        <DashboardLayout>
            <div className="flex h-full flex-col">
                <PageHeader
                    title="Ledgers"
                    description="Transaction history and account ledger entries"
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
                            data={ledgers}
                            searchPlaceholder="Search ledger entries..."
                            showEnvironment
                        />
                    )}
                </div>
            </div>
        </DashboardLayout>
    );
}
