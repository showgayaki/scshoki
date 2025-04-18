import { invoke } from "@tauri-apps/api/core";
import { Snackbar, IconButton } from "@mui/joy";
import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import CloseIcon from "@mui/icons-material/Close";

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
        <Snackbar
            open={open}
            onClose={() => setOpen(false)}
            autoHideDuration={autoHideDuration}
            variant="soft"
            color={color}
            anchorOrigin={{ vertical: "top", horizontal: "center" }} // トップのセンターに配置
            endDecorator={
                <IconButton onClick={() => setOpen(false)} size="sm" variant="plain" color={color}>
                    <CloseIcon />
                </IconButton>
            }
            onTransitionEnd={() => {
                if (color === "warning") {
                    invoke("toast_shown_ack");
                }
            }}
        >
            {message}
        </Snackbar>
    );
}