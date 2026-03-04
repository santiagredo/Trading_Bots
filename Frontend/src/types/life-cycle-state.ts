export type LifecycleState = "Off" | "Starting" | "Running" | "Stopping";

export function mapRuntimeState(value: string | undefined): LifecycleState {
    switch (value?.toLowerCase()) {
        case "running":
            return "Running";
        case "starting":
            return "Starting";
        case "stopping":
            return "Stopping";
        case "off":
        default:
            return "Off";
    }
}

export type SocketState =
    | "Disconnected"
    | "Connecting"
    | "Connected"
    | "Reconnecting"
    | "ShuttingDown"
    | "Closed";

export function mapSocketState(value: string | undefined): SocketState {
    switch (value?.toLowerCase()) {
        case "connecting":
            return "Connecting";

        case "connected":
            return "Connected";

        case "reconnecting":
            return "Reconnecting";

        case "shutting down":
            return "ShuttingDown";

        case "closed":
            return "Closed";

        case "disconnected":
        default:
            return "Disconnected";
    }
}
