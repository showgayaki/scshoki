import { useState } from "react";

export function useScreenshotFormState() {
    const [url, setUrl] = useState("");
    const [hiddenElements, setHiddenElements] = useState([] as string[]);
    const [targetPagePaths, settargetPagePaths] = useState([] as string[]);
    const [useAuth, setUseAuth] = useState(false);
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const [selectedBrowsers, setSelectedBrowsers] = useState([] as string[]);
    const [urlError, setUrlError] = useState<string | null>(null);
    const [browserError, setBrowserError] = useState<string | undefined>(undefined);

    const validate = () => {
        let hasError = false;

        if (!url) {
            setUrlError("URLを入力してください");
            hasError = true;
        } else {
            try {
                new URL(url);
            } catch {
                setUrlError("有効なURLを入力してください");
                hasError = true;
            }
        }

        if (selectedBrowsers.length === 0) {
            setBrowserError("ブラウザを選択してください");
            hasError = true;
        }

        if (hasError) {
            setTimeout(() => {
                setUrlError(null);
                setBrowserError(undefined);
            }, 3000);
            return false;
        }

        setUrlError(null);
        setBrowserError(undefined);
        return true;
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
