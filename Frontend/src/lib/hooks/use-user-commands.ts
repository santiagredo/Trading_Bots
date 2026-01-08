import { useCallback, useState } from "react";
import { userCommandsService } from "@/lib/services/user-commands";
import { UserCommand } from "@/types/user-commands";
import { Environment } from "./use-environment";

export function useUserCommands() {
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    // =======================
    // actions
    // =======================

    const execute = useCallback(
        async (env: Environment, command: UserCommand) => {
            setLoading(true);
            setError(null);

            const result = await userCommandsService.post(env, command);

            if (!result.ok) {
                setError(String(result.error));
                setLoading(false);
                return false;
            }

            setLoading(false);
            return true;
        },
        []
    );

    return {
        execute,
        loading,
        error,
    };
}
