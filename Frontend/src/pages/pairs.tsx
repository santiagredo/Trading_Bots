"use client";

import { useState } from "react";
import { DashboardLayout } from "@/components/dashboard-layout";
import { PageHeader } from "@/components/page-header";
import { Column, DataTable } from "@/components/data-table";
import { useEnvironment, type Environment } from "@/lib/hooks/use-environment";
import { useToastContext } from "@/components/toast-provider";
import React from "react";
import { select_pairs } from "@/lib/services/pairs";
import { Pair } from "@/interfaces/entities/pair";
import Loading from "@/components/ui/loading";
import {
    change_color,
    format_datetime,
    format_two_decimals,
} from "@/lib/utils";

export default function PairsPage() {
    const [loading, setLoading] = React.useState(true);
    const { error } = useToastContext();
    const { environment } = useEnvironment();

    const [pairs, setPairs] = useState<Pair[]>([]);

    const handleRefresh = async (env: Environment) => {
        setLoading(true);
        await get_pairs(env);
        setLoading(false);
    };

    async function get_pairs(env: Environment) {
        let pairs_result = await select_pairs(env);

        if (!pairs_result.ok) {
            error(
                "Pairs fetch error",
                `There was a problem fetching pairs: ${pairs_result.error}`
            );

            return;
        }

        setPairs(pairs_result.data);
    }

    React.useEffect(() => {
        const run = async (env: Environment) => {
            await get_pairs(env);
            setLoading(false);
        };

        run(environment);
    }, [environment]);

    const columns: Column<Pair>[] = [
        {
            key: "symbol",
            label: "Pair",
            render: (symbol) => <p className="font-medium">{symbol}</p>,
        },
        {
            key: "update_date",
            label: "Last update",
            render: (update_date) => (
                <p className="font-medium">
                    {format_datetime(update_date.toString())}
                </p>
            ),
        },
        {
            key: "all_time_high_price",
            label: "ATH price",
            render: (value) => (
                <p className="font-medium">
                    {format_two_decimals(value.toString())}
                </p>
            ),
        },

        /* ====== PERCENTAGES ====== */

        {
            key: "percent_from_all_time_high",
            label: "From ATH",
            render: (value) => {
                const formatted = format_two_decimals(value.toString());

                return (
                    <p className={`font-medium ${change_color(formatted)}`}>
                        {formatted}
                    </p>
                );
            },
        },
        {
            key: "fifteen_minutes_price_percent_change",
            label: "15m",
            render: (value) => {
                const formatted = format_two_decimals(value.toString());

                return (
                    <p className={`font-medium ${change_color(formatted)}`}>
                        {formatted}
                    </p>
                );
            },
        },
        {
            key: "hour_price_percent_change",
            label: "1h",
            render: (value) => {
                const formatted = format_two_decimals(value.toString());

                return (
                    <p className={`font-medium ${change_color(formatted)}`}>
                        {formatted}
                    </p>
                );
            },
        },
        {
            key: "six_hours_price_percent_change",
            label: "6h",
            render: (value) => {
                const formatted = format_two_decimals(value.toString());

                return (
                    <p className={`font-medium ${change_color(formatted)}`}>
                        {formatted}
                    </p>
                );
            },
        },
        {
            key: "day_price_percent_change",
            label: "24h",
            render: (value) => {
                const formatted = format_two_decimals(value.toString());

                return (
                    <p className={`font-medium ${change_color(formatted)}`}>
                        {formatted}
                    </p>
                );
            },
        },
        {
            key: "week_price_percent_change",
            label: "7d",
            render: (value) => {
                const formatted = format_two_decimals(value.toString());

                return (
                    <p className={`font-medium ${change_color(formatted)}`}>
                        {formatted}
                    </p>
                );
            },
        },
        {
            key: "month_price_percent_change",
            label: "30d",
            render: (value) => {
                const formatted = format_two_decimals(value.toString());

                return (
                    <p className={`font-medium ${change_color(formatted)}`}>
                        {formatted}
                    </p>
                );
            },
        },
        {
            key: "year_price_percent_change",
            label: "1y",
            render: (value) => {
                const formatted = format_two_decimals(value.toString());

                return (
                    <p className={`font-medium ${change_color(formatted)}`}>
                        {formatted}
                    </p>
                );
            },
        },

        /* ====== NON-PERCENT ====== */

        {
            key: "notional_min_notional",
            label: "Min notional",
            render: (value) => (
                <p className="font-medium">
                    {format_two_decimals(value.toString())}
                </p>
            ),
        },
    ];

    return (
        <DashboardLayout>
            <div className="flex h-full flex-col">
                <PageHeader
                    title="Trading Pairs"
                    description="Monitor trading pairs and market data"
                    showEnvironmentSelector
                />
                <div className="flex-1 overflow-auto p-6">
                    {loading ? (
                        <Loading />
                    ) : (
                        <DataTable
                            columns={columns}
                            data={pairs}
                            onRefresh={handleRefresh}
                            searchPlaceholder="Search pairs..."
                            showEnvironment
                        />
                    )}
                </div>
            </div>
        </DashboardLayout>
    );
}
