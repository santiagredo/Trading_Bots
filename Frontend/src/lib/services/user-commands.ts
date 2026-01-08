import { Result } from "@/types/result";
import { UserCommand } from "@/types/user-commands";
import { invoke } from "@tauri-apps/api/core";
import { Environment } from "../hooks/use-environment";

async function post_user_command(
    env: Environment,
    command: UserCommand
): Promise<Result<null>> {
    try {
        const res = await invoke<null>("post_user_command", { env, command });

        return { ok: true, data: res };
    } catch (error) {
        return { ok: false, error };
    }
}

export const userCommandsService = {
    post: (env: Environment, command: UserCommand): Promise<Result<null>> =>
        post_user_command(env, command),
};
