import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";

import { useDeviceStore } from "@/stores/device";
import { SnackbarSeverity } from "@/types/snackbar";

import { startUsbMonitor } from "./api";

export function useDeviceSnackbar() {
    const autoHideDuration = 2000; // スナックバーの表示時間(ms)

    const [open, setOpen] = useState(false);
    const [message, setMessage] = useState("");
    const [severity, setSeverity] = useState<SnackbarSeverity>("info");

    useEffect(() => {
        // USBデバイスの監視をスタート
        startUsbMonitor();

        const unlistenConnected = listen<string>("device_connected", (event) => {
            setMessage(event.payload);
            setSeverity("info");
            setOpen(true);
            useDeviceStore.getState().setIsConnected(true);
        });
        const unlistenDisconnected = listen<string>("device_disconnected", (event) => {
            setMessage(event.payload);
            setSeverity("warning");
            setOpen(true);
            useDeviceStore.getState().setIsConnected(false);
        });

        return () => {
            unlistenConnected.then((f) => f());
            unlistenDisconnected.then((f) => f());
        };
    }, []);

    return {
        open,
        message,
        severity,
        autoHideDuration,
        setOpen,
    };
}
