import { invoke } from "@tauri-apps/api/core";

export const startUsbMonitor = (): Promise<string[]> =>
    invoke("start_usb_monitor");

export const initDeviveInfo = (): Promise<string[]> =>
    invoke("init_devive_info");
