import { useState } from "react";

interface UrlInputFormProps {
    url: string;
    setUrl: (url: string) => void;
}

export default function UrlInputForm({ url, setUrl }: UrlInputFormProps) {
    const [protocol, setProtocol] = useState("https://");

    return (
        <div className="mb-4">
            <label className="block text-sm font-medium text-gray-700">URL</label>
            <div className="flex items-center border border-gray-300 rounded-md overflow-hidden">
                <select
                    value={protocol}
                    onChange={(e) => setProtocol(e.target.value)}
                    className="p-2 bg-gray-100 border-r border-gray-300"
                >
                    <option value="https://">https://</option>
                    <option value="http://">http://</option>
                </select>
                <input
                    type="text"
                    value={url}
                    onChange={(e) => setUrl(e.target.value)}
                    className="flex-1 p-2"
                    placeholder="example.com/path"
                />
            </div>
        </div>
    );
}
