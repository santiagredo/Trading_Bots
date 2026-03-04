import { Badge } from "./badge";

export function DbRow({ label, ok }: { label: string; ok: boolean }) {
    return (
        <div className="flex justify-between">
            <span className="text-sm text-muted-foreground">{label}</span>
            <Badge
                className={
                    ok
                        ? "bg-success text-background"
                        : "bg-red-500 text-background"
                }
            >
                {ok ? "Running" : "Not Running"}
            </Badge>
        </div>
    );
}
