import { Badge } from "@/components/ui/badge"
import { cn } from "@/lib/utils"

interface StatusBadgeProps {
  status: "active" | "inactive" | "error" | "pending" | "success" | "warning"
  label?: string
}

export function StatusBadge({ status, label }: StatusBadgeProps) {
  const statusConfig = {
    active: { className: "bg-success text-foreground", label: label || "Active" },
    inactive: { className: "bg-muted text-muted-foreground", label: label || "Inactive" },
    error: { className: "bg-destructive text-destructive-foreground", label: label || "Error" },
    pending: { className: "bg-warning text-foreground", label: label || "Pending" },
    success: { className: "bg-success text-foreground", label: label || "Success" },
    warning: { className: "bg-warning text-foreground", label: label || "Warning" },
  }

  const config = statusConfig[status]

  return <Badge className={cn(config.className)}>{config.label}</Badge>
}
