import { useState } from "react";

import UrlInputForm from "./UrlInputForm";
import BasicAuthForm from "./BasicAuthForm";
import HiddenElementsForm from "./HiddenElementsForm";
import BrowserSelect from "./BrowserSelect";
import ScreenshotButton from "./ScreenshotButton";

export default function Home() {
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

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        console.log({
            url,
            useAuth,
            username,
            password,
            selectedBrowsers,
        });
    };

    return (
        <div className="p-4 max-w-lg mx-auto">
            <h1 className="text-2xl font-bold mb-4">scshoki</h1>
            <form onSubmit={handleSubmit} className="space-y-4">
                <UrlInputForm url={url} setUrl={setUrl} />
                <BasicAuthForm
                    useAuth={useAuth}
                    setUseAuth={setUseAuth}
                    username={username}
                    setUsername={setUsername}
                    password={password}
                    setPassword={setPassword}
                />
                <HiddenElementsForm hiddenElements={hiddenElements} setHiddenElements={setHiddenElements} />
                <BrowserSelect selectedBrowsers={selectedBrowsers} setSelectedBrowsers={setSelectedBrowsers} />
                <ScreenshotButton url={url} hiddenElements={hiddenElements} selectedBrowsers={selectedBrowsers} />
            </form>
        </div>
    );
}
