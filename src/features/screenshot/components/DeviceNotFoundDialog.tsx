import CustomDialog from "@/components/CustomDialog";
import { Typography } from "@mui/material";

interface DeviceNotFoundDialogProps {
    open: boolean;
    onClose: () => void;
}

export default function DeviceNotFoundDialog({ open, onClose }: DeviceNotFoundDialogProps) {
    return (
        <CustomDialog open={open} onClose={onClose} title="デバイスが見つかりません">
            <Typography>
                スクショするには、USBでデバイスを接続してください。
            </Typography>
        </CustomDialog>
    );
}
