# DataTable Component Guide

## Overview

The `DataTable` component is a reusable, feature-rich table component designed for displaying and managing data in your crypto trading dashboard. It provides built-in search functionality, refresh capabilities, environment switching, and custom cell rendering.

## Location

`src/components/data-table.tsx`

---

## Features

-   **Search**: Filter data across all columns with real-time search
-   **Refresh**: Manual data refresh with loading state
-   **Environment Switching**: Switch between dev/prod environments with visual indicators
-   **Custom Rendering**: Render custom content in cells (badges, formatting, icons, etc.)
-   **Empty State**: Displays helpful message when no data is available
-   **Responsive**: Fully responsive design with proper overflow handling

---

## Basic Usage

### Simple Table

```tsx
import { DataTable } from "@/components/data-table";

function MyPage() {
    const data = [
        { id: 1, name: "Bitcoin", symbol: "BTC", price: 43250 },
        { id: 2, name: "Ethereum", symbol: "ETH", price: 2245 },
    ];

    const columns = [
        { key: "name", label: "Name" },
        { key: "symbol", label: "Symbol" },
        { key: "price", label: "Price" },
    ];

    return <DataTable columns={columns} data={data} />;
}
```

---

## Props

### DataTableProps<T>

| Prop                | Type                         | Required | Default               | Description                              |
| ------------------- | ---------------------------- | -------- | --------------------- | ---------------------------------------- |
| `columns`           | `Column<T>[]`                | Yes      | -                     | Array of column definitions              |
| `data`              | `T[]`                        | Yes      | -                     | Array of data objects to display         |
| `onRefresh`         | `(env: Environment) => void` | No       | -                     | Callback when refresh button is clicked  |
| `searchable`        | `boolean`                    | No       | `true`                | Enable/disable search functionality      |
| `searchPlaceholder` | `string`                     | No       | `"Search..."`         | Placeholder text for search input        |
| `emptyMessage`      | `string`                     | No       | `"No data available"` | Message shown when no data               |
| `showEnvironment`   | `boolean`                    | No       | `false`               | Show dev/prod environment toggle buttons |

### Column Definition

```typescript
interface Column<T> {
    key: string; // Property key in data object
    label: string; // Column header text
    render?: (value: unknown, row: T) => ReactNode; // Custom cell renderer
}
```

---

## Examples

### 1. Assets Page Example (with Custom Rendering)

This example shows how the Assets page uses the DataTable with custom rendering for formatted numbers and conditional styling.

```tsx
import { DataTable } from "@/components/data-table";
import type { Environment } from "@/lib/hooks/use-environment";

interface Asset {
    symbol: string;
    name: string;
    balance: number;
    value: number;
    change24h: number;
    allocation: number;
}

function AssetsPage() {
    const [assets, setAssets] = useState<Asset[]>([
        {
            symbol: "BTC",
            name: "Bitcoin",
            balance: 0.5423,
            value: 23450.23,
            change24h: 2.4,
            allocation: 45,
        },
        // ... more assets
    ]);

    const handleRefresh = async (env: Environment) => {
        // Fetch assets from your API
        const response = await fetch(`/api/assets?env=${env}`);
        const data = await response.json();
        setAssets(data);
    };

    const columns = [
        {
            key: "symbol",
            label: "Asset",
            // Custom render: Show symbol and name together
            render: (value: unknown, row: Asset) => (
                <div>
                    <p className="font-medium">{value}</p>
                    <p className="text-xs text-muted-foreground">{row.name}</p>
                </div>
            ),
        },
        {
            key: "balance",
            label: "Balance",
            // Custom render: Format as monospace with fixed decimals
            render: (value: unknown) => (
                <span className="font-mono">
                    {(value as number).toFixed(4)}
                </span>
            ),
        },
        {
            key: "value",
            label: "Value (USD)",
            // Custom render: Format as currency
            render: (value: unknown) => (
                <span className="font-mono">
                    ${(value as number).toFixed(2)}
                </span>
            ),
        },
        {
            key: "change24h",
            label: "24h Change",
            // Custom render: Conditional color based on positive/negative
            render: (value: unknown) => {
                const change = value as number;
                return (
                    <span
                        className={`font-mono ${
                            change >= 0 ? "text-success" : "text-destructive"
                        }`}
                    >
                        {change >= 0 ? "+" : ""}
                        {change.toFixed(2)}%
                    </span>
                );
            },
        },
        {
            key: "allocation",
            label: "Allocation",
            render: (value: unknown) => (
                <span className="font-mono">{value}%</span>
            ),
        },
    ];

    return (
        <DataTable
            columns={columns}
            data={assets}
            onRefresh={handleRefresh}
            searchPlaceholder="Search assets..."
            showEnvironment // Enable dev/prod switching
        />
    );
}
```

