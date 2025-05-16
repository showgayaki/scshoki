import TextInput from "@/components/TextInput";

interface UrlInputFormProps {
    url: string;
    setUrl: (url: string) => void;
}

export default function UrlInputForm({ url, setUrl }: UrlInputFormProps) {
    return (
        <div className="mb-4">
            <TextInput
                label="URL"
                type="text"
                value={url}
                onChange={(e) => setUrl(e)}
                placeholder="example.com/path"
            />
        </div>
    );
}
