import { invoke } from "@tauri-apps/api/core";
import type { ScreenshotParams } from "@/generated/ScreenshotParams";

export const takeScreenshot = (params: ScreenshotParams):
    Promise<{ success: boolean; path: string; error?: string }> =>
    invoke("take_screenshot", {params});

export const cancelScreenshot = (): Promise<{ success: boolean }> =>
    invoke("cancel_screenshot");
