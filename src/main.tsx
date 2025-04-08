import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import DependenciesAlert from "./components/DependenciesAlert";
import './styles/main.scss';

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <DependenciesAlert />
        <App />
    </React.StrictMode>,
);
