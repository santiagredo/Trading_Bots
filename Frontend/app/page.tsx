import { DashboardLayout } from "@/components/dashboard-layout"
import { PageHeader } from "@/components/page-header"
import { EngineOverview } from "@/components/engine-overview"

export default function HomePage() {
  return (
    <DashboardLayout>
      <div className="flex h-full flex-col">
        <PageHeader title="Dashboard Overview" description="Monitor your crypto trading bot engine and performance" />
        <div className="flex-1 overflow-auto p-6">
          <EngineOverview />
        </div>
      </div>
    </DashboardLayout>
  )
}
