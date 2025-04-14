import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Home from "./pages/Home";
import { InstallationProgress } from "./components/InstallationProgress";
import { DeviceToast } from "./components/DeviceToast";

function App() {
    return (
        <>
            <InstallationProgress />
            <DeviceToast />
            <Router>
                <Routes>
                    <Route path="/" element={<Home />} />
                </Routes>
            </Router>
        </>
    );
}

export default App;
