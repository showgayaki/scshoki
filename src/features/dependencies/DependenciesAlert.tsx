import { Modal, Box, Typography, List, ListItem, ListItemIcon, ListItemText, Button } from "@mui/material";
import CheckCircleIcon from "@mui/icons-material/CheckCircle";
import CancelIcon from "@mui/icons-material/Cancel";
import { useTheme } from "@mui/material/styles";

import { Dependency } from "./useDependencies";

type Props = {
    open: boolean;
    dependencies: Dependency[];
    onClose: () => void;
};

export default function DependenciesAlert({ open, dependencies, onClose }: Props) {
    const theme = useTheme();

    return (
        <Modal open={open} onClose={onClose}>
            <Box
                sx={{
                    position: "absolute",
                    top: "50%",
                    left: "50%",
                    transform: "translate(-50%, -50%)",
                    bgcolor: theme.palette.mode === "dark" ? theme.palette.background.paper : theme.palette.common.black,
                    boxShadow: 24,
                    p: 4,
                    borderRadius: 2,
                    minWidth: 250,
                    outline: "none",
                }}
            >
                <Typography variant="h6" gutterBottom>iOS Dependencies Status</Typography>
                <List>
                    {dependencies.map(({ name, installed }) => (
                        <ListItem key={name}>
                            <ListItemIcon sx={{ minWidth: "40px" }}>
                                {installed ? (
                                    <CheckCircleIcon color="success" />
                                ) : (
                                    <CancelIcon color="error" />
                                )}
                            </ListItemIcon>
                            <ListItemText primary={name} />
                        </ListItem>
                    ))}
                </List>
                <Button variant="contained" color="primary" onClick={onClose}>
                    OK
                </Button>
            </Box>
        </Modal>
    );
}