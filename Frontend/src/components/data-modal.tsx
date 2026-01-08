"use client";

import type React from "react";
import { useState, useEffect } from "react";
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select";
import { Textarea } from "@/components/ui/textarea";
import { useToast } from "@/lib/hooks/use-toast";

export type FieldType =
    | "text"
    | "number"
    | "email"
    | "password"
    | "textarea"
    | "select"
    | "date"
    | "datetime-local"
    | "checkbox";

export interface FieldConfig {
    key: string;
    label: string;
    type: FieldType;
    placeholder?: string;
    required?: boolean;
    options?: { value: string; label: string }[];
    helpText?: string;
    defaultValue?: unknown;
    disabled?: boolean;
    validation?: (value: unknown) => string | null; // Retorna error message o null si es válido
}

export interface DataModalConfig<T> {
    title: string;
    description?: string;
    createTitle?: string;
    editTitle?: string;
    createDescription?: string;
    editDescription?: string;
    fields: FieldConfig[];
    onSave: (data: Partial<T>) => void | Promise<void>;
    validate?: (data: Partial<T>) => string | null; // Validación global
}

interface DataModalProps<T> {
    open: boolean;
    onOpenChange: (open: boolean) => void;
    config: DataModalConfig<T>;
    data?: T | null; // Si hay data, es modo edición
}

