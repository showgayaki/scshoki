import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { InstallationProgress } from "./components/InstallationProgress";
import DependenciesAlert from "./components/DependenciesAlert";
import './styles/main.scss';


ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <InstallationProgress />
        <DependenciesAlert />
        <App />
    </React.StrictMode>,
);
