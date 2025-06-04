import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";

import type { GroupedTaskStatuses } from "@/types/taskStatuses";
import type { ScreenshotParams } from "@/generated/ScreenshotParams";

import { takeScreenshot as scsho, cancelScreenshot as cancel } from "./api";

export function useScreenshot() {
    const [status, setStatus] = useState("");
    const [isTakingScreenshot, setIsTakingScreenshot] = useState(false);
    const [groupedTaskStatuses, setGroupedTaskStatuses] = useState<GroupedTaskStatuses>({});
    const [success, setSuccess] = useState(false);

    const takeScreenshot = async (params: ScreenshotParams) => {
        console.log("screenshot params:", params);
        setIsTakingScreenshot(true);
        setStatus("スクショを開始しています...");
        setGroupedTaskStatuses(
            Object.fromEntries(
                params.selectedBrowsers.map(browser => [
                    browser,
                    Object.fromEntries(params.targetPagePaths.map((path) => [path, "pending"]))
                ])
            )
        );

        try {
            const response = await scsho(params);
            if (response.cancelled) {
                setStatus("スクリーンショットをキャンセルしました");
                return;
            }
            if (response.success) {
                setStatus(`スクリーンショットを保存しました: ${response.path}`);
                setSuccess(true);
            } else {
                setStatus(`エラー: ${response.error}`);
            }
        } catch (error) {
            console.error("スクリーンショット取得中にエラーが発生しました:", error);
            setStatus(`エラー: ${error}`);
        } finally {
            setTimeout(() => {
                setIsTakingScreenshot(false);
                setSuccess(false);
            }, 3000);
        }
    };

    const cancelScreenshot = async () => {
        setStatus("スクリーンショットをキャンセルしています...");
        await cancel();
    };

    useEffect(() => {
        const unlisten = listen<string>("screenshot_status", (event) => {
            const payload = event.payload;
            if (payload.includes(":")) {
                const [browser, path, status] = payload.split(":");
                switch (status) {
                    case "taking":
                        setStatus("スクショ中...");
                        break;
                    case "success":
                    case "error":
                        setGroupedTaskStatuses((prev) => ({
                            ...prev,
                            [browser]: {
                                ...prev[browser],
                                [path]: status,
                            },
                        }));
                        break;
                }
            } else {
                setStatus(payload);
            }
        });

        return () => {
            unlisten.then((f) => f());
        };
    }, []);

    return {
        status,
        isTakingScreenshot,
        groupedTaskStatuses,
        success,
        takeScreenshot,
        cancelScreenshot,
    };
}
