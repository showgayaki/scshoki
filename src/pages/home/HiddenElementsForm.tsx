interface HiddenElementsFormProps {
    hiddenElements: string;
    setHiddenElements: (value: string) => void;
}

export default function HiddenElementsForm({ hiddenElements, setHiddenElements }: HiddenElementsFormProps) {
    return (
        <label className="block">
            非表示にする要素のセレクタ（例: `.header, .ad-banner`）:
            <input
                type="text"
                value={hiddenElements}
                onChange={(e) => setHiddenElements(e.target.value)}
                placeholder=".header, .sticky, .ads"
                className="mt-1 p-2 border rounded w-full"
            />
        </label>
    );
}
