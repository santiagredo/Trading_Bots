export interface Task {
    id: number;
    nick: string;
    description: string;
    is_active: boolean;
    cooldown: number;
    delay: number;
}
