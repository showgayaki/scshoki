import {
    Autocomplete,
    Box,
    Chip,
    Dialog,
    DialogTitle,
    DialogContent,
    Stack,
} from "@mui/material";

import TextInput from "@/components/TextInput";
import FilledPrimaryChip from "@/components/CustomChips";

import { useTagsInput } from "./useTagsInput";

interface TagsInputProps {
    id: string;
    label: string;
    placeholder?: string;
    options?: string[];
    value: string;
    onChange?: (value: string[]) => void;
    useInternalState?: boolean;
}

export default function TagsInput({
    id,
    label,
    placeholder,
    options = [],
    value,
    onChange,
    useInternalState = false,
}: TagsInputProps) {
    const valueArray = value.split(",").map((s) => s.trim()).filter(Boolean);

    const {
        currentTags,
        optionsOpen,
        setOptionsOpen,
        dialogOpen,
        setDialogOpen,
        handleChange,
    } = useTagsInput(valueArray ?? [], useInternalState, onChange);

    return (
        <>
            <Autocomplete
                sx={{
                    // Autocompleteのpaddingと配下のTextFieldのpaddingで
                    // 二重にpaddingがかかるので、Autocompleteのpaddingを0にしておく
                    "& .MuiInputBase-root": {
                        p: 0,
                    },
                }}
                multiple
                freeSolo
                options={options}
                open={optionsOpen}
                onOpen={() => {
                    // 開かない（何もしない）
                }}
                onInputChange={(_, value, reason) => {
                    // 入力されたときだけ入力候補（options）を開く（reason === "input"）
                    setOptionsOpen(reason === "input" && value !== "");
                }}
                filterOptions={(options, { inputValue }) =>
                    options.filter((option) =>
                        option.toLowerCase().includes(inputValue.toLowerCase())
                    )
                }
                value={currentTags}
                onChange={(_, newValue) => handleChange(newValue)}
                limitTags={3}
                renderTags={(value: readonly string[], getTagProps) => {
                    const displayTags = value.slice(0, 3);
                    const hiddenTagCount = value.length - displayTags.length;
                    return (
                        <>
                            {displayTags.map((option: string, index: number) => (
                                <FilledPrimaryChip
                                    label={option}
                                    {...getTagProps({ index })}
                                />
                            ))}
                            {hiddenTagCount > 0 && (
                                <Box onClick={() => setDialogOpen(true)} sx={{ cursor: "pointer" }}>
                                    <Chip
                                        variant="outlined"
                                        size="small"
                                        label={`+${hiddenTagCount}`}
                                        sx={{ fontStyle: "italic" }}
                                    />
                                </Box>
                            )}
                        </>
                    );
                }}
                renderInput={(params) => (
                    <TextInput
                        id={id}
                        label={label}
                        type="text"
                        placeholder={currentTags.length ? "" : placeholder}  // 入力された時はplaceholderを非表示
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
                                    if (input && !currentTags.includes(input)) {
                                        handleChange([...currentTags, input]);
                                    }
                                }
                            },
                        }}
                    />
                )}
            />

            <Dialog open={dialogOpen} onClose={() => setDialogOpen(false)}>
                <DialogTitle sx={{ typography: 'subtitle1', py: 1 }}>
                    {label}
                </DialogTitle>
                <DialogContent dividers>
                    <Stack direction="row" spacing={1} useFlexGap flexWrap="wrap" sx={{ rowGap: 1, columnGap: 1, ml: 0 }}>
                        {currentTags.map((tag, idx) => (
                            <FilledPrimaryChip
                                key={idx}
                                label={tag}
                            />
                        ))}
                    </Stack>
                </DialogContent>
            </Dialog>
        </>
    );
}
