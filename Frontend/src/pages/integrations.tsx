import { useEffect, useMemo, useState } from "react";
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
import { Button } from "@/components/ui/button";
import Loading from "@/components/ui/loading";

import { useToastContext } from "@/components/toast-provider";
import { useEnvironment } from "@/lib/hooks/use-environment";
import { useIntegrationSettings } from "@/lib/hooks/use-integrations";

export default function IntegrationsPage() {
    const { environment } = useEnvironment();
    const { error: toastError, success } = useToastContext();

    const { settings, loadAll, update, loading, error } =
        useIntegrationSettings(environment);

    const [showSecrets, setShowSecrets] = useState(false);

    /* ======================
     * Derived values
     * ====================== */

    const apiKeySetting = useMemo(
        () => settings.find((s) => s.nick === "api_key"),
        [settings],
    );

    const secretSetting = useMemo(
        () => settings.find((s) => s.nick === "secret_pass"),
        [settings],
    );

    /* ======================
     * Effects
     * ====================== */

    useEffect(() => {
        loadAll();
    }, [loadAll]);

    useEffect(() => {
        if (error) toastError("Integration error", error);
    }, [error, toastError]);

    /* ======================
     * Handlers
     * ====================== */

    async function handleSave() {
        try {
            if (apiKeySetting) {
                const result = await update(apiKeySetting);
                if (!result.ok) {
                    toastError("API Key error", String(result.error));
                    return;
                }
            }

            if (secretSetting) {
                const result = await update(secretSetting);
                if (!result.ok) {
                    toastError("Secret error", String(result.error));
                    return;
                }
            }

            success("Saved successfully", "Integration settings updated");
        } catch (e) {
            toastError("Unexpected error", String(e));
        }
    }

    function updateLocalValue(name: string, value: string) {
        const setting = settings.find((s) => s.name === name);
        if (!setting) return;

        setting.value = value; // local mutation (hook update() will persist)
    }

    return (
        <DashboardLayout>
            <div className="flex h-full flex-col">
                <PageHeader
                    title="Integrations"
                    description="Configure your exchange integrations"
                    showEnvironmentSelector
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
                    {loading ? (
                        <Loading />
                    ) : (
                        <div className="grid gap-6 max-w-4xl">
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
                                            type={
                                                showSecrets
                                                    ? "text"
                                                    : "password"
                                            }
                                            value={apiKeySetting?.value ?? ""}
                                            onChange={(e) =>
                                                updateLocalValue(
                                                    "api_key",
                                                    e.target.value,
                                                )
                                            }
                                        />
                                    </div>

                                    <div className="grid gap-2">
                                        <Label>Secret pass</Label>
                                        <Input
                                            type={
                                                showSecrets
                                                    ? "text"
                                                    : "password"
                                            }
                                            value={secretSetting?.value ?? ""}
                                            onChange={(e) =>
                                                updateLocalValue(
                                                    "secret_pass",
                                                    e.target.value,
                                                )
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
                    )}
                </div>
            </div>
        </DashboardLayout>
    );
}
