import { useScreenshotFormState } from "./useScreenshotFormState";
import { useScreenshot } from "./useScreenshot";
import UrlInputForm from "./components/UrlInputForm";
import BasicAuthForm from "./components/BasicAuthForm";
import HiddenElementsForm from "./components/HiddenElementsForm";
import BrowserSelect from "./components/BrowserSelect";
import ScreenshotButton from "./components/ScreenshotButton";

export default function ScreenshotForm() {
    const {
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
    } = useScreenshotFormState();

    const { status, takeScreenshot } = useScreenshot();

    const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        takeScreenshot({ url, hiddenElements, selectedBrowsers });
    };

    return (
        <form className="space-y-6" onSubmit={handleSubmit}>
            <UrlInputForm url={url} setUrl={setUrl} />
            <HiddenElementsForm hiddenElements={hiddenElements} setHiddenElements={setHiddenElements} />
            <BasicAuthForm
                useAuth={useAuth}
                setUseAuth={setUseAuth}
                username={username}
                setUsername={setUsername}
                password={password}
                setPassword={setPassword}
            />
            <BrowserSelect selectedBrowsers={selectedBrowsers} setSelectedBrowsers={setSelectedBrowsers} />
            <ScreenshotButton status={status} url={url} hiddenElements={hiddenElements} selectedBrowsers={selectedBrowsers} />
        </form>
    );
}