**What's happening here:**

1. **Custom Symbol Column**: Displays both symbol (bold) and full name (muted) in one cell
2. **Number Formatting**: Uses monospace font for better number alignment
3. **Conditional Styling**: Change percentage is green for positive, red for negative
4. **Environment Switching**: `showEnvironment` prop enables dev/prod toggle buttons
5. **Refresh Handler**: `onRefresh` callback receives the environment and fetches new data

---

### 2. Orders Page Example (with Badges)

This example shows using badges and status indicators in table cells.

```tsx
import { DataTable } from "@/components/data-table"
import { StatusBadge } from "@/components/status-badge"
import { Badge } from "@/components/ui/badge"

interface Order {
  id: string
  pair: string
  side: "buy" | "sell"
  type: string
  amount: number
  price: number
  status: "active" | "pending" | "success"
  timestamp: string
}

function OrdersPage() {
  const [orders, setOrders] = useState<Order[]>([...])

  const columns = [
    { key: "id", label: "Order ID" },
    { key: "pair", label: "Pair" },
    {
      key: "side",
      label: "Side",
      // Custom render: Badge with conditional styling
      render: (value: unknown) => (
        <Badge
          variant={value === "buy" ? "default" : "outline"}
          className={value === "buy" ? "bg-success" : ""}
        >
          {(value as string).toUpperCase()}
        </Badge>
      ),
    },
    { key: "type", label: "Type" },
    {
      key: "amount",
      label: "Amount",
      render: (value: unknown) => <span className="font-mono">{value}</span>,
    },
    {
      key: "price",
      label: "Price",
      render: (value: unknown) => (
        <span className="font-mono">${(value as number).toFixed(2)}</span>
      ),
    },
    {
      key: "status",
      label: "Status",
      // Custom render: Use StatusBadge component
      render: (value: unknown) => (
        <StatusBadge status={value as "active" | "pending" | "success"} />
      ),
    },
    { key: "timestamp", label: "Timestamp" },
  ]

  return (
    <DataTable
      columns={columns}
      data={orders}
      onRefresh={handleRefresh}
      searchPlaceholder="Search orders..."
      showEnvironment
    />
  )
}
```

---

### 3. Simple Table (No Refresh or Search)

```tsx
function SimpleExample() {
    const data = [
        { name: "Alice", age: 28, role: "Developer" },
        { name: "Bob", age: 34, role: "Designer" },
    ];

    const columns = [
        { key: "name", label: "Name" },
        { key: "age", label: "Age" },
        { key: "role", label: "Role" },
    ];

    return (
        <DataTable
            columns={columns}
            data={data}
            searchable={false} // Disable search
            emptyMessage="No users found"
        />
    );
}
```

---

### 4. With Action Buttons in Cells

```tsx
import { Button } from "@/components/ui/button";
import { Edit, Trash2 } from "lucide-react";

function TableWithActions() {
    const handleEdit = (id: string) => {
        console.log("Edit:", id);
    };

    const handleDelete = (id: string) => {
        console.log("Delete:", id);
    };

    const columns = [
        { key: "name", label: "Name" },
        { key: "email", label: "Email" },
        {
            key: "id",
            label: "Actions",
            render: (value: unknown, row: any) => (
                <div className="flex gap-2">
                    <Button
                        size="sm"
                        variant="ghost"
                        onClick={() => handleEdit(value as string)}
                    >
                        <Edit className="h-4 w-4" />
                    </Button>
                    <Button
                        size="sm"
                        variant="ghost"
                        onClick={() => handleDelete(value as string)}
                    >
                        <Trash2 className="h-4 w-4" />
                    </Button>
                </div>
            ),
        },
    ];

    return <DataTable columns={columns} data={users} />;
}
```

---

## How Search Works

The search functionality filters data across **all columns**:

