import { Box } from "@mui/material";

export const FullscreenOverlay = ({ children }: { children: React.ReactNode }) => (
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
        {children}
    </Box>
);
