import { Box } from "@mui/material";

import TextInput from "@/components/TextInput";

interface BaseUrlInputProps {
    url: string;
    setUrl: (url: string) => void;
}

export default function BaseUrlInput({ url, setUrl }: BaseUrlInputProps) {
    return (
        <Box>
            <TextInput
                id="urlInput"
                label="URL"
                type="text"
                value={url}
                onChange={(e) => setUrl(e)}
                placeholder="https://example.com/path"
            />
        </Box>
    );
}
