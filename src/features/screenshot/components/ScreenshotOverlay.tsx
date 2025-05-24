import { CircularProgress, Typography } from "@mui/material";
import FullscreenOverlay from "@/components/FullscreenOverlay";

export default function ScreenshotOverlay() {
    return (
        <FullscreenOverlay>
            <CircularProgress size={80} />
            <Typography variant="h6" sx={{ mt: 2 }}>
                スクリーンショットを取得中…
            </Typography>
        </FullscreenOverlay>
    );
}
