import { Order } from "@/interfaces/entities/order";
import { Result } from "@/types/result";
import { invoke } from "@tauri-apps/api/core";

// db
async function insert_order(env: string, order: Order): Promise<Result<Order>> {
    try {
        order.id = 0;

        const data = await invoke<Order>("insert_order", {
            env,
            order,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function select_order(
    env: string,
    query?: Order
): Promise<Result<Order>> {
    try {
        const data = await invoke<Order>("select_order", {
            env,
            query,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function select_orders(
    env: string,
    query?: Order
): Promise<Result<Order[]>> {
    try {
        const data = await invoke<Order[]>("select_orders", {
            env,
            query,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

async function update_order(env: string, order: Order): Promise<Result<Order>> {
    try {
        const data = await invoke<Order>("update_order", {
            env,
            order,
        });

        return { ok: true, data };
    } catch (error) {
        return { ok: false, error };
    }
}

// service
export const orderService = {
    // db
    insert: (env: string, order: Order): Promise<Result<Order>> =>
        insert_order(env, order),

    select: (env: string, query?: Order): Promise<Result<Order>> =>
        select_order(env, query),

    selectAll: (env: string, query?: Order): Promise<Result<Order[]>> =>
        select_orders(env, query),

    update: (env: string, order: Order): Promise<Result<Order>> =>
        update_order(env, order),
};
