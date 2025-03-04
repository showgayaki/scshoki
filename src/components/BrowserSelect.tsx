interface BrowserSelectProps {
    selectedBrowser: string;
    setSelectedBrowser: (browser: string) => void;
}

export default function BrowserSelect({ selectedBrowser, setSelectedBrowser }: BrowserSelectProps) {
    const browsers = ["Chrome", "Firefox", "Edge", "Safari"];

    return (
        <div className="mb-4">
            <label className="block text-sm font-medium text-gray-700">ブラウザ選択</label>
            <div className="mt-2 flex space-x-4">
                {browsers.map((browser) => (
                    <button
                        key={browser}
                        className={`p-2 border rounded-md ${selectedBrowser === browser ? "bg-blue-500 text-white" : "bg-gray-100"}`}
                        onClick={() => setSelectedBrowser(browser)}
                    >
                        {browser}
                    </button>
                ))}
            </div>
        </div>
    );
}
