"use client"

import { useState } from "react"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Copy, Check } from "lucide-react"

interface JsonViewerProps {
  title: string
  data: unknown
}

export function JsonViewer({ title, data }: JsonViewerProps) {
  const [copied, setCopied] = useState(false)

  const handleCopy = () => {
    navigator.clipboard.writeText(JSON.stringify(data, null, 2))
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between space-y-0">
        <CardTitle className="text-base">{title}</CardTitle>
        <Button variant="ghost" size="sm" onClick={handleCopy}>
          {copied ? <Check className="h-4 w-4 text-success" /> : <Copy className="h-4 w-4" />}
        </Button>
      </CardHeader>
      <CardContent>
        <pre className="rounded-md bg-muted p-4 text-xs overflow-auto max-h-96">
          <code className="font-mono">{JSON.stringify(data, null, 2)}</code>
        </pre>
      </CardContent>
    </Card>
  )
}
