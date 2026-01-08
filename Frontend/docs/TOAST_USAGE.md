# Toast Notification System

## Overview

The toast notification system provides a simple way to show success, error, warning, and info messages to users after completing operations.

## Basic Usage

### 1. Import the hook

```tsx
import { useToastContext } from "@/components/toast-provider";
```

### 2. Use in your component

```tsx
export function MyComponent() {
    const { success, error, warning, info } = useToastContext();

    const handleSubmit = async () => {
        try {
            await submitData();
            success("Data saved!", "Your changes have been saved successfully");
        } catch (err) {
            error("Failed to save", "Please try again later");
        }
    };

    return <button onClick={handleSubmit}>Submit</button>;
}
```

## API Reference

### useToastContext()

Returns an object with the following methods:

#### `success(title: string, description?: string)`

Shows a green success toast notification.

```tsx
success("Order placed!", "Your order #1234 has been submitted");
```

#### `error(title: string, description?: string)`

Shows a red error toast notification.

```tsx
error("Connection failed", "Unable to reach the server");
```

#### `warning(title: string, description?: string)`

Shows an amber warning toast notification.

```tsx
warning("Balance low", "You have less than $100 remaining");
```

#### `info(title: string, description?: string)`

Shows a blue informational toast notification.

```tsx
info("New feature", "Check out our new trading pairs");
```

#### `toast(options: ToastOptions)`

Shows a custom toast with full control over options.

```tsx
toast({
    title: "Custom notification",
    description: "This will stay for 10 seconds",
    variant: "default", // "default" | "success" | "error" | "warning" | "info"
    duration: 10000, // milliseconds (default: 5000)
});
```

#### `dismiss(id: string)`

Manually dismiss a toast by its ID.

```tsx
const toastId = success("Processing...");
// Later...
dismiss(toastId);
```

## Real-World Examples

### API Request with Loading State

```tsx
import { useToastContext } from "@/components/toast-provider";
import { useState } from "react";

export function OrderForm() {
    const { success, error } = useToastContext();
    const [loading, setLoading] = useState(false);

    const handleCreateOrder = async (data) => {
        setLoading(true);
        try {
            const result = await createOrder(data);
            success("Order created!", `Order #${result.id} has been placed`);
        } catch (err) {
            error("Order failed", err.message || "Please try again");
        } finally {
            setLoading(false);
        }
    };

    return (
        <form onSubmit={handleCreateOrder}>
            {/* form fields */}
            <button type="submit" disabled={loading}>
                {loading ? "Creating..." : "Create Order"}
            </button>
        </form>
    );
}
```

### Delete Confirmation

```tsx
import { useToastContext } from "@/components/toast-provider";

export function DeleteButton({ itemId, onDelete }) {
    const { success, error, warning } = useToastContext();

    const handleDelete = async () => {
        warning("Deleting...", "This action cannot be undone");

        try {
            await deleteItem(itemId);
            success("Deleted", "Item has been removed");
            onDelete();
        } catch (err) {
            error("Delete failed", "Unable to delete item");
        }
    };

    return <button onClick={handleDelete}>Delete</button>;
}
```

### Environment Switching

```tsx
import { useToastContext } from "@/components/toast-provider";

export function EnvironmentSelector() {
    const { info } = useToastContext();

    const switchEnvironment = (env: "dev" | "prod") => {
        setEnvironment(env);
        info(
            `Switched to ${env.toUpperCase()}`,
            `All API calls will now use the ${env} environment`
        );
    };

    return (
        <select onChange={(e) => switchEnvironment(e.target.value)}>
            <option value="dev">Development</option>
            <option value="prod">Production</option>
        </select>
    );
}
```

## Styling

Toasts appear in the top-right corner of the screen and automatically dismiss after 5 seconds (customizable). Each variant has its own color scheme:

-   **Success**: Green (emerald-950)
-   **Error**: Red (red-950)
-   **Warning**: Amber (amber-950)
-   **Info**: Blue (blue-950)
-   **Default**: Follows your theme (card background)

## Notes

-   Toasts are automatically stacked when multiple are shown
-   The ToastProvider is already added to your App.tsx
-   You can have unlimited toasts visible at once
-   Toasts animate in from the right and slide out when dismissed
