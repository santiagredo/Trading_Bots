import { CacheHealthCheck } from "@/interfaces/health-check";
import { Card, CardContent, CardHeader, CardTitle } from "./card";
import { Database } from "lucide-react";
import { DbRow } from "./db-row";

export function DatabaseStatusCard({
    healthCheck,
}: {
    healthCheck: CacheHealthCheck | null;
}) {
    return (
        <Card className="flex flex-col justify-between">
            <CardHeader className="flex flex-row items-center justify-between pb-2">
                <CardTitle className="text-sm font-medium">
                    Database Status
                </CardTitle>
                <Database className="h-4 w-4 text-muted-foreground" />
            </CardHeader>

            <CardContent className="grid gap-2">
                <DbRow
                    label="Dev database"
                    ok={healthCheck?.db_dev_conn_is_valid === "true"}
                />
                <DbRow
                    label="Prod database"
                    ok={healthCheck?.db_prod_conn_is_valid === "true"}
                />
            </CardContent>
        </Card>
    );
}
