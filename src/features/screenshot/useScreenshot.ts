import { useState } from "react";

import { takeScreenshot as scsho } from "./api";

export function useScreenshot() {
    const [status, setStatus] = useState<string | undefined>(undefined);
    const [isCapturing, setIsCapturing] = useState(false);

    const takeScreenshot = async ({
        url,
        hiddenElements,
        selectedBrowsers,
    }: {
        url: string;
        hiddenElements: string;
        selectedBrowsers: Record<string, boolean>;
    }) => {
        if (!url) {
            setStatus("URLを入力してください");
            return;
        }

        setStatus("スクリーンショットを取得中...");
        setIsCapturing(true);

        const selectedBrowsersArray = Object.keys(selectedBrowsers).filter(
            (browser) => selectedBrowsers[browser]
        );

        try {
            const response = await scsho(url, hiddenElements, selectedBrowsersArray);
            if (response.success) {
                setStatus(`スクリーンショットを保存しました: ${response.path}`);
            } else {
                setStatus(`エラー: ${response.error}`);
            }
        } catch (error) {
            setStatus(`エラー: ${error}`);
        } finally {
            setIsCapturing(false);
        }
    };

    const cancelCapture = () => {
        // ここでは単にフラグを落とすだけ。必要ならinvokeキャンセルなど追加検討
        setStatus("スクリーンショット取得を中止しました");
        setIsCapturing(false);
    };

    return { status, isCapturing, takeScreenshot, cancelCapture };
}
