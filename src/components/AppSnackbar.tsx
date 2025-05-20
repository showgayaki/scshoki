import { Snackbar, Alert, IconButton } from "@mui/material";
import CloseIcon from "@mui/icons-material/Close";

import { SnackbarSeverity } from "@/types/snackbar";

interface AppSnackbarProps {
    open: boolean;
    onClose: () => void;
    message: string;
    autoHideDuration?: number;
    severity?: SnackbarSeverity;
    onTransitionEnd?: () => void;
}

const AppSnackbar: React.FC<AppSnackbarProps> = ({
    open,
    onClose,
    message,
    autoHideDuration,
    severity = "info",
    onTransitionEnd,
}) => {
    return (
        <Snackbar
            open={open}
            onClose={onClose}
            autoHideDuration={autoHideDuration}
            anchorOrigin={{ vertical: "top", horizontal: "center" }}
            onTransitionEnd={onTransitionEnd}
        >
            <Alert
                severity={severity}
                action={
                    <IconButton size="small" aria-label="close" color="inherit" onClick={onClose}>
                        <CloseIcon fontSize="small" />
                    </IconButton>
                }
            >
                {message}
            </Alert>
        </Snackbar>
    );
};

export default AppSnackbar;
