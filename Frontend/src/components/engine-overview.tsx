import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Server } from "lucide-react";
import Loading from "./ui/loading";
import { CacheHealthCheck } from "@/interfaces/health-check";
import { DatabaseStatusCard } from "./ui/database-status-card";

interface EngineOverviewProps {
    healthCheck: CacheHealthCheck | null;
    engineRunning: boolean;
    lastRefresh: string;
    loading: boolean;
}

export function EngineOverview({
    healthCheck,
    engineRunning,
    lastRefresh,
    loading,
}: EngineOverviewProps) {
    return (
        <div className="grid gap-6">
            <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
                {loading ? (
                    <Loading />
                ) : (
                    <EngineStatusCard
                        engineRunning={engineRunning}
                        healthCheck={healthCheck}
                        lastRefresh={lastRefresh}
                    />
                )}

                {loading ? (
                    <Loading />
                ) : (
                    <DatabaseStatusCard healthCheck={healthCheck} />
                )}
            </div>
        </div>
    );
}

function EngineStatusCard({
    engineRunning,
    healthCheck,
    lastRefresh,
}: {
    engineRunning: boolean;
    healthCheck: CacheHealthCheck | null;
    lastRefresh: string;
}) {
    return (
        <Card>
            <CardHeader className="flex flex-row items-center justify-between pb-2">
                <CardTitle className="text-sm font-medium">
                    Engine Status
                </CardTitle>
                <Server className="h-4 w-4 text-muted-foreground" />
            </CardHeader>

            <CardContent>
                <Badge
                    className={
                        engineRunning
                            ? "bg-success text-background"
                            : "bg-red-500 text-background"
                    }
                >
                    {engineRunning ? "Running" : "Not Running"}
                </Badge>

                <p className="mt-2 text-xs text-muted-foreground">
                    Startup Date:{" "}
                    {engineRunning ? healthCheck?.engine_startup_date : ""}
                </p>

                {/* {engineRunning && (
                    <p className="mt-2 text-xs text-muted-foreground">
                        Startup date: {healthCheck?.startup_date.split(".")[0]}
                    </p>
                )} */}

                <p className="mt-2 text-xs text-muted-foreground">
                    Last refresh: {lastRefresh}
                </p>
            </CardContent>
        </Card>
    );
}
