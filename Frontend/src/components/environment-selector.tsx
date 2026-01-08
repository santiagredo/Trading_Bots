"use client"

import { Button } from "@/components/ui/button"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/components/ui/dropdown-menu"
import { ChevronDown } from "lucide-react"
import { useEnvironment } from "@/lib/hooks/use-environment"

export function EnvironmentSelector() {
  const { environment, setEnvironment } = useEnvironment()

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="outline" size="sm" className="gap-2 bg-transparent">
          <span className="text-xs font-medium uppercase">{environment}</span>
          <ChevronDown className="h-3 w-3" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuItem onClick={() => setEnvironment("dev")}>
          <div className="flex flex-col">
            <span className="font-medium">Development</span>
            <span className="text-xs text-muted-foreground">localhost:8080</span>
          </div>
        </DropdownMenuItem>
        <DropdownMenuItem onClick={() => setEnvironment("prod")}>
          <div className="flex flex-col">
            <span className="font-medium">Production</span>
            <span className="text-xs text-muted-foreground">api.trading-bot.com</span>
          </div>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}
