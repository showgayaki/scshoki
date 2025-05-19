import { invoke } from "@tauri-apps/api/core";

export const startUsbMonitor = (): Promise<string[]> =>
    invoke("start_usb_monitor");
