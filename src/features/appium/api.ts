import { invoke } from "@tauri-apps/api/core";

export const startAppium = (): Promise<void> =>
    invoke("start_appium");
