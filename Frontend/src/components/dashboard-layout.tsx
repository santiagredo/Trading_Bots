"use client";

import type React from "react";
import { useState } from "react";
import { useLocation, useNavigate } from "react-router-dom";
import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import {
    Activity,
    Layers,
    LayoutDashboard,
    Menu,
    TrendingUp,
    Settings,
    BarChart3,
    Wallet,
    Gauge,
    LineChart,
    FileText,
    ShoppingCart,
    GitBranch,
    Network,
    Target,
    ListTodo,
    Logs,
} from "lucide-react";

const menuItems = [
    { name: "Dashboard", href: "/", icon: LayoutDashboard },
    { name: "Configuration", href: "/configuration", icon: Settings },
    { name: "Actions", href: "/actions", icon: Activity },
    { name: "Assets", href: "/assets", icon: Wallet },
    { name: "Binance", href: "/binance", icon: TrendingUp },
    { name: "Health Check", href: "/health-check", icon: Gauge },
    { name: "Indicators", href: "/indicators", icon: BarChart3 },
    { name: "Ledgers", href: "/ledgers", icon: FileText },
    { name: "Metrics", href: "/metrics", icon: LineChart },
    { name: "Orders", href: "/orders", icon: ShoppingCart },
    { name: "Pairs", href: "/pairs", icon: GitBranch },
    { name: "Logs", href: "/logs", icon: Logs },
    {
        name: "Strategies Overview",
        href: "/strategies-overview",
        icon: Network,
    },
    { name: "Strategies", href: "/strategies", icon: Target },
    { name: "Tasks", href: "/tasks", icon: ListTodo },
];

interface DashboardLayoutProps {
    children: React.ReactNode;
}

export function DashboardLayout({ children }: DashboardLayoutProps) {
    const [sidebarOpen, setSidebarOpen] = useState(true);
    const navigate = useNavigate();
    const location = useLocation();

    return (
        <div className="flex h-screen w-full overflow-hidden bg-background">
            <aside
                className={cn(
                    "flex flex-col border-r border-border bg-card transition-all duration-300",
                    sidebarOpen ? "w-64" : "w-16"
                )}
            >
                <div className="flex h-16 items-center justify-between border-b border-border px-4">
                    {sidebarOpen && (
                        <div className="flex items-center gap-2">
                            <Layers className="h-6 w-6 text-primary" />
                            <span className="font-semibold text-foreground">
                                Trading Bot
                            </span>
                        </div>
                    )}
                    <Button
                        variant="ghost"
                        size="icon"
                        onClick={() => setSidebarOpen(!sidebarOpen)}
                        className="h-8 w-8"
                    >
                        <Menu className="h-4 w-4" />
                    </Button>
                </div>

                <nav className="flex-1 overflow-y-auto p-2">
                    <ul className="space-y-1">
                        {menuItems.map((item) => {
                            const Icon = item.icon;
                            const isActive = location.pathname === item.href;
                            return (
                                <li key={item.name}>
                                    <button
                                        onClick={() => navigate(item.href)}
                                        className={cn(
                                            "flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm transition-colors",
                                            isActive
                                                ? "bg-primary text-primary-foreground"
                                                : "text-muted-foreground hover:bg-accent hover:text-accent-foreground"
                                        )}
                                    >
                                        <Icon className="h-4 w-4 shrink-0" />
                                        {sidebarOpen && (
                                            <span>{item.name}</span>
                                        )}
                                    </button>
                                </li>
                            );
                        })}
                    </ul>
                </nav>
            </aside>

            <main className="flex-1 overflow-hidden">{children}</main>
        </div>
    );
}
