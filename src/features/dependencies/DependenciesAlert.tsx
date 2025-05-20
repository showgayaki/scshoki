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
        <Modal open={open} onClose={onClose} sx={{ display: "flex", alignItems: "center", justifyContent: "center" }}>
            <Box
                sx={{
                    bgcolor: theme.palette.background.paper,
                    boxShadow: 24,
                    p: 4,
                    borderRadius: 2,
                    outline: "none",
                }}
            >
                <Typography variant="h6" gutterBottom>iOS Dependencies Status</Typography>
                <List sx={{ py: 0, mb: 2 }}>
                    {dependencies.map(({ name, installed }) => (
                        <ListItem key={name} sx={{ px: 0 }}>
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
                <Box sx={{ display: "flex", justifyContent: "flex-end" }}>
                    <Button variant="contained" color="primary" onClick={onClose}>
                        OK
                    </Button>
                </Box>
            </Box>
        </Modal>
    );
}
