import type * as React from "react";
import { X } from "lucide-react";
import { cn } from "@/lib/utils";

export interface ToastProps {
    id: string;
    title?: string;
    description?: string;
    variant?: "default" | "success" | "error" | "warning" | "info";
    duration?: number;
    onClose: () => void;
}

const variantStyles = {
    default: "bg-card border-border",
    success: "bg-emerald-950 border-emerald-800 text-emerald-100",
    error: "bg-red-950 border-red-800 text-red-100",
    warning: "bg-amber-950 border-amber-800 text-amber-100",
    info: "bg-blue-950 border-blue-800 text-blue-100",
};

export function Toast({
    id,
    title,
    description,
    variant = "default",
    onClose,
}: ToastProps) {
    return (
        <div
            className={cn(
                "pointer-events-auto w-full max-w-md rounded-lg border p-4 shadow-lg transition-all",
                "animate-in slide-in-from-right-full",
                variantStyles[variant]
            )}
        >
            <div className="flex items-start gap-3" id={id}>
                <div className="flex-1">
                    {title && (
                        <div className="text-sm font-semibold mb-1">
                            {title}
                        </div>
                    )}
                    {description && (
                        <div className="text-sm opacity-90">{description}</div>
                    )}
                </div>
                <button
                    onClick={onClose}
                    className="rounded-sm opacity-70 hover:opacity-100 transition-opacity"
                >
                    <X className="h-4 w-4" />
                    <span className="sr-only">Close</span>
                </button>
            </div>
        </div>
    );
}

export function ToastContainer({ children }: { children: React.ReactNode }) {
    return (
        <div className="fixed top-0 right-0 z-100 flex max-h-screen w-full flex-col-reverse gap-2 p-4 sm:flex-col md:max-w-105">
            {children}
        </div>
    );
}
