import { useState } from "react";

export function useTagsInput(
    initial: string[] = [],
    useInternalState = false,
    onChange?: (value: string[]) => void
) {
    const [tags, setTags] = useState<string[]>(initial);
    const [inputValue, setInputValue] = useState("");
    const [optionsOpen, setOptionsOpen] = useState(false);
    const [dialogOpen, setDialogOpen] = useState(false);

    const currentTags = useInternalState ? tags : initial;

    const handleAdd = () => {
        const trimmed = inputValue.trim();
        if (trimmed && !tags.includes(trimmed)) {
            setTags([...tags, trimmed]);
        }
        setInputValue("");
    };

    const handleChange = (newValue: string[]) => {
        if (useInternalState) {
            setInputValue("");
            newValue.forEach((tag) => {
                if (!tags.includes(tag)) handleAdd();
            });
        } else {
            onChange?.(newValue);
        }
    };

    return {
        currentTags,
        optionsOpen,
        setOptionsOpen,
        dialogOpen,
        setDialogOpen,
        handleChange,
    };
}
