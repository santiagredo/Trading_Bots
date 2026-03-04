import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Play, Square, RotateCcw } from "lucide-react";
import { UserCommand } from "@/types/user-commands";
import { Environment } from "@/lib/hooks/use-environment";
import { CacheHealthCheck } from "@/interfaces/health-check";
import { LifecycleState, mapRuntimeState } from "@/types/life-cycle-state";

interface EngineControlPanelProps {
    engineRunning?: boolean;
    onCommand: (env: Environment, command: UserCommand) => Promise<void>;
    loading?: boolean;
    env: Environment;
    healthCheck: CacheHealthCheck | null;
}

export function EngineControlPanel({
    onCommand,
    env,
    healthCheck,
}: EngineControlPanelProps) {
    const rawState =
        env === "dev"
            ? healthCheck?.runtime_dev_status
            : healthCheck?.runtime_prod_status;

    const healthState = mapRuntimeState(rawState);

    const envDesc = env === "dev" ? "development" : "production";

    const canStart =
        env === "dev"
            ? healthCheck?.runtime_dev_status === "off"
            : healthCheck?.runtime_prod_status === "off";

    const canStop =
        env === "dev"
            ? healthCheck?.runtime_dev_status === "running"
            : healthCheck?.runtime_prod_status === "running";

    const canRestart =
        env === "dev"
            ? healthCheck?.runtime_dev_status === "running"
            : healthCheck?.runtime_prod_status === "running";
    return (
        <Card>
            <CardHeader className="flex flex-row items-center justify-between">
                <CardTitle className="text-sm font-medium">
                    {`Runtime ${envDesc} controls`}
                </CardTitle>

                <RuntimeHealthBadge state={healthState} />
            </CardHeader>

            <CardContent className="flex flex-col gap-3">
                <Button
                    onClick={() => onCommand(env, "start")}
                    disabled={!canStart}
                    className="flex gap-2"
                >
                    <Play className="h-4 w-4" />
                    Start
                </Button>

                <Button
                    variant="destructive"
                    onClick={() => onCommand(env, "stop")}
                    disabled={!canStop}
                    className="flex gap-2"
                >
                    <Square className="h-4 w-4" />
                    Stop
                </Button>

                <Button
                    variant="outline"
                    onClick={() => onCommand(env, "restart")}
                    disabled={!canRestart}
                    className="flex gap-2"
                >
                    <RotateCcw className="h-4 w-4" />
                    Restart
                </Button>
            </CardContent>
        </Card>
    );
}

function RuntimeHealthBadge({ state }: { state: LifecycleState }) {
    const map = {
        Running: "bg-success text-black",
        Stopping: "bg-yellow-500 text-black",
        Starting: "bg-yellow-500 text-black",
        Off: "bg-red-500 text-white",
    };

    return (
        <div
            className={`text-xs px-2 py-1 rounded font-semibold ${map[state]}`}
        >
            {state}
        </div>
    );
}