```typescript
const filteredData = searchQuery
    ? data.filter((row) =>
          Object.values(row).some((value) =>
              String(value).toLowerCase().includes(searchQuery.toLowerCase())
          )
      )
    : data;
```

**Example**: If you search for "btc", it will match:

-   Symbol: "BTC"
-   Name: "Bitcoin"
-   Any other field containing "btc" (case-insensitive)

---

## How Environment Switching Works

When `showEnvironment={true}`:

1. Two buttons appear: "Dev" and "Prod"
2. Clicking a button calls `onRefresh(env)` with the selected environment
3. The active environment is highlighted with the default button variant
4. During refresh, all buttons are disabled and the refresh icon spins

```tsx
const handleRefresh = async (env: Environment) => {
    setIsRefreshing(true);
    await onRefresh?.(env);
    setIsRefreshing(false);
};
```

---

## How Refresh Works

The refresh button (↻ icon):

1. Calls the `onRefresh` callback with the current environment
2. Shows a spinning animation while `isRefreshing` is true
3. Disables both refresh and environment buttons during loading
4. Your callback should fetch new data and update state

```tsx
const handleRefresh = async (env: Environment) => {
    // Fetch data from API
    const response = await fetch(`/api/data?env=${env}`);
    const newData = await response.json();
    setData(newData);
};
```

---

## Custom Rendering Deep Dive

The `render` function in column definitions receives two arguments:

1. **value**: The value of the cell (e.g., `row[column.key]`)
2. **row**: The entire row object

This allows you to:

### Access Multiple Fields

```tsx
{
  key: "price",
  label: "Price Info",
  render: (value: unknown, row: Asset) => (
    <div>
      <div>${value}</div>
      <div className="text-xs">
        Vol: {row.volume}
      </div>
    </div>
  )
}
```

### Conditional Rendering

```tsx
{
  key: "status",
  label: "Status",
  render: (value: unknown) => {
    if (value === "active") return <span className="text-success">● Active</span>
    if (value === "pending") return <span className="text-warning">◐ Pending</span>
    return <span className="text-muted">○ Inactive</span>
  }
}
```

### Complex Components

```tsx
{
  key: "strategy",
  label: "Strategy",
  render: (value: unknown, row: Strategy) => (
    <div className="space-y-1">
      <Badge>{value}</Badge>
      <div className="text-xs text-muted-foreground">
        Win Rate: {row.winRate}%
      </div>
      <div className="text-xs">
        Trades: {row.totalTrades}
      </div>
    </div>
  )
}
```

---

## TypeScript Tips

### Strongly Type Your Data

```typescript
interface MyData {
    id: string;
    name: string;
    value: number;
}

// Type-safe columns
const columns: Column<MyData>[] = [
    {
        key: "name",
        label: "Name",
        render: (value, row) => {
            // row is typed as MyData
            // value is unknown (needs type assertion)
            return <div>{row.name}</div>;
        },
    },
];
```

### Generic Type Safety

```typescript
function MyComponent() {
  const data: MyData[] = [...]

  // DataTable<MyData> ensures type safety
  return <DataTable<MyData> columns={columns} data={data} />
}
```

---

## Common Patterns

### Pattern 1: Fetch on Mount and Refresh

```tsx
function MyPage() {
    const [data, setData] = useState([]);
    const { environment } = useEnvironment();

    // Fetch on mount
    useEffect(() => {
        fetchData(environment);
    }, [environment]);

    const fetchData = async (env: Environment) => {
        const response = await fetch(`/api/data?env=${env}`);
        const result = await response.json();
        setData(result);
    };

    return (
        <DataTable
            columns={columns}
            data={data}
            onRefresh={fetchData}
            showEnvironment
        />
    );
}
```

### Pattern 2: With Toast Notifications

```tsx
import { useToast } from "@/lib/hooks/use-toast";

function MyPage() {
    const [data, setData] = useState([]);
    const { toast } = useToast();

    const handleRefresh = async (env: Environment) => {
        try {
            const response = await fetch(`/api/data?env=${env}`);
            if (!response.ok) throw new Error("Failed to fetch");

            const result = await response.json();
            setData(result);

            toast({
                title: "Success",
                description: "Data refreshed successfully",
                variant: "success",
            });
        } catch (error) {
            toast({
                title: "Error",
                description: "Failed to refresh data",
                variant: "error",
            });
        }
    };

    return (
        <DataTable columns={columns} data={data} onRefresh={handleRefresh} />
    );
}
```

