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
import { Indicator } from "@/interfaces/entities/indicator";
import Loading from "@/components/ui/loading";
import { DataModal, DataModalConfig } from "@/components/data-modal";
import { useIndicators } from "@/lib/hooks/use-indicators";
import { RuntimeStatusBadge } from "@/components/ui/runtime-status";
import Refresh from "@/components/ui/refresh";

export default function IndicatorsPage() {
    const { environment } = useEnvironment();
    const { success, error: toastError } = useToastContext();

    const {
        uiIndicators,
        loading,
        error,
        loadWithRuntime,
        create,
        update,
        remove,
    } = useIndicators(environment);

    const [isModalOpen, setIsModalOpen] = useState(false);
    const [editingData, setEditingData] = useState<Indicator | null>(null);

    const onSave = async (indicator: Indicator) => {
        const result = editingData?.id
            ? await update(indicator)
            : await create(indicator);

        if (!result.ok) {
            toastError("Indicator save error", String(result.error));
            return;
        }

        success("Indicator saved", "Changes saved successfully");
        setIsModalOpen(false);
    };

    const onDelete = async (indicator: Indicator) => {
        const result = await remove(indicator);

        if (!result.ok) {
            toastError("Delete error", String(result.error));
            return;
        }

        success("Indicator deleted", "The indicator was removed successfully");
    };

    React.useEffect(() => {
        loadWithRuntime();
    }, [loadWithRuntime]);

    React.useEffect(() => {
        if (error) {
            toastError("Indicators error", `There was a problem: ${error}`);
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
            key: "is_active",
            label: "Is active",
            render: (is_active) => (
                <p className="font-medium">{is_active ? "Yes" : "No"}</p>
            ),
        },
        {
            key: "symbol",
            label: "Symbol",
            render: (symbol) => <p className="font-medium">{symbol}</p>,
        },
        {
            key: "nick",
            label: "Nick",
            render: (nick) => <p className="font-medium">{nick}</p>,
        },
        {
            key: "direction",
            label: "Direction",
            render: (direction) => <p className="font-medium">{direction}</p>,
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

    const modalConfig: DataModalConfig<Indicator> = {
        title: "indicator",
        description: "Create or edit a trading indicator",
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
                key: "symbol",
                label: "Symbol",
                type: "text",
                required: true,
                placeholder: "Trading symbol",
            },
            {
                key: "nick",
                label: "Nick",
                type: "text",
                required: true,
                placeholder: "Indicator nickname",
            },
            {
                key: "direction",
                label: "Direction",
                type: "text",
                required: true,
                placeholder: "buy / sell / above / below",
            },
            {
                key: "is_percentage",
                label: "Is percentage",
                type: "checkbox",
            },
            {
                key: "is_active",
                label: "Is active",
                type: "checkbox",
            },
            {
                key: "value",
                label: "Value",
                type: "number",
                required: true,
                placeholder: "Indicator value",
            },
        ],
        onSave,
    };

    return (
        <DashboardLayout>
            <div className="flex h-full flex-col">
                <PageHeader
                    title="Indicators"
                    description="Manage your trading indicators"
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
                                New Indicator
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
                                data={uiIndicators}
                                searchPlaceholder="Search indicators..."
                                showEnvironment
                                onEdit={(row) => {
                                    setEditingData(row);
                                    setIsModalOpen(true);
                                }}
                                onDelete={onDelete}
                                deleteConfirmTitle="Delete Indicator?"
                                deleteConfirmDescription="This will permanently delete this indicator. This action cannot be undone."
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
