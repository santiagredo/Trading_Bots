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
import { Strategy } from "@/interfaces/entities/strategy";
import { format_datetime, number_gt_zero } from "@/lib/utils";
import Loading from "@/components/ui/loading";
import { DataModal, DataModalConfig } from "@/components/data-modal";
import { useStrategies } from "@/lib/hooks/use-strategies";
import { StrategyUI } from "@/interfaces/strategy-ui";
import { RuntimeStatusBadge } from "@/components/ui/runtime-status";
import Refresh from "@/components/ui/refresh";

export default function StrategiesPage() {
    const { environment } = useEnvironment();
    const { success, error: toastError } = useToastContext();

    const {
        uiStrategies,
        loading,
        error,
        loadWithRuntime,
        create,
        update,
        remove,
    } = useStrategies(environment);

    const [isModalOpen, setIsModalOpen] = useState(false);
    const [editingData, setEditingData] = useState<Strategy | null>(null);

    const onSave = async (strategy: Strategy) => {
        const result = editingData?.id
            ? await update(strategy)
            : await create(strategy);

        if (!result.ok) {
            toastError("Strategy save error", String(result.error));
            return;
        }

        success("Strategy saved", "Changes saved successfully");
        setIsModalOpen(false);
    };

    const onDelete = async (strategy: Strategy) => {
        console.log(strategy);
        const result = await remove(strategy);

        if (!result.ok) {
            toastError("Delete error", String(result.error));
            return;
        }

        success("Strategy deleted", "The strategy was removed successfully");
    };

    React.useEffect(() => {
        loadWithRuntime();
    }, [loadWithRuntime]);

    React.useEffect(() => {
        if (error) {
            toastError("Strategies error", `There was a problem: ${error}`);
        }
    }, [error, toastError]);

    const columns: Column<StrategyUI>[] = [
        {
            key: "id",
            label: "ID",
            render: (id) => <p className="font-medium">{id}</p>,
        },
        {
            key: "name",
            label: "Name",
            render: (name) => <p className="font-medium">{name}</p>,
        },
        {
            key: "is_active",
            label: "Is active",
            render: (is_active) => (
                <p className="font-medium">{is_active ? "Yes" : "No"}</p>
            ),
        },
        {
            key: "can_trade",
            label: "Can trade",
            render: (can_trade) => (
                <p className="font-medium">{can_trade ? "Yes" : "No"}</p>
            ),
        },
        {
            key: "description",
            label: "Description",
            render: (description) => (
                <p className="font-medium">{description ?? "-"}</p>
            ),
        },
        {
            key: "last_execution",
            label: "Last execution",
            render: (last_execution) => (
                <p className="font-medium">
                    {last_execution
                        ? format_datetime(last_execution.toString())
                        : "-"}
                </p>
            ),
        },
        {
            key: "cooldown",
            label: "Cooldown",
            render: (cooldown) => (
                <p className="font-medium">{cooldown ?? "-"}</p>
            ),
        },
        {
            key: "error_last_date",
            label: "Last error",
            render: (error_last_date) => (
                <p className="font-medium">
                    {error_last_date
                        ? format_datetime(error_last_date.toString())
                        : "-"}
                </p>
            ),
        },
        {
            key: "error_cooldown",
            label: "Error cooldown",
            render: (error_cooldown) => (
                <p className="font-medium">{error_cooldown ?? "-"}</p>
            ),
        },
        {
            key: "runtimeStatus",
            label: "Runtime",
            render: (_, row) => (
                <RuntimeStatusBadge status={row.runtimeStatus} />
            ),
        },
        {
            key: "last_error_message",
            label: "Last runtime error message",
            render: (error_message) => (
                <p className="font-medium">{error_message ?? "-"}</p>
            ),
        },
        {
            key: "last_error_date",
            label: "Last runtime error date",
            render: (error_date) => (
                <p className="font-medium">
                    {error_date ? format_datetime(error_date as string) : "-"}
                </p>
            ),
        },
    ];

    const modalConfig: DataModalConfig<Strategy> = {
        title: "strategy",
        description: "Create or edit a trading strategy",
        fields: [
            {
                key: "id",
                label: "ID",
                type: "number",
                required: false,
                placeholder: "Strategy ID",
                disabled: true,
            },
            {
                key: "name",
                label: "Name",
                type: "text",
                required: true,
                placeholder: "Strategy name",
            },
            {
                key: "is_active",
                label: "Is active",
                type: "checkbox",
                placeholder: "Is active",
            },
            {
                key: "can_trade",
                label: "Can trade",
                type: "checkbox",
                placeholder: "Can trade",
            },
            {
                key: "description",
                label: "Description",
                type: "text",
                required: false,
                placeholder: "Add a description",
            },
            {
                key: "cooldown",
                label: "Cooldown (secs)",
                type: "number",
                required: true,
                placeholder: "Add a cooldown time",
                validation: (value) => {
                    if (
                        typeof value !== "string" &&
                        typeof value !== "number"
                    ) {
                        return "Please enter a value greater than 0";
                    }

                    if (!number_gt_zero(value)) {
                        return "Please enter a value greater than 0";
                    }

                    return null;
                },
            },
            {
                key: "error_cooldown",
                label: "Error cooldown (secs)",
                type: "number",
                required: true,
                placeholder: "Add an error cooldown time",
                validation: (value) => {
                    if (
                        typeof value !== "string" &&
                        typeof value !== "number"
                    ) {
                        return "Please enter a value greater than 0";
                    }

                    if (!number_gt_zero(value)) {
                        return "Please enter a value greater than 0";
                    }

                    return null;
                },
            },
        ],
        onSave,
    };

    return (
        <DashboardLayout>
            <div className="flex h-full flex-col">
                <PageHeader
                    title="Strategies"
                    description="Manage your trading strategies"
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
                                    setEditingData(null); // null = create mode
                                    setIsModalOpen(true);
                                }}
                            >
                                <Plus className="h-4 w-4" />
                                New Strategy
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
                                data={uiStrategies}
                                searchPlaceholder="Search strategies..."
                                showEnvironment
                                onEdit={(row) => {
                                    setEditingData(row);
                                    setIsModalOpen(true);
                                }}
                                onDelete={onDelete}
                                deleteConfirmTitle="Delete Strategy?"
                                deleteConfirmDescription="This will permanently delete this strategy and all its associated data. This action cannot be undone."
                            />

                            <DataModal
                                open={isModalOpen}
                                onOpenChange={setIsModalOpen}
                                config={modalConfig}
                                data={editingData} // null to create, object to edit
                            />
                        </>
                    )}
                </div>
            </div>
        </DashboardLayout>
    );
}
