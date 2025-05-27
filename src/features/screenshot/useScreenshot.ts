import { useState } from "react";

import type { ScreenshotParams } from "@/generated/ScreenshotParams";
import { takeScreenshot as scsho, cancelScreenshot as cancel } from "./api";

export function useScreenshot() {
    const [status, setStatus] = useState("");
    const [isCapturing, setIsCapturing] = useState(false);

    const takeScreenshot = async (params: ScreenshotParams) => {
        console.log("screenshot params:", params);
        setStatus("スクリーンショット取得中...");
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

    const cancelScreenshot = async () => {
        setStatus("スクリーンショットをキャンセルしています...");
        const result = await cancel();
        console.log("cancel result:", result);

        if (result.success) {
            setStatus("スクリーンショットをキャンセルしました");
            // 少し待ってから isCapturing を false にする
            setTimeout(() => {
                setIsCapturing(false);
            }, 1000);
        }
    };

    return { status, isCapturing, takeScreenshot, cancelScreenshot };
}
