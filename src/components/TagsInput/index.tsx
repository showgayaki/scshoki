import {
    Autocomplete,
    Box,
    Chip,
    Stack,
} from "@mui/material";

import CustomDialog from "@/components/CustomDialog";
import TextInput from "@/components/TextInput";
import FilledPrimaryChip from "@/components/CustomChips";

import { useTagsInput } from "./useTagsInput";

interface TagsInputProps {
    id: string;
    label: string;
    placeholder?: string;
    options?: string[];
    value: string[];
    onChange: (value: string[]) => void;
}

export default function TagsInput({
    id,
    label,
    placeholder,
    options = [],
    value,
    onChange,
}: TagsInputProps) {
    const {
        tags,
        optionsOpen,
        setOptionsOpen,
        dialogOpen,
        setDialogOpen,
        handleChange,
        handleDelete,
        handleKeyDown,
    } = useTagsInput(value, id, onChange);

    return (
        <>
            <Autocomplete
                sx={{
                    // Autocompleteのpaddingと配下のTextFieldのpaddingで
                    // 二重にpaddingがかかるので、Autocompleteのpaddingを0にしておく
                    "& .MuiInputBase-root": {
                        p: 0,
                        // pl: 1, // 左側のpaddingは残す
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
                value={tags}
                onChange={(_, newValue) => {
                    console.log("newValue:", newValue);
                    const filteredValue =
                        id === "targetPages" && newValue.length === 0 && tags.length > 0
                            ? [tags[0]]
                            : newValue;
                    handleChange(filteredValue);
                }}
                limitTags={3}
                disableClearable={id === "targetPages" && tags.length === 1}
                renderTags={(value: readonly string[], getTagProps) => {
                    const displayTags = value.slice(0, 3);
                    const hiddenTagCount = value.length - displayTags.length;
                    return (
                        <>
                            {displayTags.map((tag, index) => {
                                const { key, ...rest } = getTagProps({ index });
                                return (
                                    <FilledPrimaryChip
                                        key={key}
                                        {...rest}
                                        label={tag}
                                        sx={{
                                            "&.MuiChip-root": {
                                                ml: index == 0 ? 1.5 : 0.5,
                                            }
                                        }}
                                        onDelete={
                                            id === "targetPages" && tags.length === 1
                                                ? undefined
                                                : () => handleDelete(tag)
                                        }
                                    />
                                );
                            })}
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
                        placeholder={tags.length ? "" : placeholder}  // 入力された時はplaceholderを非表示
                        textFieldProps={{
                            ...params,
                            onKeyDown: handleKeyDown,
                        }}
                        // Autocompleteのクリアボタンはデフォルトで表示されるので不要
                        showClearButton={false}
                    />
                )}
            />

            <CustomDialog open={dialogOpen} onClose={() => setDialogOpen(false)} title={label}>
                <Stack
                    direction="row"
                    spacing={1}
                    useFlexGap
                    flexWrap="wrap"
                    sx={{ rowGap: 1, columnGap: 1, ml: 0 }}
                >
                    {tags.map((tag, index) => (
                        <FilledPrimaryChip
                            key={index}
                            label={tag}
                            onDelete={
                                id === "targetPages" && tags.length === 1
                                    ? undefined
                                    : () => handleDelete(tag)
                            }
                        />
                    ))}
                </Stack>
            </CustomDialog>
        </>
    );
}
