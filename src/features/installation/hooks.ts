import { useState, useEffect, useRef } from "react";

import type { TaskStatuses } from "@/types/taskStatuses";

import { checkInstalledBinaries, installTask } from "./api";
import { delay } from "./utils";

export function useInstallationTasks(tasks: { key: string; label: string }[]) {
    const [taskStatuses, setTaskStatuses] = useState<TaskStatuses>(() =>
        Object.fromEntries(tasks.map((label) => [label, "pending"]))
    );
    const [currentTask, setCurrentTask] = useState<string | null>(null);
    const [isInstalling, setIsInstalling] = useState(false);
    const [success, setSuccess] = useState(false);
    const [showDependenciesAlert, setShowDependenciesAlert] = useState(false);
    const isExecuted = useRef(false); // StrictMode（開発時）での2回実行を回避

    const check = async () => {
        if (isExecuted.current) return;
        isExecuted.current = true;

        try {
            const installed = await checkInstalledBinaries();
            setTaskStatuses(
                Object.fromEntries(tasks.map(
                    (task) => [task.label, installed.includes(task.label) ? "success" : "pending"]
                ))
            );

            const missing = tasks.filter(task => !installed.includes(task.label));
            if (missing.length > 0) {
                setCurrentTask(missing[0].label);
                setIsInstalling(true);
            } else {
                setShowDependenciesAlert(true);
                setSuccess(true);
            }
        } catch (error) {
            console.error("Error checking installed binaries:", error);
        }
    };

    const installedBinaries = async () => {
        for (const task of tasks) {
            if (taskStatuses[task.label] !== "pending") continue;
            setCurrentTask(task.label);
            await delay(100);
            try {
                await installTask(task.key);
                setTaskStatuses(prev => ({ ...prev, [task.label]: "success" }));
                await delay(100);
            } catch (error) {
                console.error(`Error in ${task.key}:`, error);
                setTaskStatuses(prev => ({ ...prev, [task.label]: "error" }));
                break;
            }
        }

        setSuccess(true);
        setCurrentTask(null);
        await delay(3000);
        setIsInstalling(false);
        setShowDependenciesAlert(true);
    };

    useEffect(() => {
        check();
    }, []);

    useEffect(() => {
        if (isInstalling) {
            installedBinaries();
        }
    }, [isInstalling]);

    return {
        taskStatuses,
        currentTask,
        isInstalling,
        success,
        showDependenciesAlert,
    };
}
