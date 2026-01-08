export interface Ledger {
    id: number;
    order_id: number | null;
    record_type_id: number;
    creation_date: string;
    asset_id: number;

    free_amount: string;
    free_previous_balance: string;
    free_new_balance: string;

    locked_amount: string;
    locked_previous_balance: string;
    locked_new_balance: string;
}
