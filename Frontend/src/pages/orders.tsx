"use client";

import { useEffect } from "react";
import { DashboardLayout } from "@/components/dashboard-layout";
import { PageHeader } from "@/components/page-header";
import { Column, DataTable } from "@/components/data-table";
import { useEnvironment } from "@/lib/hooks/use-environment";
import { useOrders } from "@/lib/hooks/use-orders";
import { Order } from "@/interfaces/entities/order";
import Loading from "@/components/ui/loading";
import Refresh from "@/components/ui/refresh";
import { format_datetime } from "@/lib/utils";
import { Badge } from "@/components/ui/badge";
import { useToastContext } from "@/components/toast-provider";

export default function OrdersPage() {
    const { environment } = useEnvironment();
    const { error: toastError } = useToastContext();

    const { orders, loading, error, loadAll } = useOrders(environment);

    useEffect(() => {
        loadAll();
    }, [loadAll]);

    useEffect(() => {
        if (error) {
            toastError("Strategies error", `There was a problem: ${error}`);
        }
    }, [error, toastError]);

    const columns: Column<Order>[] = [
        {
            key: "id",
            label: "ID",
            render: (id) => <p className="font-medium">{id}</p>,
        },
        {
            key: "strategy_id",
            label: "Strategy",
            render: (strategy_id) => (
                <p className="font-medium">{strategy_id}</p>
            ),
        },
        {
            key: "is_sell",
            label: "Side",
            render: (is_sell) => (
                <Badge
                    variant={is_sell ? "outline" : "default"}
                    className={!is_sell ? "bg-success" : ""}
                >
                    {is_sell ? "SELL" : "BUY"}
                </Badge>
            ),
        },
        {
            key: "base_asset_amount",
            label: "Base Amount",
            render: (value) => <p className="font-mono">{value}</p>,
        },
        {
            key: "quote_asset_amount",
            label: "Quote Amount",
            render: (value) => <p className="font-mono">{value}</p>,
        },
        {
            key: "price_entry",
            label: "Entry Price",
            render: (value) => <p className="font-mono">{value}</p>,
        },
        {
            key: "price_target",
            label: "Target Price",
            render: (value) => <p className="font-mono">{value}</p>,
        },
        {
            key: "price_abort",
            label: "Abort Price",
            render: (value) => <p className="font-mono">{value}</p>,
        },
        {
            key: "creation_date",
            label: "Created",
            render: (date) => (date ? format_datetime(date.toString()) : "-"),
        },
        {
            key: "update_date",
            label: "Updated",
            render: (date) => (date ? format_datetime(date.toString()) : "-"),
        },
    ];

    return (
        <DashboardLayout>
            <div className="flex h-full flex-col">
                <PageHeader
                    title="Orders"
                    description="Monitor and manage trading orders"
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
                            data={orders}
                            searchPlaceholder="Search orders..."
                            showEnvironment
                        />
                    )}
                </div>
            </div>
        </DashboardLayout>
    );
}
