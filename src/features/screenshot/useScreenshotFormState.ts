import { useState } from "react";

export function useScreenshotFormState() {
    const [url, setUrl] = useState("");
    const [useAuth, setUseAuth] = useState(false);
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const [hiddenElements, setHiddenElements] = useState("");
    const [selectedBrowsers, setSelectedBrowsers] = useState<Record<string, boolean>>({
        Chrome: false,
        Firefox: false,
        Safari: false,
    });

    return {
        url,
        setUrl,
        useAuth,
        setUseAuth,
        username,
        setUsername,
        password,
        setPassword,
        hiddenElements,
        setHiddenElements,
        selectedBrowsers,
        setSelectedBrowsers,
    };
}
