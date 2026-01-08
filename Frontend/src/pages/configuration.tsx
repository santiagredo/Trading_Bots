import { useEffect, useState } from "react";
import { Save } from "lucide-react";

import { DashboardLayout } from "@/components/dashboard-layout";
import { PageHeader } from "@/components/page-header";
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Switch } from "@/components/ui/switch";
import { Button } from "@/components/ui/button";
import Loading from "@/components/ui/loading";

import { useToastContext } from "@/components/toast-provider";
import { useEngineConfiguration } from "@/lib/hooks/use-engine-configuration";

export default function ConfigurationPage() {
    const { error: toastError, success } = useToastContext();

    const {
        // configuration
        configuration,
        loadConfiguration,
        saveConfiguration,
        setConfiguration,

        // health / runtime
        // healthCheck,
        loadHealthCheck,
        // lastRefresh,

        engineRunning,
        setEngineRunning,
        saveEngineState,

        // ui
        loading,
        error,
    } = useEngineConfiguration();

    const [showSecrets, setShowSecrets] = useState(false);

    useEffect(() => {
        loadHealthCheck();
        loadConfiguration();
    }, [loadHealthCheck, loadConfiguration]);

    useEffect(() => {
        if (error) toastError("Error", error);
    }, [error]);

    async function handleSave() {
        try {
            const engineResult = await saveEngineState();

            if (!engineResult.ok) {
                toastError("Engine error", String(engineResult.error));
                return;
            }

            if (
                engineRunning &&
                (configuration.api_key !== "" ||
                    configuration.secret_pass !== "")
            ) {
                const configResult = await saveConfiguration(configuration);

                if (!configResult.ok) {
                    toastError(
                        "Configuration error",
                        String(configResult.error)
                    );
                    return;
                }
            }

            success(
                "Changes saved",
                "Engine state and configuration saved successfully"
            );
        } catch (e) {
            toastError("Unexpected error", String(e));
        }
    }

    return (
        <DashboardLayout>
            <div className="flex h-full flex-col">
                <PageHeader
                    title="Configuration"
                    description="Configure your trading bot settings and parameters"
                    actions={
                        <Button
                            size="sm"
                            className="gap-2"
                            onClick={handleSave}
                            disabled={loading}
                        >
                            <Save className="h-4 w-4" />
                            Save Changes
                        </Button>
                    }
                />

                <div className="flex-1 overflow-auto p-6">
                    <div className="grid gap-6 max-w-4xl">
                        {/* =======================
                         * Engine
                         * ======================= */}
                        <Card>
                            <CardHeader>
                                <CardTitle>Engine Settings</CardTitle>
                            </CardHeader>

                            <CardContent className="space-y-6">
                                <div className="flex items-center justify-between">
                                    <div>
                                        <Label>Run engine</Label>
                                        <p className="text-sm text-muted-foreground">
                                            Start / stop engine used to create
                                            crypto transactions
                                        </p>
                                    </div>

                                    {loading ? (
                                        <Loading />
                                    ) : (
                                        <Switch
                                            checked={engineRunning}
                                            onCheckedChange={setEngineRunning}
                                        />
                                    )}
                                </div>
                            </CardContent>
                        </Card>

                        {/* =======================
                         * Binance
                         * ======================= */}
                        <Card>
                            <CardHeader>
                                <CardTitle>Binance</CardTitle>
                                <CardDescription>
                                    Configure your Binance credentials
                                </CardDescription>
                            </CardHeader>

                            <CardContent className="space-y-6">
                                <div className="grid gap-2">
                                    <Label>API Key</Label>
                                    <Input
                                        type={showSecrets ? "text" : "password"}
                                        value={configuration?.api_key ?? ""}
                                        onChange={(e) =>
                                            setConfiguration((prev) => ({
                                                ...prev,
                                                api_key: e.target.value,
                                            }))
                                        }
                                    />
                                </div>

                                <div className="grid gap-2">
                                    <Label>Secret pass</Label>
                                    <Input
                                        type={showSecrets ? "text" : "password"}
                                        value={configuration?.secret_pass ?? ""}
                                        onChange={(e) =>
                                            setConfiguration((prev) => ({
                                                ...prev,
                                                secret_pass: e.target.value,
                                            }))
                                        }
                                    />
                                </div>

                                <div className="flex justify-end">
                                    <Button
                                        type="button"
                                        size="sm"
                                        variant="ghost"
                                        onClick={() =>
                                            setShowSecrets((v) => !v)
                                        }
                                    >
                                        {showSecrets ? "Hide" : "Show"}
                                    </Button>
                                </div>
                            </CardContent>
                        </Card>
                    </div>
                </div>
            </div>
        </DashboardLayout>
    );
}
