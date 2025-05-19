import { invoke } from "@tauri-apps/api/core";

export const takeScreenshot = (
    url: string,
    hiddenElements: string,
    selectedBrowsers: string[]
): Promise<{ success: boolean; path: string; error?: string }> =>
    invoke("take_screenshot", { url, hiddenElements, selectedBrowsers });
