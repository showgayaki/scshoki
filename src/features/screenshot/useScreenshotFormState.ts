import { useState } from "react";

export function useScreenshotFormState() {
    const [url, setUrl] = useState("");
    const [hiddenElements, setHiddenElements] = useState("");
    const [targetPagePaths, settargetPagePaths] = useState("");
    const [useAuth, setUseAuth] = useState(false);
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const [selectedBrowsers, setSelectedBrowsers] = useState<Record<string, boolean>>({
        Chrome: false,
        Firefox: false,
        Safari: false,
    });

    return {
        url,
        setUrl,
        hiddenElements,
        setHiddenElements,
        targetPagePaths,
        settargetPagePaths,
        useAuth,
        setUseAuth,
        username,
        setUsername,
        password,
        setPassword,
        selectedBrowsers,
        setSelectedBrowsers,
    };
}
