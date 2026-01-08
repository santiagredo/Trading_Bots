import type React from "react";

import { createContext, useContext } from "react";
import { useToast, type ToastOptions } from "@/lib/hooks/use-toast";
import { Toast, ToastContainer } from "@/components/ui/toast";

interface ToastContextType {
    toast: (options: ToastOptions) => string;
    success: (title: string, description?: string) => string;
    error: (title: string, description?: string) => string;
    warning: (title: string, description?: string) => string;
    info: (title: string, description?: string) => string;
    dismiss: (id: string) => void;
}

const ToastContext = createContext<ToastContextType | undefined>(undefined);

export function ToastProvider({ children }: { children: React.ReactNode }) {
    const { toasts, toast, success, error, warning, info, dismiss } =
        useToast();

    return (
        <ToastContext.Provider
            value={{ toast, success, error, warning, info, dismiss }}
        >
            {children}
            <ToastContainer>
                {toasts.map((toast) => (
                    <Toast
                        key={toast.id}
                        {...toast}
                        onClose={() => dismiss(toast.id)}
                    />
                ))}
            </ToastContainer>
        </ToastContext.Provider>
    );
}

export function useToastContext() {
    const context = useContext(ToastContext);
    if (!context) {
        throw new Error("useToastContext must be used within a ToastProvider");
    }
    return context;
}
