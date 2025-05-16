import { useState, useEffect, useRef } from "react";

import { checkInstalledBinaries, installTask, startAppium } from "./api";
import { delay } from "./utils";

export function useInstallationTasks(tasks: { key: string; label: string }[]) {
    const [completedTasks, setCompletedTasks] = useState<string[]>([]);
    const [currentTask, setCurrentTask] = useState<string | null>(null);
    const [isInstalling, setIsInstalling] = useState(false);
    const [success, setSuccess] = useState(false);
    const [showDependenciesAlert, setShowDependenciesAlert] = useState(false);
    const isExecuted = useRef(false);

    const check = async () => {
        if (isExecuted.current) return;
        isExecuted.current = true;

        try {
            const installed = await checkInstalledBinaries();
            setCompletedTasks(installed);
            const missing = tasks.filter(task => !installed.includes(task.label));
            if (missing.length > 0) {
                setCurrentTask(missing[0].label);
                setIsInstalling(true);
            } else {
                setShowDependenciesAlert(true);
                setSuccess(true);
                await startAppium();
            }
        } catch (error) {
            console.error("Error checking installed binaries:", error);
        }
    };

    const start = async () => {
        for (const task of tasks) {
            if (completedTasks.includes(task.label)) continue;
            setCurrentTask(task.label);
            await delay(100);
            try {
                await installTask(task.key);
                setCompletedTasks(prev => [...prev, task.label]);
                await delay(100);
            } catch (error) {
                console.error(`Error in ${task.key}:`, error);
                break;
            }
        }
        await startAppium();
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
            start();
        }
    }, [isInstalling]);

    return {
        completedTasks,
        currentTask,
        isInstalling,
        success,
        showDependenciesAlert,
    };
}
