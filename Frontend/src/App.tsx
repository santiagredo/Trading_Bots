import { Suspense } from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import { HomePage } from "./pages/home";
import ActionsPage from "./pages/actions";
import AssetsPage from "./pages/assets";
import BinancePage from "./pages/binance";
import ConfigurationPage from "./pages/configuration";
import HealthCheckPage from "./pages/health-check";
import IndicatorsPage from "./pages/indicators";
import LedgersPage from "./pages/ledgers";
import MetricsPage from "./pages/metrics";
import OrdersPage from "./pages/orders";
import PairsPage from "./pages/pairs";
import Logs from "./pages/logs";
import StrategiesOverviewPage from "./pages/strategies-overview";
import StrategiesPage from "./pages/strategies";
import TasksPage from "./pages/tasks";
import { AppLoading } from "./components/app-loading";
import { ToastProvider } from "./components/toast-provider";

export default function App() {
    return (
        <ToastProvider>
            <BrowserRouter>
                <Suspense fallback={<AppLoading />}></Suspense>
                <Routes>
                    <Route path="/" element={<HomePage />} />
                    <Route path="/actions" element={<ActionsPage />} />
                    <Route path="/assets" element={<AssetsPage />} />
                    <Route path="/binance" element={<BinancePage />} />
                    <Route
                        path="/configuration"
                        element={<ConfigurationPage />}
                    />
                    <Route path="/health-check" element={<HealthCheckPage />} />
                    <Route path="/indicators" element={<IndicatorsPage />} />
                    <Route path="/ledgers" element={<LedgersPage />} />
                    <Route path="/metrics" element={<MetricsPage />} />
                    <Route path="/orders" element={<OrdersPage />} />
                    <Route path="/pairs" element={<PairsPage />} />
                    <Route path="/logs" element={<Logs />} />
                    <Route
                        path="/strategies-overview"
                        element={<StrategiesOverviewPage />}
                    />
                    <Route path="/strategies" element={<StrategiesPage />} />
                    <Route path="/tasks" element={<TasksPage />} />
                </Routes>
            </BrowserRouter>
        </ToastProvider>
    );
}
