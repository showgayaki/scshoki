import AppSnackbar from "@/components/AppSnackbar";
import { useDeviceSnackbar } from "./useDeviceSnackbar";
import { initDeviveInfo } from "./api";

export function DeviceSnackbar() {
    const { open, setOpen, message, severity, autoHideDuration } = useDeviceSnackbar();

    return (
        <AppSnackbar
            open={open}
            onClose={() => setOpen(false)}
            message={message}
            severity={severity}
            autoHideDuration={autoHideDuration}
            onTransitionEnd={() => {
                // デバイス切断(warning)スナックバーが閉じた後にデバイス情報を初期化
                if (severity === "warning") {
                    initDeviveInfo();
                }
            }}
        />
    );
}
