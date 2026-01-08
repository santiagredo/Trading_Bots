"use client";

import { useState } from "react";
import { DashboardLayout } from "@/components/dashboard-layout";
import { PageHeader } from "@/components/page-header";
import { Column, DataTable } from "@/components/data-table";
import { Button } from "@/components/ui/button";
import { useEnvironment } from "@/lib/hooks/use-environment";
import React from "react";
import { useToastContext } from "@/components/toast-provider";
import Loading from "@/components/ui/loading";
import { DataModal, DataModalConfig } from "@/components/data-modal";
import { useTasks } from "@/lib/hooks/use-tasks";
import { RuntimeStatusBadge } from "@/components/ui/runtime-status";
import Refresh from "@/components/ui/refresh";
import { Task } from "@/interfaces/entities/task";

export default function TasksPage() {
    const { environment } = useEnvironment();
    const { success, error: toastError } = useToastContext();

    const {
        uiTasks,
        loading,
        error,
        loadWithRuntime,
        update,
        startActive,
        stopActive,
    } = useTasks(environment);

    const [isModalOpen, setIsModalOpen] = useState(false);
    const [editingData, setEditingData] = useState<Task | null>(null);

    const onSave = async (data: Partial<Task>) => {
        if (!editingData) {
            toastError(
                "Task save error",
                "Editing data is required for update"
            );
            return;
        }

        const taskToUpdate: Task = {
            ...editingData,
            nick: data.nick ?? editingData.nick,
            description: data.description ?? editingData.description,
            is_active: data.is_active ?? editingData.is_active,
            cooldown: data.cooldown ?? editingData.cooldown,
            delay: data.delay ?? editingData.delay,
        };

        const result = await update(taskToUpdate);

        if (!result.ok) {
            toastError("Task save error", String(result.error));
            return;
        }

        success("Task updated", "Changes saved successfully");
        setIsModalOpen(false);
    };

    React.useEffect(() => {
        loadWithRuntime();
    }, [loadWithRuntime]);

    React.useEffect(() => {
        if (error) {
            toastError("Tasks error", `There was a problem: ${error}`);
        }
    }, [error, toastError]);

    const columns: Column<any>[] = [
        {
            key: "nick",
            label: "Task",
            render: (value) => (
                <div>
                    <p className="font-medium">{value}</p>
                </div>
            ),
        },
        {
            key: "description",
            label: "Description",
            render: (value) => <p>{value}</p>,
        },
        {
            key: "cooldown",
            label: "Cooldown",
            render: (value) => <p>{value}</p>,
        },
        {
            key: "delay",
            label: "Delay",
            render: (value) => <p>{value}</p>,
        },
        {
            key: "is_active",
            label: "Active",
            render: (value) => <p>{value ? "Yes" : "No"}</p>,
        },
        {
            key: "runtimeStatus",
            label: "Runtime",
            render: (_, row) => (
                <RuntimeStatusBadge status={row.runtimeStatus} />
            ),
        },
    ];

    const modalConfig: DataModalConfig<Task> = {
        title: "task",
        description: "Create or edit a scheduled task",
        fields: [
            {
                key: "id",
                label: "ID",
                type: "number",
                disabled: true,
            },
            {
                key: "nick",
                label: "Nick",
                type: "text",
                required: false,
                placeholder: "Task nickname",
                disabled: true,
            },
            {
                key: "description",
                label: "Description",
                type: "text",
                required: false,
                placeholder: "Task description",
                disabled: true,
            },
            {
                key: "is_active",
                label: "Active",
                type: "checkbox",
            },
            {
                key: "cooldown",
                label: "Cooldown (s)",
                type: "number",
                required: true,
            },
            {
                key: "delay",
                label: "Delay (s)",
                type: "number",
                required: false,
            },
        ],
        onSave,
    };

    return (
        <DashboardLayout>
            <div className="flex h-full flex-col">
                <PageHeader
                    title="Tasks"
                    description="Manage scheduled and manual tasks"
                    showEnvironmentSelector
                    actions={
                        <>
                            <Refresh
                                onRefresh={loadWithRuntime}
                                isRefreshing={loading}
                            />
                            <Button
                                size="sm"
                                onClick={startActive}
                                disabled={loading}
                                className="ml-2"
                            >
                                Start Active
                            </Button>
                            <Button
                                size="sm"
                                onClick={stopActive}
                                disabled={loading}
                                className="ml-2"
                            >
                                Stop Active
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
                                data={uiTasks}
                                searchPlaceholder="Search tasks..."
                                showEnvironment
                                onEdit={(row) => {
                                    setEditingData(row);
                                    setIsModalOpen(true);
                                }}
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
