import { useEffect } from "react";
import { Save } from "lucide-react";
import { DashboardLayout } from "@/components/dashboard-layout";
import { PageHeader } from "@/components/page-header";
import {
    Card,
    CardContent,
    // CardDescription,
    CardHeader,
    CardTitle,
} from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import { Button } from "@/components/ui/button";
import Loading from "@/components/ui/loading";

import { useToastContext } from "@/components/toast-provider";
import { useEngineConfiguration } from "@/lib/hooks/use-engine-configuration";

export default function ConfigurationPage() {
    const { error: toastError, success } = useToastContext();

    const {
        loadHealthCheck,

        engineRunning,
        setEngineRunning,
        saveEngineState,

        // ui
        loading,
        error,
    } = useEngineConfiguration();

    useEffect(() => {
        loadHealthCheck();
    }, [loadHealthCheck]);

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

            success("Changes saved", "Engine state saved successfully");
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
                    </div>
                </div>
            </div>
        </DashboardLayout>
    );
}
