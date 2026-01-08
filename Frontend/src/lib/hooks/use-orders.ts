import { useCallback, useState } from "react";
import { Order } from "@/interfaces/entities/order";
import { orderService } from "@/lib/services/orders";

export function useOrders(env: string) {
    const [orders, setOrders] = useState<Order[]>([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    // =======================
    // loaders
    // =======================

    const loadAll = useCallback(
        async (_query?: Order) => {
            setLoading(true);
            setError(null);

            const result = await orderService.selectAll(env);

            if (result.ok) {
                setOrders(result.data);
            } else {
                setError(String(result.error));
            }

            setLoading(false);
        },
        [env]
    );

    const loadOne = useCallback(
        async (query?: Order) => {
            setLoading(true);
            setError(null);

            const result = await orderService.select(env, query);

            setLoading(false);
            return result;
        },
        [env]
    );

    // =======================
    // mutations
    // =======================

    const create = useCallback(
        async (order: Order) => {
            setLoading(true);
            setError(null);

            const result = await orderService.insert(env, order);

            if (result.ok) {
                setOrders((prev) => [...prev, result.data]);
            } else {
                setError(String(result.error));
            }

            setLoading(false);
            return result;
        },
        [env]
    );

    const update = useCallback(
        async (order: Order) => {
            setLoading(true);
            setError(null);

            const result = await orderService.update(env, order);

            if (result.ok) {
                setOrders((prev) =>
                    prev.map((o) => (o.id === result.data.id ? result.data : o))
                );
            } else {
                setError(String(result.error));
            }

            setLoading(false);
            return result;
        },
        [env]
    );

    return {
        orders,
        loading,
        error,

        loadAll,
        loadOne,

        create,
        update,
    };
}
