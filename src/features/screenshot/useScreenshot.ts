import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";

import type { TaskStatus } from "@/types/taskStatus";
import type { ScreenshotParams } from "@/generated/ScreenshotParams";

import { takeScreenshot as scsho, cancelScreenshot as cancel } from "./api";

export function useScreenshot() {
    const [status, setStatus] = useState("");
    const [isTakingScreenshot, setIsTakingScreenshot] = useState(false);
    const [taskStatuses, setTaskStatuses] = useState<TaskStatus>({});

    const takeScreenshot = async (params: ScreenshotParams) => {
        console.log("screenshot params:", params);
        setIsTakingScreenshot(true);
        setStatus("スクショを開始しています...");
        setTaskStatuses(
            Object.fromEntries(params.targetPagePaths.map((path) => [path, "pending"]))
        );

        try {
            const response = await scsho(params);
            if (response.cancelled) {
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
                setIsTakingScreenshot(false);
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
                const status = payload.split(":")[0];
                const path = payload.split(":")[1];
                switch (status) {
                    case "taking":
                        setStatus("スクショ中...");
                        break;
                    case "success":
                    case "error":
                        setTaskStatuses((prev) => ({
                            ...prev,
                            [path]: status,
                        }));
                        break;
                }
            }else{
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
        taskStatuses,
        takeScreenshot,
        cancelScreenshot,
    };
}
