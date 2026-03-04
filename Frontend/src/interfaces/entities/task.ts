import { LifecycleState } from "@/types/life-cycle-state";
import { TaskState } from "../enums/task-state";

export interface Task {
    id: number;
    nick: string;
    description: string;
    is_active: boolean;
    cooldown: number;
    delay: number;
}

export interface CacheTask {
    model: Task;
    state: TaskState;
}

export interface CacheTasks {
    models: Record<number, CacheTask>;
    startup_date: string;
    last_update_date: string;
    status: LifecycleState;
}
