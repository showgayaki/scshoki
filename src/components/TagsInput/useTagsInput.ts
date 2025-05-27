import { useState, KeyboardEvent } from "react";

function formatTag(contextId: string, tag: string): string {
    return contextId == "targetPages" && !tag.startsWith("/") ? "/" + tag.trim() : tag.trim();
}

export function useTagsInput(
    tags: string[],
    contextId: string,
    onChange: (value: string[]) => void,
) {
    const [optionsOpen, setOptionsOpen] = useState(false);
    const [dialogOpen, setDialogOpen] = useState(false);

    const handleAdd = (tag: string) => {
        const formatted = formatTag(contextId, tag);
        if (Boolean(formatted) && !tags.includes(formatted)) {
            console.log("Adding tag:", formatted);
            const newTags = [...tags, formatted];
            onChange?.(newTags);
        }
    };

    const handleChange = (newValue: string[]) => {
        const formatted = newValue.map(tag => formatTag(contextId, tag));
        onChange?.(formatted);
        console.log("Updated tags:", formatted);
    };

    const handleDelete = (tagToDelete: string) => {
        const newTags = tags.filter((tag) => tag !== tagToDelete);
        onChange?.(newTags);
    };

    const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
        console.log("e.currentTarget.value:", e.currentTarget.value);
        if (
            contextId === "targetPages" &&
            tags.length === 1 &&
            (e.key === "Backspace" || e.key === "Delete") &&
            e.currentTarget.value === ""
        ) {
            console.warn("Cannot delete the last tag in targetPages");
            e.preventDefault();
            e.stopPropagation();
            return;
        }

        if (e.key === "Enter" && Boolean(e.currentTarget.value.trim())) {
            e.preventDefault();
            handleAdd(e.currentTarget.value);
        }
    };

    return {
        tags,
        optionsOpen,
        setOptionsOpen,
        dialogOpen,
        setDialogOpen,
        handleChange,
        handleDelete,
        handleKeyDown,
    };
}
