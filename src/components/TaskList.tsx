import {
    List,
    ListItem,
    ListItemIcon,
    ListItemText,
} from "@mui/material";
import CheckCircleIcon from "@mui/icons-material/CheckCircle";
import RadioButtonUncheckedIcon from "@mui/icons-material/RadioButtonUnchecked";
import CancelIcon from "@mui/icons-material/Cancel";

import type { TaskStatuses } from "@/types/taskStatuses";

interface TaskListProps {
    taskStatuses: TaskStatuses;
}

export default function TaskList({ taskStatuses }: TaskListProps) {
    return (
        <List>
            {Object.entries(taskStatuses).map(([path, status]) => {
                return (
                    <ListItem key={path}>
                        <ListItemIcon sx={{ minWidth: "40px" }}>
                            {status === "success" ? (
                                <CheckCircleIcon color="success" />
                            ) : status === "error" ? (
                                <CancelIcon color="error" />
                            ) : (
                                <RadioButtonUncheckedIcon color="warning" />
                            )}
                        </ListItemIcon>
                        <ListItemText primary={path} />
                    </ListItem>
                );
            })}
        </List>
    );
}
