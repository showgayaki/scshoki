import { Box, CircularProgress, Typography } from "@mui/material";

export default function AppiumStartupOverlay() {
    return (
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
                zIndex: 1300,
            }}
        >
            <CircularProgress size={80} />
            <Typography variant="h6" sx={{ mt: 2 }}>
                Appiumを起動しています…
            </Typography>
        </Box>
    );
}
