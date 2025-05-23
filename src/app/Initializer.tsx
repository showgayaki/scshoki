import { useEffect, useState } from "react";
import { useAppiumReady } from "@/features/appium/useAppiumReady";

import InstallationProgress from "@/features/installation/InstallationProgress";
import { startAppium } from "@/features/appium/api";
import AppiumStartupOverlay from "@/features/appium/AppiumStartupOverlay";
import { useDependencies } from "@/features/dependencies/useDependencies";
import DependenciesAlert from "@/features/dependencies/DependenciesAlert";


export const AppInitializer = () => {
    const appiumReady = useAppiumReady();
    const [installComplete, setInstallComplete] = useState(false);
    const { dependencies, open, setOpen } = useDependencies();

    useEffect(() => {
        if (!installComplete) return;
        startAppium().then(() => console.log("Appium started"));
    }, [installComplete]);

    return (
        <>
            {!installComplete && <InstallationProgress onComplete={() => setInstallComplete(true)} />}
            {installComplete && !appiumReady && <AppiumStartupOverlay />}
            <DependenciesAlert open={open} dependencies={dependencies} onClose={() => setOpen(false)} />
        </>
    );
};
