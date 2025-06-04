import { Typography } from "@mui/material";
import FullscreenCircularProgress from "@/components/FullscreenCircularProgress";

export default function AppiumStartupProgress() {
    return (
        <FullscreenCircularProgress>
            <Typography variant="h6">
                Appiumを起動しています…
            </Typography>
        </FullscreenCircularProgress>
    );
}
