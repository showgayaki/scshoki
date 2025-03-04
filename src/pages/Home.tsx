import { useState } from "react";
import UrlInputForm from "../components/UrlInputForm";
import BasicAuthForm from "../components/BasicAuthForm";
import HiddenElementsForm from "../components/HiddenElementsForm";
import BrowserSelect from "../components/BrowserSelect";
import ScreenshotButton from "../components/ScreenshotButton";

export default function Home() {
    const [url, setUrl] = useState("");
    const [useAuth, setUseAuth] = useState(false);
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const [hiddenElements, setHiddenElements] = useState("");
    const [selectedBrowser, setSelectedBrowser] = useState("Chrome");

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        console.log({
            url,
            useAuth,
            username,
            password,
            selectedBrowser,
        });
        // TauriのRust側に送信する処理をここに追加予定
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
                <BrowserSelect selectedBrowser={selectedBrowser} setSelectedBrowser={setSelectedBrowser} />
                <ScreenshotButton url={url} hiddenElements={hiddenElements} />
            </form>
        </div>
    );
}
