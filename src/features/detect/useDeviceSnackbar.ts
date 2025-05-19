import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";

import { startUsbMonitor } from "./api";

export function useDeviceSnackbar() {
    const autoHideDuration = 2000; // スナックバーの表示時間(ms)

    const [open, setOpen] = useState(false);
    const [message, setMessage] = useState("");
    const [color, setColor] = useState<"primary" | "warning">("primary");

    useEffect(() => {
        // USBデバイスの監視をスタート
        startUsbMonitor();

        const unlistenConnected = listen<string>("device_connected", (event) => {
            setMessage(event.payload);
            setColor("primary");
            setOpen(true);
        });
        const unlistenDisconnected = listen<string>("device_disconnected", (event) => {
            setMessage(event.payload);
            setColor("warning");
            setOpen(true);
        });

        return () => {
            unlistenConnected.then((f) => f());
            unlistenDisconnected.then((f) => f());
        };
    }, []);

    return {
        open,
        message,
        color,
        autoHideDuration,
        setOpen,
    };
}
