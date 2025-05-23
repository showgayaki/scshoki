import { FormControl, InputLabel, TextField } from "@mui/material";

interface TextInputProps {
    id?: string;
    label?: string;
    value: string;
    onChange: (value: string) => void;
    placeholder?: string;
    disabled?: boolean;
    type?: string;
}

export default function TextInput({
    id,
    label,
    value,
    onChange,
    placeholder,
    disabled,
    type = "text",
}: TextInputProps) {
    return (
        <FormControl fullWidth>
            {
                label && (
                    <InputLabel sx={{ position: "relative", top: "14px", left: "-14px" }}
                        shrink htmlFor={id}>
                        {label}
                    </InputLabel>
                )
            }
            <TextField
                sx={{
                    "& .MuiInputBase-input": {
                        backgroundColor: disabled ? "#1c1c1c" : undefined,
                        color: disabled ? "#333333" : undefined,
                    },
                }}
                id={id}
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
        </FormControl>

    );
}
