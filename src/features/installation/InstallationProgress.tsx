import { Box, CircularProgress, Typography, List, ListItem, ListItemIcon, ListItemText } from "@mui/material";
import CheckCircleIcon from "@mui/icons-material/CheckCircle";
import RadioButtonUncheckedIcon from "@mui/icons-material/RadioButtonUnchecked";

import DependenciesAlert from "../../components/DependenciesAlert";
import { useInstallationTasks } from "./hooks";
import { useEffect } from "react";
import { INSTALL_TASKS } from "./constants";

export const InstallationProgress = ({ onComplete }: { onComplete: () => void }) => {
    const {
        completedTasks,
        currentTask,
        isInstalling,
        success,
        showDependenciesAlert,
    } = useInstallationTasks(INSTALL_TASKS);

    useEffect(() => {
        if (success && !isInstalling) {
            onComplete();
        }
    }, [success, isInstalling, onComplete]);

    return (
        <>
            {isInstalling && (
                <Box
                    sx={{
                        position: "fixed",
                        top: 0,
                        left: 0,
                        width: "100%",
                        height: "100%",
                        backgroundColor: "rgba(0, 0, 0, 0.5)",
                        display: "flex",
                        justifyContent: "center",
                        alignItems: "center",
                        flexDirection: "column",
                        zIndex: 1000,
                    }}
                >
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
                </Box>
            )}
            {showDependenciesAlert && <DependenciesAlert />}
        </>
    );
};
