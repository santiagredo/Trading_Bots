import { OrderDirection } from "@/types/order-direction";

export interface QueryOptions {
    limit?: number;
    offset?: number;
    order_by?: string;
    order_direction?: OrderDirection;
}
