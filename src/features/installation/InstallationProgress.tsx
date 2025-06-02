import { useEffect } from "react";
import {
    Box,
    CircularProgress,
    Typography,
} from "@mui/material";
import CheckCircleIcon from "@mui/icons-material/CheckCircle";

import FullscreenOverlay from "@/components/FullscreenOverlay";
import TaskList from "@/components/TaskList";

import { INSTALL_TASKS } from "./constants";
import { useInstallationTasks } from "./hooks";

export default function InstallationProgress({ onComplete }: { onComplete: () => void }) {
    const {
        taskStatuses,
        currentTask,
        isInstalling,
        success,
    } = useInstallationTasks(INSTALL_TASKS);

    useEffect(() => {
        if (success && !isInstalling) {
            onComplete();
        }
    }, [success, isInstalling, onComplete]);

    return (
        <>
            {isInstalling && (
                <FullscreenOverlay>
                    <Box sx={{ mb: 4 }}>
                        {success
                            ? <CheckCircleIcon color="success" sx={{ width: 80, height: 80 }} />
                            : <CircularProgress size={80} />
                        }
                    </Box>
                    <Typography variant="h6" sx={{ mb: 2 }}>
                        {currentTask ? `${currentTask} をインストール中です...` : "インストールが完了しました！"}
                    </Typography>
                    <TaskList taskStatuses={taskStatuses} />
                </FullscreenOverlay>
            )}
        </>
    );
};
