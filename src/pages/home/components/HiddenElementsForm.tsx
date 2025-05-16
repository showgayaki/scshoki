interface HiddenElementsFormProps {
    hiddenElements: string;
    setHiddenElements: (value: string) => void;
}

import TextInput from "@/components/TextInput";

export default function HiddenElementsForm({ hiddenElements, setHiddenElements }: HiddenElementsFormProps) {
    return (
        <label className="block">
            非表示にする要素のセレクタ（例: `.header, .ad-banner`）:
            <TextInput
                type="text"
                value={hiddenElements}
                onChange={(e) => setHiddenElements(e)}
                placeholder=".header, .sticky, .ads"
                className="mt-1 w-full"
            />
        </label>
    );
}
