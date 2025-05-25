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
    const [urlError, setUrlError] = useState<string | null>(null);
    const [browserError, setBrowserError] = useState<string | undefined>(undefined);

    const validate = () => {
        let errorCleared = false;

        if (!url) {
            setUrlError("URLを入力してください");
            errorCleared = true;
        } else {
            try {
                new URL(url);
            } catch {
                setUrlError("有効なURLを入力してください");
                errorCleared = true;
            }
        }

        const isAnyBrowserSelected = Object.values(selectedBrowsers).some((selected) => selected);
        if (!isAnyBrowserSelected) {
            setBrowserError("ブラウザを選択してください");
            errorCleared = true;
        }

        if (errorCleared) {
            setTimeout(() => {
                setUrlError(null);
                setBrowserError(undefined);
            }, 3000);
            return;
        }

        setUrlError(null);
        setBrowserError(undefined);
    };

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
        urlError,
        setUrlError,
        browserError,
        setBrowserError,
        validate,
    };
}
