import { RuntimeStatus } from "@/types/runtime-status";
import { Badge } from "./badge";

export function RuntimeStatusBadge({ status }: { status: RuntimeStatus }) {
    switch (status) {
        case "loaded":
            return <Badge variant="default">Loaded</Badge>;
        case "outdated":
            return <Badge variant="destructive">Outdated</Badge>;
        case "not_loaded":
            return <Badge variant="secondary">Not loaded</Badge>;
    }
}
