import type { MouseEventHandler } from "react";

interface ScreenshotButtonProps {
    url: string;
    hiddenElements: string;
    selectedBrowsers: Record<string, boolean>;
    onClick: MouseEventHandler<HTMLButtonElement>;
    status?: string;
}

export default function ScreenshotButton({ onClick, status }: ScreenshotButtonProps) {
    return (
        <div>
            <button type="button" onClick={onClick} className="px-4 py-2 bg-blue-500 text-white rounded">
                スクショ！
            </button>
            {status && <p className="mt-2 text-sm">{status}</p>}
        </div>
    );
}
