import { useEffect } from "react";
import { Box, CircularProgress, Typography, List, ListItem, ListItemIcon, ListItemText } from "@mui/material";
import CheckCircleIcon from "@mui/icons-material/CheckCircle";
import RadioButtonUncheckedIcon from "@mui/icons-material/RadioButtonUnchecked";

import FullscreenOverlay from "@/components/FullscreenOverlay";
import { INSTALL_TASKS } from "./constants";
import { useInstallationTasks } from "./hooks";

export default function InstallationProgress({ onComplete }: { onComplete: () => void }) {
    const {
        completedTasks,
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
                    <Typography variant="h6">
                        {currentTask ? `${currentTask} をインストール中です...` : "インストールが完了しました！"}
                    </Typography>
                    <List sx={{ mt: 2 }}>
                        {INSTALL_TASKS.map((task) => (
                            <ListItem key={task.key}>
                                <ListItemIcon sx={{ minWidth: "40px" }}>
                                    {completedTasks.includes(task.label) ? (
                                        <CheckCircleIcon color="success" />
                                    ) : (
                                        <RadioButtonUncheckedIcon color="warning" />
                                    )}
                                </ListItemIcon>
                                <ListItemText primary={task.label} />
                            </ListItem>
                        ))}
                    </List>
                </FullscreenOverlay>
            )}
        </>
    );
};
