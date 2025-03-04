import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface ScreenshotButtonProps {
    url: string;
    hiddenElements: string; // 非表示にする要素のセレクタを追加
}

export default function ScreenshotButton({ url, hiddenElements }: ScreenshotButtonProps) {
    const [status, setStatus] = useState<string | null>(null);

    const handleScreenshot = async () => {
        if (!url) {
            setStatus("URLを入力してください");
            return;
        }

        setStatus("スクリーンショットを取得中...");

        try {
            const response = await invoke<{ success: boolean; path: string; error?: string }>(
                "take_screenshot",
                { url, hiddenElements }
            );

            if (response.success) {
                setStatus(`スクリーンショットを保存しました: ${response.path}`);
            } else {
                setStatus(`エラー: ${response.error}`);
            }
        } catch (error) {
            setStatus(`エラー: ${error}`);
        }
    };

    return (
        <div>
            <button onClick={handleScreenshot} className="px-4 py-2 bg-blue-500 text-white rounded">
                スクリーンショットを撮る
            </button>
            {status && <p className="mt-2 text-sm">{status}</p>}
        </div>
    );
}
