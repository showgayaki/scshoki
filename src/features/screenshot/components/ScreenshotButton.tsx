import { Box, Button } from "@mui/material";

export default function ScreenshotButton() {
    return (
        <>
            <Box sx={{ display: "flex", justifyContent: "flex-end" }}>
                <Button type="submit" variant="contained" color="info">
                    スクショ！
                </Button>
            </Box>
        </>
    );
}
