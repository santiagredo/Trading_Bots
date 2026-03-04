"use client";

import { useEffect } from "react";
import { DashboardLayout } from "@/components/dashboard-layout";
import { PageHeader } from "@/components/page-header";
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from "@/components/ui/card";
import { StatCard } from "@/components/stat-card";
import Loading from "@/components/ui/loading";
import { Activity, Clock, Zap } from "lucide-react";
import { useBinance } from "@/lib/hooks/use-binance";
import Refresh from "@/components/ui/refresh";
import { useEnvironment } from "@/lib/hooks/use-environment";

export default function BinancePage() {
    const { environment } = useEnvironment();
    const { accountInformation, loading, error, load } =
        useBinance(environment);

    useEffect(() => {
        load();
    }, [load]);

    return (
        <DashboardLayout>
            <div className="flex h-full flex-col">
                <PageHeader
                    title="Binance Integration"
                    description="Monitor Binance API connection and activity"
                    actions={
                        <Refresh onRefresh={load} isRefreshing={loading} />
                    }
                    showEnvironmentSelector
                />

                <div className="flex-1 overflow-auto p-6">
                    {loading ? (
                        <Loading />
                    ) : error ? (
                        <Card>
                            <CardContent className="p-6 text-red-500">
                                Error loading Binance account: {error}
                            </CardContent>
                        </Card>
                    ) : accountInformation ? (
                        <div className="grid gap-6">
                            <div className="grid gap-4 md:grid-cols-3">
                                <StatCard
                                    title="Trading Enabled"
                                    value={
                                        accountInformation.canTrade
                                            ? "Yes"
                                            : "No"
                                    }
                                    description="Account trading permission"
                                    icon={Activity}
                                />
                                <StatCard
                                    title="Withdraw Enabled"
                                    value={
                                        accountInformation.canWithdraw
                                            ? "Yes"
                                            : "No"
                                    }
                                    description="Withdraw permission"
                                    icon={Zap}
                                />
                                <StatCard
                                    title="Deposit Enabled"
                                    value={
                                        accountInformation.canDeposit
                                            ? "Yes"
                                            : "No"
                                    }
                                    description="Deposit permission"
                                    icon={Clock}
                                />
                            </div>

                            <Card>
                                <CardHeader>
                                    <CardTitle>Account Details</CardTitle>
                                    <CardDescription>
                                        Binance account information
                                    </CardDescription>
                                </CardHeader>
                                <CardContent className="space-y-4">
                                    <Row
                                        label="UID"
                                        value={accountInformation.uid}
                                    />
                                    <Row
                                        label="Account Type"
                                        value={accountInformation.accountType}
                                    />
                                    <Row
                                        label="Brokered"
                                        value={
                                            accountInformation.brokered
                                                ? "Yes"
                                                : "No"
                                        }
                                    />
                                    <Row
                                        label="Self Trade Prevention"
                                        value={
                                            accountInformation.requireSelfTradePrevention
                                                ? "Enabled"
                                                : "Disabled"
                                        }
                                    />
                                </CardContent>
                            </Card>

                            <Card>
                                <CardHeader>
                                    <CardTitle>Commission Rates</CardTitle>
                                    <CardDescription>
                                        Trading fee configuration
                                    </CardDescription>
                                </CardHeader>
                                <CardContent className="space-y-4">
                                    <Row
                                        label="Maker"
                                        value={accountInformation.commissionRates.maker.toString()}
                                    />
                                    <Row
                                        label="Taker"
                                        value={accountInformation.commissionRates.taker.toString()}
                                    />
                                    <Row
                                        label="Buyer"
                                        value={accountInformation.commissionRates.buyer.toString()}
                                    />
                                    <Row
                                        label="Seller"
                                        value={accountInformation.commissionRates.seller.toString()}
                                    />
                                </CardContent>
                            </Card>

                            <Card>
                                <CardHeader>
                                    <CardTitle>Balances</CardTitle>
                                    <CardDescription>
                                        Available and locked funds
                                    </CardDescription>
                                </CardHeader>
                                <CardContent className="space-y-2">
                                    {accountInformation.balances
                                        .filter(
                                            (b) =>
                                                Number(b.free) > 0 ||
                                                Number(b.locked) > 0,
                                        )
                                        .map((balance) => (
                                            <div
                                                key={balance.asset}
                                                className="flex items-center justify-between text-sm"
                                            >
                                                <span className="font-medium">
                                                    {balance.asset}
                                                </span>
                                                <span className="font-mono">
                                                    Free:{" "}
                                                    {balance.free.toString()} |
                                                    Locked:{" "}
                                                    {balance.locked.toString()}
                                                </span>
                                            </div>
                                        ))}
                                </CardContent>
                            </Card>

                            <Card>
                                <CardHeader>
                                    <CardTitle>Permissions</CardTitle>
                                    <CardDescription>
                                        Available permissions
                                    </CardDescription>
                                </CardHeader>
                                <CardContent className="space-y-2">
                                    {accountInformation.permissions.map(
                                        (permission) => (
                                            <div
                                                key={permission}
                                                className="flex items-center justify-between text-sm"
                                            >
                                                <span className="font-medium">
                                                    {permission}
                                                </span>
                                            </div>
                                        ),
                                    )}
                                </CardContent>
                            </Card>
                        </div>
                    ) : null}
                </div>
            </div>
        </DashboardLayout>
    );
}

//  Helper Row
function Row({ label, value }: { label: string; value: string | number }) {
    return (
        <div className="flex items-center justify-between">
            <span className="text-sm font-medium">{label}</span>
            <span className="font-mono text-sm">{value}</span>
        </div>
    );
}
