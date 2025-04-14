import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { InstallationProgress } from "./components/InstallationProgress";
import { DeviceToast } from "./components/DeviceToast";
import './styles/main.scss';


ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <InstallationProgress />
        <DeviceToast />
        <App />
    </React.StrictMode>,
);
