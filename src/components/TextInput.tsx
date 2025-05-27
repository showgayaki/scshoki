import { FormControl, InputLabel, TextField, TextFieldProps, InputAdornment, IconButton } from "@mui/material";
import ClearIcon from "@mui/icons-material/Clear";

interface TextInputProps {
    id?: string;
    label?: string;
    value?: string;
    onChange?: (value: string) => void;
    placeholder?: string;
    disabled?: boolean;
    type?: string;
    showClearButton?: boolean;
    textFieldProps?: Partial<TextFieldProps>;
}

export default function TextInput({
    id,
    label,
    value,
    onChange,
    placeholder,
    disabled,
    type,
    showClearButton = true,
    textFieldProps,
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
                    "& .MuiInputBase-input, & .MuiInputBase-root .MuiInputBase-input.MuiAutocomplete-input": {
                        backgroundColor: disabled ? "#1c1c1c" : undefined,
                        color: disabled ? "#333333" : undefined,
                        padding: "8.5px 14px",
                    },
                }}
                id={id ?? id}
                value={value}
                onChange={(e) => onChange?.(e.target.value)}
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
                    ...(showClearButton && value
                        ? {
                            input: {
                                endAdornment: (
                                    <InputAdornment position="end" sx={{ marginRight: "-6px" }}>
                                        <IconButton
                                            onClick={() => onChange?.("")}
                                            edge="end"
                                            size="small"
                                        >
                                            <ClearIcon fontSize="small" />
                                        </IconButton>
                                    </InputAdornment>
                                ),
                            },
                        }
                        : {}),
                }}
                {...textFieldProps}
            />
        </FormControl>
    );
}
