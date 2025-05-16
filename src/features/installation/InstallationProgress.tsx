import { useEffect } from "react";
import { CircularProgress, Typography, List, ListItem, ListItemIcon, ListItemText } from "@mui/material";
import CheckCircleIcon from "@mui/icons-material/CheckCircle";
import RadioButtonUncheckedIcon from "@mui/icons-material/RadioButtonUnchecked";

import { INSTALL_TASKS } from "./constants";
import { useInstallationTasks } from "./hooks";
import { FullscreenOverlay } from "../../components/FullscreenOverlay";

export const InstallationProgress = ({ onComplete }: { onComplete: () => void }) => {
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
                    {success ? <CheckCircleIcon color="success" sx={{ width: 80, height: 80 }} /> : <CircularProgress size={80} />}
                    <Typography variant="h6" sx={{ mt: 2 }}>
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
