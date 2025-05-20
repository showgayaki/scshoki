import { Box, Button } from "@mui/material";

interface ScreenshotButtonProps {
    url: string;
    hiddenElements: string;
    selectedBrowsers: Record<string, boolean>;
    status?: string;
}

export default function ScreenshotButton({ status }: ScreenshotButtonProps) {
    return (
        <>
            <Box sx={{ display: "flex", justifyContent: "flex-end" }}>
                <Button type="submit" variant="contained" color="primary">
                    スクショ！
                </Button>
            </Box>
            {status && <p className="mt-2 text-sm">{status}</p>}
        </>
    );
}
