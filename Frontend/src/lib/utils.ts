import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}

export function format_datetime(date: Date | string | number): string {
    return new Intl.DateTimeFormat("sv-SE", {
        year: "numeric",
        month: "2-digit",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
    }).format(new Date(date));
}

export function format_two_decimals(value: string | number): string {
    return Number(value).toFixed(2);
}

export function change_color(value: string): string {
    const v = value.trim();
    if (v === "-2" || v === "0.00") return "text-gray-400";
    if (v.startsWith("-")) return "text-red-500";
    return "text-green-500";
}

export function number_gt_zero(value: string | number): boolean {
    const n = typeof value === "number" ? value : Number(value);
    return Number.isFinite(n) && n > 0;
}
