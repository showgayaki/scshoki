import Snackbar from "@mui/joy/Snackbar";
import { IconButton } from "@mui/joy";
import CloseIcon from "@mui/icons-material/Close";

interface AppSnackbarProps {
    open: boolean;
    onClose: () => void;
    message: string;
    autoHideDuration?: number;
    color?: "primary" | "neutral" | "danger" | "success" | "warning";
    onTransitionEnd?: () => void;
}

const AppSnackbar: React.FC<AppSnackbarProps> = ({
    open, onClose, message, autoHideDuration, color = 'neutral', onTransitionEnd
}) => {
    return (
        <Snackbar
            open={open}
            onClose={onClose}
            autoHideDuration={autoHideDuration}
            variant="soft"
            color={color}
            anchorOrigin={{ vertical: "top", horizontal: "center" }}
            endDecorator={
                <IconButton
                    size="sm"
                    aria-label="close"
                    variant="plain"
                    color={color}
                    onClick={onClose}
                >
                    <CloseIcon fontSize="small" />
                </IconButton>
            }
            onTransitionEnd={onTransitionEnd}
        >
            {message}
        </Snackbar>
    );
};

export default AppSnackbar;
