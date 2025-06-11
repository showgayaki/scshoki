import { invoke } from "@tauri-apps/api/core";

import type { ScreenshotParams } from "@/generated/ScreenshotParams";
import type { ScreenshotResponse } from "@/generated/ScreenshotResponse";

export const checkDeviceConnected = (): Promise<boolean> =>
    invoke("check_device_connected");

export const takeScreenshot = (params: ScreenshotParams): Promise<ScreenshotResponse> =>
    invoke("take_screenshot", { params });

export const cancelScreenshot = (): Promise<{ success: boolean }> =>
    invoke("cancel_screenshot");
