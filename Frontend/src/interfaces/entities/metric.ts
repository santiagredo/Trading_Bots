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

interface Duration {
    secs: number;
    nanos: number;
}

export function durationToMs(d: Duration): number {
    return d.secs * 1_000 + d.nanos / 1_000_000;
}

export interface CriticalMetric {
    id: number;

    executions_ok: number;
    executions_err: number;

    total_execution_time: Duration;
    max_execution_time: Duration;

    active_posting: number;
    max_active_posting: number;

    skipped_due_to_lock: number;

    /** ISO datetime string or null */
    last_success: string | null;

    /** ISO datetime string or null */
    last_error: string | null;

    consecutive_errors: number;

    slowest_duration: Duration;
}
