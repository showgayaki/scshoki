import { invoke } from "@tauri-apps/api/core";
import type { ScreenshotParams } from "@/generated/ScreenshotParams";

export const takeScreenshot = (params: ScreenshotParams):
    Promise<{ success: boolean; path: string; error?: string }> => {
    return invoke("take_screenshot", {params});
};
