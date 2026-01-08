import type { ReactNode } from "react";
import { EnvironmentSelector } from "@/components/environment-selector";

interface PageHeaderProps {
    title: string;
    description?: string;
    actions?: ReactNode;
    showEnvironmentSelector?: boolean;
}

export function PageHeader({
    title,
    description,
    actions,
    showEnvironmentSelector = false,
}: PageHeaderProps) {
    return (
        <div className="flex items-center justify-between border-b border-border p-6">
            <div>
                <h1 className="text-2xl font-semibold text-balance">{title}</h1>
                {description && (
                    <p className="mt-1 text-sm text-muted-foreground">
                        {description}
                    </p>
                )}
            </div>
            <div className="flex items-center gap-3">
                {showEnvironmentSelector && <EnvironmentSelector />}
                {actions}
            </div>
        </div>
    );
}
