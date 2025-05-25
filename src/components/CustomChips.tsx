import { Chip, ChipProps } from "@mui/material";

export default function FilledPrimaryChip(props: ChipProps) {
    return (
        <Chip
            variant="filled"
            color="primary"
            size="small"
            {...props}
        />
    );
}
