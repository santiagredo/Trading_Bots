"use client";

import { useState } from "react";
import { DashboardLayout } from "@/components/dashboard-layout";
import { PageHeader } from "@/components/page-header";
import { Column, DataTable } from "@/components/data-table";
import { Button } from "@/components/ui/button";
import { Plus } from "lucide-react";
import { useEnvironment } from "@/lib/hooks/use-environment";
import React from "react";
import { useToastContext } from "@/components/toast-provider";
import { Action } from "@/interfaces/entities/action";
import Loading from "@/components/ui/loading";
import { DataModal, DataModalConfig } from "@/components/data-modal";
import { useActions } from "@/lib/hooks/use-actions";
import { RuntimeStatusBadge } from "@/components/ui/runtime-status";
import Refresh from "@/components/ui/refresh";

export default function ActionsPage() {
    const { environment } = useEnvironment();
    const { success, error: toastError } = useToastContext();

    const {
        uiActions,
        loading,
        error,
        loadWithRuntime,
        create,
        update,
        remove,
    } = useActions(environment);

    const [isModalOpen, setIsModalOpen] = useState(false);
    const [editingData, setEditingData] = useState<Action | null>(null);

    const onSave = async (action: Action) => {
        const result = editingData?.id
            ? await update(action)
            : await create(action);

        if (!result.ok) {
            toastError("Action save error", String(result.error));
            return;
        }

        success("Action saved", "Changes saved successfully");
        setIsModalOpen(false);
    };

    const onDelete = async (action: Action) => {
        const result = await remove(action);

        if (!result.ok) {
            toastError("Delete error", String(result.error));
            return;
        }

        success("Action deleted", "The action was removed successfully");
    };

    React.useEffect(() => {
        loadWithRuntime();
    }, [loadWithRuntime]);

    React.useEffect(() => {
        if (error) {
            toastError("Actions error", `There was a problem: ${error}`);
        }
    }, [error, toastError]);

    const columns: Column<any>[] = [
        {
            key: "id",
            label: "ID",
            render: (id) => <p className="font-medium">{id}</p>,
        },
        {
            key: "strategy_id",
            label: "Strategy ID",
            render: (strategy_id) => (
                <p className="font-medium">{strategy_id}</p>
            ),
        },
        {
            key: "pair_id",
            label: "Pair ID",
            render: (pair_id) => <p className="font-medium">{pair_id}</p>,
        },
        {
            key: "is_active",
            label: "Is active",
            render: (is_active) => (
                <p className="font-medium">{is_active ? "Yes" : "No"}</p>
            ),
        },
        {
            key: "is_sell",
            label: "Side",
            render: (is_sell) => (
                <p className="font-medium">{is_sell ? "Sell" : "Buy"}</p>
            ),
        },
        {
            key: "is_quote_asset",
            label: "Asset",
            render: (is_quote_asset) => (
                <p className="font-medium">
                    {is_quote_asset ? "Quote" : "Base"}
                </p>
            ),
        },
        {
            key: "is_percentage",
            label: "Percentage",
            render: (is_percentage) => (
                <p className="font-medium">{is_percentage ? "Yes" : "No"}</p>
            ),
        },
        {
            key: "value",
            label: "Value",
            render: (value) => <p className="font-medium">{value}</p>,
        },
        {
            key: "runtimeStatus",
            label: "Runtime",
            render: (_, row) => (
                <RuntimeStatusBadge status={row.runtimeStatus} />
            ),
        },
    ];

    const modalConfig: DataModalConfig<Action> = {
        title: "action",
        description: "Create or edit a trading action",
        fields: [
            {
                key: "id",
                label: "ID",
                type: "number",
                disabled: true,
            },
            {
                key: "strategy_id",
                label: "Strategy ID",
                type: "number",
                required: true,
                placeholder: "Associated strategy ID",
            },
            {
                key: "pair_id",
                label: "Pair ID",
                type: "number",
                required: true,
                placeholder: "Trading pair ID",
            },
            {
                key: "is_active",
                label: "Is active",
                type: "checkbox",
            },
            {
                key: "is_sell",
                label: "Sell action",
                type: "checkbox",
            },
            {
                key: "is_quote_asset",
                label: "Use quote asset",
                type: "checkbox",
            },
            {
                key: "is_percentage",
                label: "Is percentage",
                type: "checkbox",
            },
            {
                key: "value",
                label: "Value",
                type: "number",
                required: true,
                placeholder: "Action value",
            },
        ],
        onSave,
    };

    return (
        <DashboardLayout>
            <div className="flex h-full flex-col">
                <PageHeader
                    title="Actions"
                    description="Manage your trading actions"
                    showEnvironmentSelector
                    actions={
                        <>
                            <Refresh
                                onRefresh={loadWithRuntime}
                                isRefreshing={loading}
                            />
                            <Button
                                size="sm"
                                className="gap-2"
                                onClick={() => {
                                    setEditingData(null);
                                    setIsModalOpen(true);
                                }}
                            >
                                <Plus className="h-4 w-4" />
                                New Action
                            </Button>
                        </>
                    }
                />
                <div className="flex-1 overflow-auto p-6">
                    {loading ? (
                        <Loading />
                    ) : (
                        <>
                            <DataTable
                                columns={columns}
                                data={uiActions}
                                searchPlaceholder="Search actions..."
                                showEnvironment
                                onEdit={(row) => {
                                    setEditingData(row);
                                    setIsModalOpen(true);
                                }}
                                onDelete={onDelete}
                                deleteConfirmTitle="Delete Action?"
                                deleteConfirmDescription="This will permanently delete this action. This action cannot be undone."
                            />

                            <DataModal
                                open={isModalOpen}
                                onOpenChange={setIsModalOpen}
                                config={modalConfig}
                                data={editingData}
                            />
                        </>
                    )}
                </div>
            </div>
        </DashboardLayout>
    );
}
