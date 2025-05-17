import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export function useScreenshot() {
    const [status, setStatus] = useState<string | undefined>(undefined);

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

        const selectedBrowsersArray = Object.keys(selectedBrowsers).filter((browser) => selectedBrowsers[browser]);

        try {
            const response = await invoke<{ success: boolean; path: string; error?: string }>(
                "take_screenshot",
                { url, hiddenElements, selectedBrowsers: selectedBrowsersArray }
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

    return { status, takeScreenshot };
}
