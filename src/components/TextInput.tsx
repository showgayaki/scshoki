import { Box, TextField } from "@mui/material";

interface TextInputProps {
    label?: string;
    value: string;
    onChange: (value: string) => void;
    placeholder?: string;
    disabled?: boolean;
    type?: string;
}

export default function TextInput({
    label,
    value,
    onChange,
    placeholder,
    disabled,
    type = "text",
}: TextInputProps) {
    return (
        <TextField
            label={label}
            value={value}
            onChange={(e) => onChange(e.target.value)}
            placeholder={placeholder}
            disabled={disabled}
            type={type}
            variant="outlined"
            size="small"
            fullWidth
            slotProps={{
                inputLabel: {
                    shrink: true,
                },
            }}
        />
    );
}