### Pattern 3: With useApiQuery Hook

```tsx
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { tradingApi } from "@/lib/api/trading";

function MyPage() {
    const { data, isLoading, refetch } = useApiQuery((env) =>
        tradingApi.getOrders(env)
    );

    if (isLoading) return <LoadingSpinner />;

    return (
        <DataTable
            columns={columns}
            data={data || []}
            onRefresh={(env) => refetch()}
            showEnvironment
        />
    );
}
```

---

## Styling Customization

### Custom Empty Message Style

The empty message uses default table styling. To customize:

```tsx
<DataTable
    columns={columns}
    data={data}
    emptyMessage="No transactions found. Start trading to see data here."
/>
```

### Custom Search Input Style

The search input inherits from the Input component. Modify `src/components/ui/input.tsx` to change all inputs, or wrap DataTable and override styles.

---

## Related Components

-   **StatusBadge** (`src/components/status-badge.tsx`): For status indicators
-   **EmptyState** (`src/components/empty-state.tsx`): For empty page states
-   **LoadingSpinner** (`src/components/loading-spinner.tsx`): For loading states
-   **JsonViewer** (`src/components/json-viewer.tsx`): For raw JSON display

---

## Best Practices

1. **Always provide a refresh handler** when data can change
2. **Use custom rendering** for formatted numbers, dates, and currency
3. **Enable environment switching** for pages with env-specific data
4. **Keep column keys matching data keys** for automatic rendering
5. **Use TypeScript interfaces** for type safety
6. **Provide meaningful empty messages** to guide users
7. **Use monospace fonts** for numbers and codes for better alignment
8. **Add loading states** in refresh handlers for better UX

---

## Troubleshooting

### Search not working?

-   Ensure your data is an array of objects
-   Check that column keys match data object keys
-   Search is case-insensitive and searches all fields

### Refresh button not appearing?

-   Ensure you passed the `onRefresh` prop
-   Check that the function is defined

### Environment buttons not showing?

-   Set `showEnvironment={true}` prop
-   Ensure `onRefresh` is also provided

### Custom render not showing?

-   Check that your render function returns valid React nodes
-   Verify type assertions for the value parameter
-   Check console for TypeScript errors

---

## Advanced Example: Full Page Integration

```tsx
import { useState, useEffect } from "react";
import { DashboardLayout } from "@/components/dashboard-layout";
import { PageHeader } from "@/components/page-header";
import { DataTable } from "@/components/data-table";
import { StatCard } from "@/components/stat-card";
import { useEnvironment } from "@/lib/hooks/use-environment";
import { useToast } from "@/lib/hooks/use-toast";
import { TrendingUp } from "lucide-react";

export default function MyPage() {
    const [data, setData] = useState([]);
    const [loading, setLoading] = useState(true);
    const { environment } = useEnvironment();
    const { toast } = useToast();

    useEffect(() => {
        fetchData(environment);
    }, [environment]);

    const fetchData = async (env: Environment) => {
        setLoading(true);
        try {
            const response = await fetch(`/api/endpoint?env=${env}`);
            if (!response.ok) throw new Error("Failed to fetch");

            const result = await response.json();
            setData(result);
        } catch (error) {
            toast({
                title: "Error",
                description: "Failed to load data",
                variant: "error",
            });
        } finally {
            setLoading(false);
        }
    };

    const columns = [
        { key: "id", label: "ID" },
        {
            key: "name",
            label: "Name",
            render: (value: unknown) => (
                <span className="font-medium">{value}</span>
            ),
        },
        // ... more columns
    ];

    if (loading) return <LoadingSpinner />;

    return (
        <DashboardLayout>
            <div className="flex h-full flex-col">
                <PageHeader
                    title="My Page"
                    description="Description here"
                    showEnvironmentSelector
                />
                <div className="flex-1 overflow-auto p-6">
                    <div className="grid gap-6">
                        <div className="grid gap-4 md:grid-cols-3">
                            <StatCard
                                title="Total Items"
                                value={data.length}
                                icon={TrendingUp}
                            />
                        </div>
                        <DataTable
                            columns={columns}
                            data={data}
                            onRefresh={fetchData}
                            searchPlaceholder="Search..."
                            showEnvironment
                        />
                    </div>
                </div>
            </div>
        </DashboardLayout>
    );
}
```

