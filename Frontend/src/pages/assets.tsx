"use client";

import { useState } from "react";
import { DashboardLayout } from "@/components/dashboard-layout";
import { PageHeader } from "@/components/page-header";
import { Column, DataTable } from "@/components/data-table";
import { StatCard } from "@/components/stat-card";
import { Wallet, TrendingUp, DollarSign } from "lucide-react";
import type { Environment } from "@/lib/hooks/use-environment";
import { Asset } from "@/interfaces/entities/asset";
import React from "react";
import { select_assets } from "@/lib/services/assets";
import { useToastContext } from "@/components/toast-provider";
import { useEnvironment } from "@/lib/hooks/use-environment";
import Loading from "@/components/ui/loading";

export default function AssetsPage() {
    const [loading, setLoading] = React.useState(true);
    const { success, error } = useToastContext();
    const { environment } = useEnvironment();

    const [assets, setAssets] = useState<Asset[]>([]);

    const handleRefresh = async (env: Environment) => {
        setLoading(true);
        await get_assets(env);
        setLoading(false);
    };

    async function get_assets(env: Environment) {
        let assets_result = await select_assets(env);

        if (!assets_result.ok) {
            error(
                "Assets fetch error",
                `There was a problem fetching assets: ${assets_result.error}`
            );

            return;
        }

        setAssets(assets_result.data);
    }

    React.useEffect(() => {
        const run = async (env: Environment) => {
            await get_assets(env);
            setLoading(false);
        };

        run(environment);
    }, [environment]);

    const columns: Column<Asset>[] = [
        {
            key: "name",
            label: "Name",
            render: (name) => <p className="font-medium">{name}</p>,
        },
        {
            key: "ticker",
            label: "Ticker",
            render: (ticker) => <p className="font-medium">{ticker}</p>,
        },
        {
            key: "free",
            label: "Free amount",
            render: (free) => <p className="font-medium">{free}</p>,
        },
        {
            key: "locked",
            label: "Locked amount",
            render: (locked) => <p className="font-medium">{locked}</p>,
        },
    ];

    return (
        <DashboardLayout>
            <div className="flex h-full flex-col">
                <PageHeader
                    title="Assets"
                    description="View and manage your crypto asset portfolio"
                />
                <div className="flex-1 overflow-auto p-6">
                    <div className="grid gap-6">
                        {loading ? (
                            <Loading />
                        ) : (
                            <DataTable
                                columns={columns}
                                data={assets}
                                onRefresh={handleRefresh}
                                searchPlaceholder="Search assets..."
                                showEnvironment
                            />
                        )}
                    </div>
                </div>
            </div>
        </DashboardLayout>
    );
}
