import { useState } from "react";
import { Autocomplete, Chip, Stack } from "@mui/material";
import TextInput from "@/components/TextInput";

interface TagsInputProps {
    id: string;
    label: string;
    placeholder?: string;
    options?: string[];
    value: string[];
    onChange: (value: string[]) => void;
}

export default function TagsInput({ id, label, placeholder, options = [], value, onChange }: TagsInputProps) {
    const [open, setOpen] = useState(false);

    return (
        <>
            <Autocomplete
                sx={{
                    // Autocompleteのpaddingと配下のTextFieldのpaddingで
                    // 二重にpaddingがかかるので、Autocompleteのpaddingを0にしておく
                    '& .MuiInputBase-root': {
                        p: 0,
                    },
                    mb: 1,
                }}
                multiple
                freeSolo
                options={options}
                open={open}
                onOpen={() => {
                    // 開かない（何もしない）
                }}
                onInputChange={(_, value, reason) => {
                    // 入力されたときだけ入力候補（options）を開く（reason === "input"）
                    setOpen(reason === "input" && value !== "");
                }}
                filterOptions={(options, { inputValue }) =>
                    options.filter((option) =>
                        option.toLowerCase().includes(inputValue.toLowerCase())
                    )
                }
                value={value}
                onChange={(_, newValue) => onChange(newValue)}
                renderTags={() => null}
                renderInput={(params) => (
                    <TextInput
                        id={id}
                        label={label}
                        type="text"
                        placeholder={placeholder}
                        textFieldProps={{
                            ...params,
                            onKeyDown: (e) => {
                                const inputValue = params.inputProps?.value;
                                if (
                                    e.key === "Enter" &&
                                    typeof inputValue === "string" &&
                                    inputValue.trim()
                                ) {
                                    e.preventDefault();
                                    const input = inputValue.trim();
                                    if (input && !value.includes(input)) {
                                        onChange([...value, input]);
                                    }
                                }
                            },
                        }}
                    />
                )}
            />
            <Stack direction="row" spacing={1.5}>
                {value.map((option, index) => (
                    <Chip key={index} label={option} color="primary" onDelete={() => {
                            const newValues = value.filter((_, i) => i !== index);
                            onChange(newValues);
                        }}
                    />
                ))}
            </Stack>
        </>
    );
}
