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
                type="text"
                value={hiddenElements}
                onChange={(e) => setHiddenElements(e)}
                placeholder="header, .sticky, .ads"
                label="スクロール中に非表示にする要素のセレクタ"
            />
        </Box>
    );
}
