import { useEffect, useState } from "react";

export function useScreenshotFormState() {
    const [baseUrl, setBaseUrl] = useState("");
    const [hiddenElements, setHiddenElements] = useState([] as string[]);
    const [targetPagePaths, setTargetPagePaths] = useState([] as string[]);
    const [useAuth, setUseAuth] = useState(false);
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const [selectedBrowsers, setSelectedBrowsers] = useState([] as string[]);
    const [urlError, setUrlError] = useState<string | null>(null);
    const [usernameError, setUsernameError] = useState<string | null>(null);
    const [passwordError, setPasswordError] = useState<string | null>(null);
    const [browserError, setBrowserError] = useState<string | undefined>(undefined);

    const validate = () => {
        let hasError = false;
        console.log("targetPagePaths:", targetPagePaths);

        if (!baseUrl) {
            setUrlError("URLを入力してください");
            hasError = true;
        } else {
            try {
                new URL(baseUrl);
            } catch {
                setUrlError("有効なURLを入力してください");
                hasError = true;
            }
        }

        if (selectedBrowsers.length === 0) {
            setBrowserError("ブラウザを選択してください");
            hasError = true;
        }

        if (useAuth) {
            if (username === "") {
                setUsernameError("ユーザー名を入力してください");
                hasError = true;
            }
            if (password === "") {
                setPasswordError("パスワードを入力してください");
                hasError = true;
            }
        }

        if (hasError) {
            setTimeout(() => {
                setUrlError(null);
                setUsernameError(null);
                setPasswordError(null);
                setBrowserError(undefined);
            }, 3000);
            return false;
        }

        return true;
    };


    useEffect(() => {
        try {
            new URL(baseUrl);
            console.log("useScreenshotFormState initialized with URL:", baseUrl);
            if (!targetPagePaths.includes("/")) {
                console.log("Adding default path '/' to targetPagePaths");
                setTargetPagePaths((prev) => ["/", ...prev]);
            }
        } catch {
            // Do nothing if URL is invalid
        }

        if (baseUrl === "") {
            console.log("Resetting targetPagePaths to default");
            setTargetPagePaths([]);
        }
    }, [baseUrl]);

    return {
        baseUrl,
        setBaseUrl,
        hiddenElements,
        setHiddenElements,
        targetPagePaths,
        setTargetPagePaths,
        useAuth,
        setUseAuth,
        username,
        setUsername,
        password,
        setPassword,
        selectedBrowsers,
        setSelectedBrowsers,
        urlError,
        usernameError,
        passwordError,
        browserError,
        validate,
    };
}
