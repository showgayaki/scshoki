import { Box } from "@mui/material";
import TextInput from "@/components/TextInput";

interface HiddenElementsFormProps {
    hiddenElements: string;
    setHiddenElements: (value: string) => void;
}

export default function HiddenElementsForm({ hiddenElements, setHiddenElements }: HiddenElementsFormProps) {
    return (
        <Box>
            <TextInput
                id="hiddenElementsInput"
                label="スクロール中に非表示にする要素のセレクタ"
                type="text"
                value={hiddenElements}
                onChange={(e) => setHiddenElements(e)}
                placeholder="header, .sticky, #ads"
            />
        </Box>
    );
}
