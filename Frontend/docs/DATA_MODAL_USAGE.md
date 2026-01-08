# DataModal - Complete Usage Guide

## Overview

`DataModal` is a generic and reusable component for creating and editing any type of data in your application. It works similarly to `DataTable`, accepting dynamic configurations that define form fields.

## Features

-   **Generic with TypeScript**: Works with any data type using `<T>`
-   **Multiple field types**: text, number, email, textarea, select, date, checkbox, etc.
-   **Built-in validation**: Per-field and global validation
-   **Create/edit mode**: Automatically detects mode based on whether data is provided
-   **Toast integration**: Automatic success/error notifications
-   **Customizable messages**: Configurable titles and descriptions
-   **Required fields**: Support for required fields with validation
-   **Disabled fields**: Useful for read-only data in edit mode

## Installation

The component is already available at `src/components/data-modal.tsx`. Just import it:

```tsx
import { DataModal, type DataModalConfig } from "@/components/data-modal";
```

## Basic Usage

### 1. Define your data type

```tsx
interface Strategy {
    id: string;
    name: string;
    type: string;
    status: "active" | "inactive";
}
```

### 2. Configure the modal

```tsx
const modalConfig: DataModalConfig<Strategy> = {
    title: "Strategy",
    description: "Manage your trading strategies",
    fields: [
        {
            key: "name",
            label: "Name",
            type: "text",
            required: true,
            placeholder: "Strategy name",
        },
        {
            key: "type",
            label: "Type",
            type: "select",
            required: true,
            options: [
                { value: "scalping", label: "Scalping" },
                { value: "momentum", label: "Momentum" },
            ],
        },
        {
            key: "status",
            label: "Status",
            type: "select",
            defaultValue: "inactive",
            options: [
                { value: "active", label: "Active" },
                { value: "inactive", label: "Inactive" },
            ],
        },
    ],
    onSave: (data) => {
        console.log("Data saved:", data);
        // Here you save the data to your state or make an API call
    },
};
```

### 3. Use the modal in your component

```tsx
export default function StrategiesPage() {
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [editingData, setEditingData] = useState<Strategy | null>(null);

    return (
        <>
            <Button
                onClick={() => {
                    setEditingData(null); // null = create mode
                    setIsModalOpen(true);
                }}
            >
                New Strategy
            </Button>

            <DataModal
                open={isModalOpen}
                onOpenChange={setIsModalOpen}
                config={modalConfig}
                data={editingData} // null to create, object to edit
            />
        </>
    );
}
```

## Available Field Types

### Text

```tsx
{
  key: "name",
  label: "Name",
  type: "text",
  placeholder: "Enter a name",
  required: true,
}
```

### Number

```tsx
{
  key: "amount",
  label: "Amount",
  type: "number",
  placeholder: "0.00",
  required: true,
}
```

### Email

```tsx
{
  key: "email",
  label: "Email",
  type: "email",
  placeholder: "user@example.com",
  required: true,
}
```

### Textarea

```tsx
{
  key: "description",
  label: "Description",
  type: "textarea",
  placeholder: "Describe your strategy...",
  helpText: "Maximum 500 characters",
}
```

### Select

```tsx
{
  key: "status",
  label: "Status",
  type: "select",
  options: [
    { value: "active", label: "Active" },
    { value: "inactive", label: "Inactive" },
    { value: "pending", label: "Pending" },
  ],
  defaultValue: "pending",
}
```

### Date

```tsx
{
  key: "startDate",
  label: "Start Date",
  type: "date",
}
```

### Datetime Local

```tsx
{
  key: "executionTime",
  label: "Execution Time",
  type: "datetime-local",
}
```

### Checkbox

```tsx
{
  key: "autoTrade",
  label: "Auto-trading enabled",
  type: "checkbox",
  defaultValue: false,
}
```

## Validation

### Per-field validation

```tsx
{
  key: "pairs",
  label: "Pairs",
  type: "text",
  required: true,
  validation: (value) => {
    const pairs = String(value).split(",")
    if (pairs.length < 2) {
      return "You must add at least 2 pairs"
    }
    return null // null = valid
  },
}
```

