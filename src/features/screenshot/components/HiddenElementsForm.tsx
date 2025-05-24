import { Box } from "@mui/material";

import TagsInput from "@/components/TagsInput";

interface HiddenElementsFormProps {
    hiddenElements: string;
    setHiddenElements: (value: string) => void;
}

export default function HiddenElementsForm({ hiddenElements, setHiddenElements }: HiddenElementsFormProps) {
    const options = [
        "header",
        ".header",
        "#header",
        "footer",
        ".footer",
        "#footer",
        ".sticky",
        ".fixed",
    ]

    return (
        <Box>
            <TagsInput
                id="tagsInput"
                label="スクロール中に非表示にする要素のセレクタ"
                placeholder="header, .sticky, #footer"
                options={options}
                value={hiddenElements}
                onChange={(newValues) => setHiddenElements(newValues.join(", "))}
            />
        </Box>
    );
}
