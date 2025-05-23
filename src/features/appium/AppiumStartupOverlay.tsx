import { CircularProgress, Typography } from "@mui/material";
import FullscreenOverlay from "@/components/FullscreenOverlay";

export default function AppiumStartupOverlay() {
    return (
        <FullscreenOverlay>
            <CircularProgress size={80} />
            <Typography variant="h6" sx={{ mt: 2 }}>
                Appiumを起動しています…
            </Typography>
        </FullscreenOverlay>
    );
}
