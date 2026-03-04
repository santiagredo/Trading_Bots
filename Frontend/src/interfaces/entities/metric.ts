export interface Metric {
    id: number;
    creation_date: string;

    executions_ok: number;
    executions_err: number;
    total_execution_time: number;
    max_execution_time: number;
    slowest_duration: number;

    active_posting: number;
    max_active_posting: number;
    skipped_due_to_lock: number;

    last_success: string | null;
    last_error: string | null;

    consecutive_errors: number;
}