export function DataModal<T>({
    open,
    onOpenChange,
    config,
    data,
}: DataModalProps<T>) {
    const { toast } = useToast();
    const [formData, setFormData] = useState<Record<string, unknown>>({});
    const [errors, setErrors] = useState<Record<string, string>>({});
    const [isSubmitting, setIsSubmitting] = useState(false);

    const isEditMode = !!data;

    useEffect(() => {
        if (!open) return;

        const initialData: Record<string, unknown> = {};

        config.fields.forEach((field) => {
            if (data && field.key in (data as Record<string, unknown>)) {
                initialData[field.key] = (data as Record<string, unknown>)[
                    field.key
                ];
            } else if (field.defaultValue !== undefined) {
                initialData[field.key] = field.defaultValue;
            } else if (field.type === "checkbox") {
                initialData[field.key] = false;
            } else {
                initialData[field.key] = "";
            }
        });

        setFormData(initialData);
        setErrors({});
    }, [open, data, config.fields]);

    const validateField = (
        field: FieldConfig,
        value: unknown
    ): string | null => {
        if (field.required && !value) {
            return `${field.label} is required`;
        }
        if (field.validation) {
            return field.validation(value);
        }
        return null;
    };

    const handleFieldChange = (
        key: string,
        value: unknown,
        type?: FieldType
    ) => {
        setFormData((prev) => ({
            ...prev,
            [key]:
                type === "number"
                    ? value === "" || value === null
                        ? ""
                        : Number(value)
                    : value,
        }));

        if (errors[key]) {
            setErrors((prev) => {
                const newErrors = { ...prev };
                delete newErrors[key];
                return newErrors;
            });
        }
    };

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        const newErrors: Record<string, string> = {};

        // Validar cada campo
        config.fields.forEach((field) => {
            const error = validateField(field, formData[field.key]);
            if (error) {
                newErrors[field.key] = error;
            }
        });

        // Validación global
        if (config.validate) {
            const globalError = config.validate(formData as Partial<T>);
            if (globalError) {
                toast({
                    title: "Validation error",
                    description: globalError,
                    variant: "error",
                });
                return;
            }
        }

        if (Object.keys(newErrors).length > 0) {
            setErrors(newErrors);
            toast({
                title: "Validation error",
                description: "Please correct errors in form",
                variant: "error",
            });
            return;
        }

        setIsSubmitting(true);

        try {
            const submitData: Record<string, unknown> = {};

            config.fields.forEach((field) => {
                const rawValue = formData[field.key];

                if (field.type === "number") {
                    submitData[field.key] =
                        rawValue === "" || rawValue === null
                            ? null
                            : Number(rawValue);
                } else {
                    submitData[field.key] = rawValue;
                }
            });

            await config.onSave(formData as Partial<T>);

            const title = isEditMode
                ? config.editTitle || `${config.title} updated`
                : config.createTitle || `${config.title} created`;

            toast({
                title,
                description: `Data saved successfully`,
                variant: "success",
            });

            onOpenChange(false);
        } catch (error) {
            toast({
                title: "Error",
                description:
                    error instanceof Error
                        ? error.message
                        : "Data couldn't be saved",
                variant: "error",
            });
        } finally {
            setIsSubmitting(false);
        }
    };

    const renderField = (field: FieldConfig) => {
        const value = formData[field.key];
        const error = errors[field.key];
        const inputId = `field-${field.key}`;

        switch (field.type) {
            case "select":
                return (
                    <div key={field.key} className="grid gap-2">
                        <Label htmlFor={inputId}>
                            {field.label}{" "}
                            {field.required && (
                                <span className="text-destructive">*</span>
                            )}
                        </Label>
                        <Select
                            value={String(value || "")}
                            onValueChange={(newValue) =>
                                handleFieldChange(field.key, newValue)
                            }
                            disabled={field.disabled}
                        >
                            <SelectTrigger
                                id={inputId}
                                className={error ? "border-destructive" : ""}
                            >
                                <SelectValue
                                    placeholder={
                                        field.placeholder ||
                                        `Select ${field.label.toLowerCase()}`
                                    }
                                />
                            </SelectTrigger>
                            <SelectContent>
                                {field.options?.map((option) => (
                                    <SelectItem
                                        key={option.value}
                                        value={option.value}
                                    >
                                        {option.label}
                                    </SelectItem>
                                ))}
                            </SelectContent>
                        </Select>
                        {error && (
                            <p className="text-xs text-destructive">{error}</p>
                        )}
                        {field.helpText && !error && (
                            <p className="text-xs text-muted-foreground">
                                {field.helpText}
                            </p>
                        )}
                    </div>
                );

            case "textarea":
                return (
                    <div key={field.key} className="grid gap-2">
                        <Label htmlFor={inputId}>
                            {field.label}{" "}
                            {field.required && (
                                <span className="text-destructive">*</span>
                            )}
                        </Label>
                        <Textarea
                            id={inputId}
                            placeholder={field.placeholder}
                            value={String(value || "")}
                            onChange={(e) =>
                                handleFieldChange(field.key, e.target.value)
                            }
                            disabled={field.disabled}
                            className={error ? "border-destructive" : ""}
                        />
                        {error && (
                            <p className="text-xs text-destructive">{error}</p>
                        )}
                        {field.helpText && !error && (
                            <p className="text-xs text-muted-foreground">
                                {field.helpText}
                            </p>
                        )}
                    </div>
                );

            case "checkbox":
                return (
                    <div key={field.key} className="flex items-center gap-2">
                        <input
                            type="checkbox"
                            id={inputId}
                            checked={Boolean(value)}
                            onChange={(e) =>
                                handleFieldChange(field.key, e.target.checked)
                            }
                            disabled={field.disabled}
                            className="h-4 w-4"
                        />
                        <Label htmlFor={inputId} className="cursor-pointer">
                            {field.label}{" "}
                            {field.required && (
                                <span className="text-destructive">*</span>
                            )}
                        </Label>
                        {field.helpText && (
                            <p className="text-xs text-muted-foreground ml-6">
                                {field.helpText}
                            </p>
                        )}
                    </div>
                );

            default:
                return (
                    <div key={field.key} className="grid gap-2">
                        <Label htmlFor={inputId}>
                            {field.label}{" "}
                            {field.required && (
                                <span className="text-destructive">*</span>
                            )}
                        </Label>
                        <Input
                            id={inputId}
                            type={field.type}
                            placeholder={field.placeholder}
                            value={String(value ?? "")} // usar ?? "" para evitar undefined
                            onChange={(e) =>
                                handleFieldChange(
                                    field.key,
                                    e.target.value,
                                    field.type
                                )
                            }
                            disabled={field.disabled}
                            className={error ? "border-destructive" : ""}
                        />
                        {error && (
                            <p className="text-xs text-destructive">{error}</p>
                        )}
                        {field.helpText && !error && (
                            <p className="text-xs text-muted-foreground">
                                {field.helpText}
                            </p>
                        )}
                    </div>
                );
        }
    };

    return (
        <Dialog open={open} onOpenChange={onOpenChange}>
            <DialogContent className="sm:max-w-150 max-h-[90vh] overflow-y-auto">
                <form onSubmit={handleSubmit}>
                    <DialogHeader>
                        <DialogTitle>
                            {isEditMode
                                ? config.editTitle || `Edit ${config.title}`
                                : config.createTitle || `New ${config.title}`}
                        </DialogTitle>
                        <DialogDescription>
                            {isEditMode
                                ? config.editDescription || config.description
                                : config.createDescription ||
                                  config.description}
                        </DialogDescription>
                    </DialogHeader>

                    <div className="grid gap-4 py-4">
                        {config.fields.map((field) => renderField(field))}
                    </div>

                    <DialogFooter>
                        <Button
                            type="button"
                            variant="outline"
                            onClick={() => onOpenChange(false)}
                            disabled={isSubmitting}
                        >
                            Cancelar
                        </Button>
                        <Button type="submit" disabled={isSubmitting}>
                            {isSubmitting
                                ? "Saving..."
                                : isEditMode
                                ? "Update"
                                : "Create"}
                        </Button>
                    </DialogFooter>
                </form>
            </DialogContent>
        </Dialog>
    );
}
