import {
    Box,
    IconButton,
    Typography,
} from "@mui/material";

import FullscreenCircularProgress from "@/components/FullscreenCircularProgress";
import TaskList from "@/components/TaskList";
import type { TaskStatuses } from "@/types/taskStatuses";

interface ScreenshotProgressProps {
    status: string;
    selectedBrowsers: string[];
    taskStatuses: TaskStatuses;
    success: boolean;
    handleCancel: () => void;
}

export default function ScreenshotProgress({ status, selectedBrowsers, taskStatuses, success, handleCancel }: ScreenshotProgressProps) {
    return (
        <FullscreenCircularProgress success={success}>
            <Typography
                variant="h6"
                sx={{ textAlign: "center", mb: 3 }}
                className="whitespace-pre-line"
            >
                {status}
            </Typography>
            <TaskList groups={selectedBrowsers} taskStatuses={taskStatuses} />
            <CancelButton onClick={handleCancel} />
        </FullscreenCircularProgress>
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
