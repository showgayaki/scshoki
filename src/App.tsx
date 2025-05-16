import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";

import Home from "./pages/Home";
import { InstallationProgress } from "./features/installation/InstallationProgress";
import { DeviceToast } from "./components/DeviceToast";
import AppiumStartupOverlay from "./components/AppiumStartupOverlay";

function App() {
    const [appiumReady, setAppiumReady] = useState(false);
    const [installComplete, setInstallComplete] = useState(false);

    useEffect(() => {
        const unlisten = listen("appium_ready", () => {
            console.log("Appium is ready!");
            setTimeout(() => {
                setAppiumReady(true);
            }, 1000);
        });

        return () => {
            unlisten.then((f) => f());
        };
    }, []);

    return (
        <>
            {!installComplete && <InstallationProgress onComplete={() => setInstallComplete(true)} />}
            <DeviceToast />
            {installComplete && !appiumReady && <AppiumStartupOverlay />}
            <Router>
                <Routes>
                    <Route path="/" element={<Home />} />
                </Routes>
            </Router>
        </>
    );
}

export default App;
