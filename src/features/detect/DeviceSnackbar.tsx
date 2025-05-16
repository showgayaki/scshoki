import { invoke } from "@tauri-apps/api/core";

import AppSnackbar from "@/components/AppSnackbar";
import { useDeviceSnackbar } from "./useDeviceSnackbar";

export function DeviceSnackbar() {
    const { open, setOpen, message, color, autoHideDuration } = useDeviceSnackbar();

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
