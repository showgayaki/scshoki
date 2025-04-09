import { useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Modal, Box, Typography, List, ListItem, ListItemIcon, ListItemText, Button } from "@mui/material";
import CheckCircleIcon from "@mui/icons-material/CheckCircle";
import CancelIcon from "@mui/icons-material/Cancel";
import { useTheme } from "@mui/material/styles";

export default function DependenciesAlert() {
    const [dependencies, setDependencies] = useState<{ name: string; installed: boolean }[]>([]);
    const [open, setOpen] = useState(false);
    const isExecuted = useRef(false);
    const theme = useTheme(); // テーマ取得

    // 依存関係の状態を取得する関数
    const checkDependencies = async () => {
        try {
            const installed = await invoke<{ [key: string]: boolean }[]>("is_ios_dependencies_installed");
            console.log("Dependencies:", installed);

            const formattedList = installed.map(obj => {
                const [name, installed] = Object.entries(obj)[0];
                return { name, installed };
            });

            setDependencies(formattedList);

            // すべてインストール済みならモーダルを開かない
            setOpen(!formattedList.every(dep => dep.installed));
        } catch (err) {
            console.error("invoke error:", err);
        }
    };

    // 初回レンダリング時にデータ取得
    if (!isExecuted.current) {
        checkDependencies();
        isExecuted.current = true;
    };

    return (
        <Modal open={open} onClose={() => setOpen(false)}>
            <Box
                sx={{
                    position: "absolute",
                    top: "50%",
                    left: "50%",
                    transform: "translate(-50%, -50%)",
                    bgcolor: theme.palette.mode === "dark" ? theme.palette.background.paper : theme.palette.common.black,
                    boxShadow: 24,
                    p: 4,
                    borderRadius: 2,
                    minWidth: 250,
                    outline: "none",
                }}
            >
                <Typography variant="h6" gutterBottom>iOS Dependencies Status</Typography>
                <List>
                    {dependencies.map(({ name, installed }) => (
                        <ListItem key={name}>
                            <ListItemIcon sx={{ minWidth: "40px" }}>
                                {installed ? (
                                    <CheckCircleIcon color="success" />
                                ) : (
                                    <CancelIcon color="error" />
                                )}
                            </ListItemIcon>
                            <ListItemText primary={name} />
                        </ListItem>
                    ))}
                </List>
                <Button variant="contained" color="primary" onClick={() => setOpen(false)}>
                    OK
                </Button>
            </Box>
        </Modal>
    );
}