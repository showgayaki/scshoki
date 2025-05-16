import { useState } from "react";
import { useAppiumReady } from "@/features/appium/useAppiumReady";

import { InstallationProgress } from "@/features/installation/InstallationProgress";
import { DeviceSnackbar } from "@/features/detect/DeviceSnackbar";
import AppiumStartupOverlay from "@/features/appium/AppiumStartupOverlay";
import { useDependencies } from "@/features/dependencies/useDependencies";
import DependenciesAlert from "@/features/dependencies/DependenciesAlert";
import { AppRouter } from "./router";

function App() {
    const appiumReady = useAppiumReady();
    const [installComplete, setInstallComplete] = useState(false);
    const { dependencies, open, setOpen } = useDependencies();

    return (
        <>
            {!installComplete && <InstallationProgress onComplete={() => setInstallComplete(true)} />}
            {installComplete && !appiumReady && <AppiumStartupOverlay />}
            <DependenciesAlert open={open} dependencies={dependencies} onClose={() => setOpen(false)} />
            <DeviceSnackbar />
            <AppRouter />
        </>
    );
}

export default App;
