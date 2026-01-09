"use client";

import { Toast, ToastContainer } from "@/components/ui/toast";
import { useToast } from "./use-toast";

export function Toaster() {
    const { toasts } = useToast();

    return (
        <ToastContainer>
            {toasts.map((toast) => (
                <Toast
                    key={toast.id}
                    id={toast.id}
                    title={toast.title}
                    description={toast.description}
                    variant={toast.variant}
                    onClose={toast.onClose}
                />
            ))}
        </ToastContainer>
    );
}
