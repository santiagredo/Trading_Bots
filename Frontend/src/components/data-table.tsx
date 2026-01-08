"use client";

import type React from "react";

import { useState } from "react";
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Search, RefreshCw, Edit, Trash2 } from "lucide-react";
import { useEnvironment, type Environment } from "@/lib/hooks/use-environment";
import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogCancel,
    AlertDialogContent,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
} from "@/components/ui/alert-dialog";

export interface Column<T> {
    key: string;
    label: string;
    render?: (value: T[keyof T], row: T) => React.ReactNode;
}

interface DataTableProps<T> {
    columns: Column<T>[];
    data: T[];
    onRefresh?: (env: Environment) => void;
    onEdit?: (row: T) => void;
    onDelete?: (row: T) => void;
    searchable?: boolean;
    searchPlaceholder?: string;
    emptyMessage?: string;
    showEnvironment?: boolean;
    showActions?: boolean;
    deleteConfirmTitle?: string;
    deleteConfirmDescription?: string;
    // onEnvChange?: (env: Environment) => void;
}

export function DataTable<T extends Record<string, any>>({
    columns,
    data,
    onRefresh,
    onEdit,
    onDelete,
    searchable = true,
    searchPlaceholder = "Search...",
    emptyMessage = "No data available",
    showEnvironment = false,
    showActions = true,
    deleteConfirmTitle = "Are you sure?",
    deleteConfirmDescription = "This action cannot be undone. This will permanently delete the item.",
}: // onEnvChange,
DataTableProps<T>) {
    const [searchQuery, setSearchQuery] = useState("");
    const [isRefreshing, setIsRefreshing] = useState(false);
    const { environment, setEnvironment } = useEnvironment();
    const [deleteItem, setDeleteItem] = useState<T | null>(null);

    const filteredData = searchQuery
        ? data.filter((row) =>
              Object.values(row).some((value) =>
                  String(value)
                      .toLowerCase()
                      .includes(searchQuery.toLowerCase())
              )
          )
        : data;

    const handleDeleteConfirm = () => {
        if (deleteItem && onDelete) {
            onDelete(deleteItem);
            setDeleteItem(null);
        }
    };

    const shouldShowActions = showActions && (onEdit || onDelete);
    const displayColumns = shouldShowActions
        ? [
              ...columns,
              { key: "__actions", label: "Actions", render: undefined },
          ]
        : columns;

    const handleRefresh = async (env: Environment) => {
        setIsRefreshing(true);
        setEnvironment(env);
        await onRefresh?.(env);
        setIsRefreshing(false);
    };

    return (
        <div className="space-y-4">
            {(searchable || onRefresh) && (
                <div className="flex items-center justify-between gap-4">
                    {searchable && (
                        <div className="relative flex-1 max-w-sm">
                            <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                            <Input
                                placeholder={searchPlaceholder}
                                value={searchQuery}
                                onChange={(e) => setSearchQuery(e.target.value)}
                                className="pl-10"
                            />
                        </div>
                    )}
                    <div className="flex items-center gap-2">
                        {showEnvironment && (
                            <div className="flex gap-2">
                                <Button
                                    variant={
                                        environment === "dev"
                                            ? "default"
                                            : "outline"
                                    }
                                    size="sm"
                                    onClick={() => handleRefresh("dev")}
                                    disabled={isRefreshing}
                                >
                                    Dev
                                </Button>
                                <Button
                                    variant={
                                        environment === "prod"
                                            ? "default"
                                            : "outline"
                                    }
                                    size="sm"
                                    onClick={() => handleRefresh("prod")}
                                    disabled={isRefreshing}
                                >
                                    Prod
                                </Button>
                            </div>
                        )}
                        {onRefresh && (
                            <Button
                                variant="outline"
                                size="sm"
                                onClick={() => handleRefresh(environment)}
                                disabled={isRefreshing}
                            >
                                <RefreshCw
                                    className={`h-4 w-4 ${
                                        isRefreshing ? "animate-spin" : ""
                                    }`}
                                />
                            </Button>
                        )}
                    </div>
                </div>
            )}

            <div className="rounded-md border border-border">
                <Table>
                    <TableHeader>
                        <TableRow>
                            {displayColumns.map((column) => (
                                <TableHead key={column.key}>
                                    {column.label}
                                </TableHead>
                            ))}
                        </TableRow>
                    </TableHeader>

                    <TableBody>
                        {filteredData.length === 0 ? (
                            <TableRow>
                                <TableCell
                                    colSpan={displayColumns.length}
                                    className="h-24 text-center text-muted-foreground"
                                >
                                    {emptyMessage}
                                </TableCell>
                            </TableRow>
                        ) : (
                            filteredData.map((row, rowIndex) => (
                                <TableRow key={rowIndex}>
                                    {columns.map((column) => (
                                        <TableCell key={column.key}>
                                            {column.render
                                                ? column.render(
                                                      row[column.key],
                                                      row
                                                  )
                                                : String(row[column.key] ?? "")}
                                        </TableCell>
                                    ))}
                                    {shouldShowActions && (
                                        <TableCell>
                                            <div className="flex items-center gap-1">
                                                {onEdit && (
                                                    <Button
                                                        variant="ghost"
                                                        size="icon-sm"
                                                        onClick={() =>
                                                            onEdit(row)
                                                        }
                                                        title="Edit"
                                                    >
                                                        <Edit className="h-4 w-4" />
                                                    </Button>
                                                )}
                                                {onDelete && (
                                                    <Button
                                                        variant="ghost"
                                                        size="icon-sm"
                                                        onClick={() =>
                                                            setDeleteItem(row)
                                                        }
                                                        title="Delete"
                                                        className="text-destructive hover:text-destructive"
                                                    >
                                                        <Trash2 className="h-4 w-4" />
                                                    </Button>
                                                )}
                                            </div>
                                        </TableCell>
                                    )}
                                </TableRow>
                            ))
                        )}
                    </TableBody>
                </Table>
            </div>

            <AlertDialog
                open={!!deleteItem}
                onOpenChange={(open) => !open && setDeleteItem(null)}
            >
                <AlertDialogContent>
                    <AlertDialogHeader>
                        <AlertDialogTitle>
                            {deleteConfirmTitle}
                        </AlertDialogTitle>
                        <AlertDialogDescription>
                            {deleteConfirmDescription}
                        </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                        <AlertDialogCancel>Cancel</AlertDialogCancel>
                        <AlertDialogAction
                            onClick={handleDeleteConfirm}
                            className="bg-destructive hover:bg-destructive/90"
                        >
                            Delete
                        </AlertDialogAction>
                    </AlertDialogFooter>
                </AlertDialogContent>
            </AlertDialog>
        </div>
    );
}