### Global validation

```tsx
const modalConfig: DataModalConfig<Order> = {
    // ... fields ...
    validate: (data) => {
        if (data.stopLoss && data.takeProfit) {
            if (Number(data.stopLoss) >= Number(data.takeProfit)) {
                return "Stop loss must be lower than take profit";
            }
        }
        return null; // null = valid
    },
};
```

## Create vs Edit Mode

The modal automatically detects the mode based on whether you pass data:

```tsx
// CREATE mode (data = null)
<DataModal
  open={isOpen}
  onOpenChange={setIsOpen}
  config={config}
  data={null} // No data = create
/>

// EDIT mode (data = object)
<DataModal
  open={isOpen}
  onOpenChange={setIsOpen}
  config={config}
  data={selectedStrategy} // With data = edit
/>
```

### Customize titles by mode

```tsx
const config: DataModalConfig<Strategy> = {
    title: "Strategy", // Not used if you define createTitle/editTitle
    createTitle: "New Strategy",
    editTitle: "Edit Strategy",
    createDescription: "Create a new trading strategy",
    editDescription: "Modify strategy data",
    // ...
};
```

## Integration with DataTable

Complete example of how to use DataModal with DataTable:

```tsx
interface Asset {
    id: string;
    symbol: string;
    balance: number;
    status: "available" | "locked";
}

export default function AssetsPage() {
    const [assets, setAssets] = useState<Asset[]>([]);
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [editingAsset, setEditingAsset] = useState<Asset | null>(null);

    const modalConfig: DataModalConfig<Asset> = {
        title: "Asset",
        fields: [
            {
                key: "symbol",
                label: "Symbol",
                type: "text",
                placeholder: "BTC",
                required: true,
            },
            {
                key: "balance",
                label: "Balance",
                type: "number",
                placeholder: "0.00",
                required: true,
            },
            {
                key: "status",
                label: "Status",
                type: "select",
                defaultValue: "available",
                options: [
                    { value: "available", label: "Available" },
                    { value: "locked", label: "Locked" },
                ],
            },
        ],
        onSave: (data) => {
            if (editingAsset) {
                // Edit existing
                setAssets((prev) =>
                    prev.map((a) =>
                        a.id === editingAsset.id
                            ? { ...editingAsset, ...data }
                            : a
                    )
                );
            } else {
                // Create new
                const newAsset: Asset = {
                    id: `AST-${assets.length + 1}`,
                    ...data,
                } as Asset;
                setAssets((prev) => [...prev, newAsset]);
            }
        },
    };

    const columns = [
        { key: "symbol", label: "Symbol" },
        { key: "balance", label: "Balance" },
        { key: "status", label: "Status" },
        {
            key: "actions",
            label: "Actions",
            render: (_: unknown, row: Asset) => (
                <Button
                    size="icon-sm"
                    variant="ghost"
                    onClick={() => {
                        setEditingAsset(row);
                        setIsModalOpen(true);
                    }}
                >
                    <Edit className="h-4 w-4" />
                </Button>
            ),
        },
    ];

    return (
        <>
            <Button
                onClick={() => {
                    setEditingAsset(null);
                    setIsModalOpen(true);
                }}
            >
                New Asset
            </Button>

            <DataTable columns={columns} data={assets} />

            <DataModal
                open={isModalOpen}
                onOpenChange={setIsModalOpen}
                config={modalConfig}
                data={editingAsset}
            />
        </>
    );
}
```

## Complete Example: Orders

