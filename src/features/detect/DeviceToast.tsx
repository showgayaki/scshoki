import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import AppSnackbar from "@/components/AppSnackbar";

export function DeviceToast() {
    const autoHideDuration = 2000; // トーストの表示時間(ms)

    const [open, setOpen] = useState(false);
    const [message, setMessage] = useState("");
    const [color, setColor] = useState<"primary" | "warning">("primary");

    useEffect(() => {
        // USBデバイスの監視をスタート
        invoke("start_usb_monitor");

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

    return (
        <AppSnackbar
            open={open}
            onClose={() => setOpen(false)}
            message={message}
            color={color}
            autoHideDuration={autoHideDuration}
            onTransitionEnd={() => {
                if (color === "warning") {
                    invoke("init_devive_info");
                }
            }}
        />
    );
}
