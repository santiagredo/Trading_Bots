import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Play, Square, RotateCcw } from "lucide-react";
import { UserCommand } from "@/types/user-commands";
import { Environment } from "@/lib/hooks/use-environment";
import { CacheHealthCheck } from "@/interfaces/health-check";
import { RuntimeHealthState } from "@/types/runtime-health-state";

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
    const flags = getEnvHealthFlags(healthCheck, env);
    const healthState = resolveRuntimeHealthState(flags);
    const envDesc = env === "dev" ? "development" : "production";

    const canStart = healthState === "down";
    const canStop = healthState === "healthy" || healthState === "degraded";
    const canRestart = healthState === "healthy" || healthState === "degraded";

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

function getEnvHealthFlags(
    health: CacheHealthCheck | null,
    env: Environment
): string[] {
    if (!health) return [];

    const suffix = env === "prod" ? "_prod_is_init" : "_dev_is_init";

    return Object.entries(health)
        .filter(([key]) => key.includes(suffix))
        .map(([, value]) => value);
}

function resolveRuntimeHealthState(flags: string[]): RuntimeHealthState {
    if (flags.length < 1) return "down";

    const allOk = flags.every((v) => v === "true");
    const allFalse = flags.every((v) => v === "false");

    if (allOk) return "healthy";
    if (allFalse) return "down";
    return "degraded";
}

function RuntimeHealthBadge({ state }: { state: RuntimeHealthState }) {
    const map = {
        healthy: "bg-success text-black",
        degraded: "bg-yellow-500 text-black",
        down: "bg-red-500 text-white",
    };

    const label = {
        healthy: "RUNTIME HEALTHY",
        degraded: "RUNTIME DEGRADED",
        down: "RUNTIME DOWN",
    };

    return (
        <div
            className={`text-xs px-2 py-1 rounded font-semibold ${map[state]}`}
        >
            {label[state]}
        </div>
    );
}
