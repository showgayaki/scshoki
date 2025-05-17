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

    const handleClick = () => {
        takeScreenshot({ url, hiddenElements, selectedBrowsers });
    };

    return (
        <form className="space-y-4">
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
            <ScreenshotButton onClick={handleClick} status={status} url={url} hiddenElements={hiddenElements} selectedBrowsers={selectedBrowsers} />
        </form>
    );
}