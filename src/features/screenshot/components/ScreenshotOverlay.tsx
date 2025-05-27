import { CircularProgress, IconButton, Typography, Box } from "@mui/material";

import FullscreenOverlay from "@/components/FullscreenOverlay";

interface ScreenshotOverlayProps {
    handleCancel: () => void;
}

export default function ScreenshotOverlay({ handleCancel }: ScreenshotOverlayProps) {
    return (
        <FullscreenOverlay>
            <CircularProgress size={80} />
            <Typography variant="h6" sx={{ mt: 2 }}>
                スクリーンショットを取得中…
            </Typography>
            <Box
                sx={{
                    position: "absolute",
                    bottom: 180,
                    display: "flex",
                    justifyContent: "center",
                    width: "100%",
                }}
            >
                <IconButton
                    onClick={handleCancel}
                    color="error"
                    sx={{
                        px: 4,
                        py: 1,
                        backgroundColor: "white",
                        border: "1px solid #ccc",
                        borderRadius: "8px",
                        "&:hover": {
                            backgroundColor: "#fdd",
                        },
                    }}
                >
                    <Typography variant="button" sx={{ fontWeight: "bold" }}>
                        Cancel
                    </Typography>
                </IconButton>
            </Box>
        </FullscreenOverlay>
    );
}
