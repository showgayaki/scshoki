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
            if (response.cancelled){
                setStatus("スクリーンショットをキャンセルしました");
                return;
            }
            if (response.success) {
                setStatus(`スクリーンショットを保存しました: ${response.path}`);
            } else {
                setStatus(`エラー: ${response.error}`);
            }
        } catch (error) {
            console.error("スクリーンショット取得中にエラーが発生しました:", error);
            setStatus(`エラー: ${error}`);
        } finally {
            setTimeout(() => {
                setIsCapturing(false);
            }, 3000);
        }
    };

    const cancelScreenshot = async () => {
        setStatus("スクリーンショットをキャンセルしています...");
        await cancel();
    };

    return { status, isCapturing, takeScreenshot, cancelScreenshot };
}
