import { Box } from "@mui/material";

import TextInput from "@/components/TextInput";
import BottomArrowTooltip from "@/components/CustomTooltips";

interface BaseUrlInputProps {
    url: string;
    setUrl: (url: string) => void;
    error?: string | null;
}

export default function BaseUrlInput({ url, setUrl, error }: BaseUrlInputProps) {
    return (
        <BottomArrowTooltip title={error || ""}>
            <Box sx={{ display: "inline-block", width: "100%" }}>
                <TextInput
                    id="urlInput"
                    label="URL"
                    type="text"
                    value={url}
                    onChange={setUrl}
                    placeholder="https://example.com/"
                    textFieldProps={{
                        error: Boolean(error),
                    }}
                />
            </Box>
        </BottomArrowTooltip>
    );
}
