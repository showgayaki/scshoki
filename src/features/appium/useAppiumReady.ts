import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";

export function useAppiumReady() {
    const [ready, setReady] = useState(false);

    useEffect(() => {
        const unlisten = listen<string>("appium_ready", () => {
            console.log("Appium is ready!");
            setReady(true);
        });

        return () => {
            unlisten.then((f) => f());
        };
    }, []);

    return ready;
}
