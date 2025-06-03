import {
    Box,
    CircularProgress,
    IconButton,
    Typography,
} from "@mui/material";

import FullscreenOverlay from "@/components/FullscreenOverlay";
import TaskList from "@/components/TaskList";
import type { TaskStatuses } from "@/types/taskStatuses";

interface ScreenshotProgressProps {
    status: string;
    selectedBrowsers: string[];
    taskStatuses: TaskStatuses;
    handleCancel: () => void;
}

export default function ScreenshotProgress({ status, selectedBrowsers, taskStatuses, handleCancel }: ScreenshotProgressProps) {
    return (
        <FullscreenOverlay>
            <CircularProgress size={80} />
            <Typography
                variant="h6"
                sx={{ mt: 2, textAlign: "center" }}
                className="whitespace-pre-line"
            >
                {status}
            </Typography>
            <TaskList groups={selectedBrowsers} taskStatuses={taskStatuses} />
            <CancelButton onClick={handleCancel} />
        </FullscreenOverlay>
    );
}

function CancelButton({ onClick }: { onClick: () => void }) {
    return (
        <Box
            sx={{
                display: "flex",
                justifyContent: "center",
                width: "100%",
            }}
        >
            <IconButton
                onClick={onClick}
                color="error"
                sx={{
                    px: 4,
                    py: 1,
                    backgroundColor: "white",
                    border: "1px solid #ccc",
                    borderRadius: "8px",
                    "&:hover": {
                        backgroundColor: "#fdd",
                    },
                }}
            >
                <Typography variant="button" sx={{ fontWeight: "bold" }}>
                    Cancel
                </Typography>
            </IconButton>
        </Box>
    );
}
