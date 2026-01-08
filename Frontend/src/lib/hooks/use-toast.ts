import { useState, useCallback } from "react";

export interface Toast {
    id: string;
    title?: string;
    description?: string;
    variant?: "default" | "success" | "error" | "warning" | "info";
    duration?: number;
}

export interface ToastOptions {
    title?: string;
    description?: string;
    variant?: "default" | "success" | "error" | "warning" | "info";
    duration?: number;
}

let toastCount = 0;

export function useToast() {
    const [toasts, setToasts] = useState<Toast[]>([]);

    const addToast = useCallback((options: ToastOptions) => {
        const id = `toast-${++toastCount}`;
        const duration = options.duration ?? 5000;

        const newToast: Toast = {
            id,
            ...options,
        };

        setToasts((prev) => [...prev, newToast]);

        if (duration > 0) {
            setTimeout(() => {
                removeToast(id);
            }, duration);
        }

        return id;
    }, []);

    const removeToast = useCallback((id: string) => {
        setToasts((prev) => prev.filter((toast) => toast.id !== id));
    }, []);

    const toast = useCallback(
        (options: ToastOptions) => {
            return addToast(options);
        },
        [addToast]
    );

    const success = useCallback(
        (title: string, description?: string) => {
            return addToast({ title, description, variant: "success" });
        },
        [addToast]
    );

    const error = useCallback(
        (title: string, description?: string) => {
            return addToast({ title, description, variant: "error" });
        },
        [addToast]
    );

    const warning = useCallback(
        (title: string, description?: string) => {
            return addToast({ title, description, variant: "warning" });
        },
        [addToast]
    );

    const info = useCallback(
        (title: string, description?: string) => {
            return addToast({ title, description, variant: "info" });
        },
        [addToast]
    );

    return {
        toasts,
        toast,
        success,
        error,
        warning,
        info,
        dismiss: removeToast,
    };
}
