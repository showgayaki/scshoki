import { useState, useEffect, useRef } from "react";
import { Box, CircularProgress, Typography, List, ListItem, ListItemIcon, ListItemText } from "@mui/material";
import CheckCircleIcon from "@mui/icons-material/CheckCircle";
import RadioButtonUncheckedIcon from "@mui/icons-material/RadioButtonUnchecked";
import { invoke } from "@tauri-apps/api/core";


function delay(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

export const InstallationProgress = () => {
    const tasks = [
        { key: "setup_node", label: "Node.js" },
        { key: "setup_appium", label: "Appium" },
        { key: "setup_chromedriver", label: "ChromeDriver" },
        { key: "setup_geckodriver", label: "GeckoDriver" },
    ];

    const [completedTasks, setCompletedTasks] = useState<string[]>([]);
    const [currentTask, setCurrentTask] = useState<string | null>(null);
    const [isInstalling, setIsInstalling] = useState(false);
    const [success, setSuccess] = useState(false);
    const isExecuted = useRef(false);

    const checkInstalledBinaries = async () => {
        if (isExecuted.current) return;
        isExecuted.current = true;

        try {
            const installed: string[] = await invoke("check_installed_binaries");
            setCompletedTasks(installed); // すでにインストール済みのものをセット

            // 未インストールのものがあればインストール開始
            const missing = tasks.filter(task => !installed.includes(task.label));
            if (missing.length > 0) {
                setCurrentTask(missing[0].label); // 最初の未インストールタスクをセット
                setIsInstalling(true);
            } else {
                // Rust に通知を送ってAppiumを起動
                console.log("All binaries are already installed.");
                await invoke("start_appium");
            }
        } catch (error) {
            console.error("Error checking installed binaries:", error);
        }
    };

    const startInstallation = async () => {
        console.log("startInstallation called!");
        for (const task of tasks) {
            if (completedTasks.includes(task.label)) continue; // すでにインストール済みならスキップ

            setCurrentTask(task.label);
            // 100ms 待って React に UI を更新させる
            // これがないと、UI が更新されないことがある
            await delay(100);

            try {
                await invoke(task.key);
                setCompletedTasks((prev) => [...prev, task.label]); // 完了リストに追加
                console.log(`${task.label} installation completed.`);

                await delay(100);
            } catch (error) {
                console.error(`Error in ${task.key}:`, error);
                break;
            }
        }

        // Rust に通知を送ってAppiumを起動
        console.log("Installation completed, starting Appium...");
        await invoke("start_appium");

        setSuccess(true);
        setCurrentTask(null);
        // 「インストールが完了しました！」が見えるように、ちょっと待機
        await delay(3000);
        setIsInstalling(false);
    };

    // アプリ起動時にインストール済みのバイナリをチェック
    if (!isExecuted.current) {
        checkInstalledBinaries();
        isExecuted.current = true;
    };

    // インストールされていないバイナリがあったら、インストールを実行
    useEffect(() => {
        if (isInstalling) {
            startInstallation();
        }
    }, [isInstalling]);

    return (
        <>
            {isInstalling && (
                <Box
                    sx={{
                        position: "fixed",
                        top: 0,
                        left: 0,
                        width: "100%",
                        height: "100%",
                        backgroundColor: "rgba(0, 0, 0, 0.5)", // 半透明の背景
                        display: "flex",
                        justifyContent: "center",
                        alignItems: "center",
                        flexDirection: "column",
                        zIndex: 1000, // 他の要素の上に表示
                    }}
                >
                    {success ? <CheckCircleIcon color="success" sx={{ width: 80, height: 80 }} /> : <CircularProgress size={80} />}
                    <Typography variant="h6" sx={{ mt: 2 }}>
                        {currentTask ? `${currentTask} をインストール中です...` : "インストールが完了しました！"}
                    </Typography>
                    <List sx={{ mt: 2 }}>
                        {tasks.map((task) => (
                            <ListItem key={task.key}>
                                <ListItemIcon sx={{ minWidth: "40px" }}>
                                    {completedTasks.includes(task.label) ? (
                                        <CheckCircleIcon color="success" />
                                    ) : (
                                        <RadioButtonUncheckedIcon color="warning" />
                                    )}
                                </ListItemIcon>
                                <ListItemText primary={task.label} />
                            </ListItem>
                        ))}
                    </List>
                </Box>
            )}
        </>
    );
};
