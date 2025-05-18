interface ScreenshotButtonProps {
    url: string;
    hiddenElements: string;
    selectedBrowsers: Record<string, boolean>;
    status?: string;
}

export default function ScreenshotButton({ status }: ScreenshotButtonProps) {
    return (
        <div>
            <button className="px-4 py-2 bg-blue-500 text-white rounded">
                スクショ！
            </button>
            {status && <p className="mt-2 text-sm">{status}</p>}
        </div>
    );
}
