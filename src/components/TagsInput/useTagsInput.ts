import { useState } from "react";

export function useTagsInput(initial: string[] = []) {
    const [tags, setTags] = useState<string[]>(initial);
    const [inputValue, setInputValue] = useState("");

    const handleAdd = () => {
        const trimmed = inputValue.trim();
        if (trimmed && !tags.includes(trimmed)) {
            setTags([...tags, trimmed]);
        }
        setInputValue("");
    };

    return {
        tags,
        setInputValue,
        handleAdd,
    };
}
