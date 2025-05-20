import { ThemeProvider, CssBaseline } from "@mui/material";
import { DeviceSnackbar } from "@/features/detect/DeviceSnackbar";

import darkTheme from "./theme";
import { AppInitializer } from "./Initializer";
import { AppRouter } from "./router";

function App() {
    return (
        <>
            <ThemeProvider theme={darkTheme}>
                <CssBaseline />
                <AppInitializer />
                <DeviceSnackbar />
                <AppRouter />
            </ThemeProvider>
        </>
    );
}

export default App;
