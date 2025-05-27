import { useState } from "react";

import type { ScreenshotParams } from "@/generated/ScreenshotParams";
import { takeScreenshot as scsho } from "./api";

export function useScreenshot() {
    const [status, setStatus] = useState<string | undefined>(undefined);
    const [isCapturing, setIsCapturing] = useState(false);

    const takeScreenshot = async (params: ScreenshotParams) => {
        console.log("screenshot params:", params);
        setStatus("スクリーンショットを取得中...");
        setIsCapturing(true);

        try {
            const response = await scsho(params);
            if (response.success) {
                setStatus(`スクリーンショットを保存しました: ${response.path}`);
            } else {
                setStatus(`エラー: ${response.error}`);
            }
        } catch (error) {
            console.error("スクリーンショット取得中にエラー:", error);
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
