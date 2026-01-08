import { RefreshCw } from "lucide-react";
import { Button } from "./button";

interface RefreshProps {
    onRefresh: () => void | Promise<void>;
    isRefreshing?: boolean;
}

export default function Refresh({
    onRefresh,
    isRefreshing = false,
}: RefreshProps) {
    return (
        <Button
            variant="outline"
            size="sm"
            onClick={onRefresh}
            disabled={isRefreshing}
        >
            <RefreshCw
                className={`h-4 w-4 ${isRefreshing ? "animate-spin" : ""}`}
            />
        </Button>
    );
}
