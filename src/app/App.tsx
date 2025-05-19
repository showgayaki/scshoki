import { DeviceSnackbar } from "@/features/detect/DeviceSnackbar";

import { AppInitializer } from "./Initializer";
import { AppRouter } from "./router";

function App() {
    return (
        <>
            <AppInitializer />
            <DeviceSnackbar />
            <AppRouter />
        </>
    );
}

export default App;
