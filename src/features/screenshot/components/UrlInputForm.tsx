import { Box } from "@mui/material";

import TextInput from "@/components/TextInput";

interface UrlInputFormProps {
    url: string;
    setUrl: (url: string) => void;
}

export default function UrlInputForm({ url, setUrl }: UrlInputFormProps) {
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
