import { Box, Tooltip } from "@mui/material";

import TextInput from "@/components/TextInput";

interface BaseUrlInputProps {
    url: string;
    setUrl: (url: string) => void;
    error?: string | null;
}

export default function BaseUrlInput({ url, setUrl, error }: BaseUrlInputProps) {
    return (
        <Tooltip title={error || ""} open={Boolean(error)} placement="bottom" arrow>
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
        </Tooltip>
    );
}