```tsx
interface Order {
    id: string;
    pair: string;
    type: "buy" | "sell";
    price: number;
    amount: number;
    stopLoss?: number;
    takeProfit?: number;
    notes?: string;
}

const orderModalConfig: DataModalConfig<Order> = {
    title: "Order",
    createTitle: "New Order",
    editTitle: "Edit Order",
    fields: [
        {
            key: "pair",
            label: "Pair",
            type: "text",
            placeholder: "BTC/USDT",
            required: true,
        },
        {
            key: "type",
            label: "Type",
            type: "select",
            required: true,
            options: [
                { value: "buy", label: "Buy" },
                { value: "sell", label: "Sell" },
            ],
        },
        {
            key: "price",
            label: "Price",
            type: "number",
            placeholder: "0.00",
            required: true,
            validation: (value) => {
                if (Number(value) <= 0) {
                    return "Price must be greater than 0";
                }
                return null;
            },
        },
        {
            key: "amount",
            label: "Amount",
            type: "number",
            placeholder: "0.00",
            required: true,
            validation: (value) => {
                if (Number(value) <= 0) {
                    return "Amount must be greater than 0";
                }
                return null;
            },
        },
        {
            key: "stopLoss",
            label: "Stop Loss",
            type: "number",
            placeholder: "Optional",
        },
        {
            key: "takeProfit",
            label: "Take Profit",
            type: "number",
            placeholder: "Optional",
        },
        {
            key: "notes",
            label: "Notes",
            type: "textarea",
            placeholder: "Additional notes about the order...",
            helpText: "Optional: Add relevant information",
        },
    ],
    validate: (data) => {
        if (data.stopLoss && data.takeProfit) {
            const stopLoss = Number(data.stopLoss);
            const takeProfit = Number(data.takeProfit);
            const price = Number(data.price);

            if (data.type === "buy") {
                if (stopLoss >= price) {
                    return "For a buy order, stop loss must be lower than price";
                }
                if (takeProfit <= price) {
                    return "For a buy order, take profit must be higher than price";
                }
            } else {
                if (stopLoss <= price) {
                    return "For a sell order, stop loss must be higher than price";
                }
                if (takeProfit >= price) {
                    return "For a sell order, take profit must be lower than price";
                }
            }
        }
        return null;
    },
    onSave: async (data) => {
        // API call
        await fetch("/api/orders", {
            method: "POST",
            body: JSON.stringify(data),
        });
    },
};
```

## Tips and Best Practices

1. **Validation**: Use per-field validation for simple errors and global validation for complex rules between fields
2. **Default values**: Use `defaultValue` to pre-fill common fields
3. **Help text**: Add `helpText` to guide users on what to enter
4. **Data transformation**: Transform data in `onSave` before saving (e.g., convert strings to arrays)
5. **Async onSave**: `onSave` supports promises for API calls
6. **Reusability**: Define modal config outside the component if it doesn't change
7. **TypeScript**: Leverage generic typing for autocomplete and type safety

## API Reference

### DataModalConfig<T>

| Property          | Type                                        | Description                           |
| ----------------- | ------------------------------------------- | ------------------------------------- |
| title             | string                                      | Base modal title                      |
| description       | string                                      | Base description                      |
| createTitle       | string                                      | Title in create mode (optional)       |
| editTitle         | string                                      | Title in edit mode (optional)         |
| createDescription | string                                      | Description in create mode (optional) |
| editDescription   | string                                      | Description in edit mode (optional)   |
| fields            | FieldConfig[]                               | Array of field configurations         |
| onSave            | (data: Partial<T>) => void \| Promise<void> | Callback on save                      |
| validate          | (data: Partial<T>) => string \| null        | Global validation (optional)          |

### FieldConfig

| Property     | Type                      | Description                           |
| ------------ | ------------------------- | ------------------------------------- |
| key          | string                    | Field name (must match type property) |
| label        | string                    | Visible field label                   |
| type         | FieldType                 | Input type to render                  |
| placeholder  | string                    | Placeholder text (optional)           |
| required     | boolean                   | Whether field is required             |
| options      | Array<{value, label}>     | Options for select                    |
| helpText     | string                    | Help text below field                 |
| defaultValue | unknown                   | Default value                         |
| disabled     | boolean                   | Whether field is disabled             |
| validation   | (value) => string \| null | Custom validation function            |

### FieldType

`text | number | email | password | textarea | select | date | datetime-local | checkbox`
