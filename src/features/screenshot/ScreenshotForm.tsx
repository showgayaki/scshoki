import { useScreenshotFormState } from "./useScreenshotFormState";
import { useScreenshot } from "./useScreenshot";
import BaseUrlInput from "./components/BaseUrlInput";
import HiddenElementsSection from "./components/HiddenElementsSection";
import TargetPagePathsSection from "./components/TargetPagePathsSection";
import BasicAuthSection from "./components/BasicAuthSection";
import BrowserSelect from "./components/BrowserSelect";
import ScreenshotButton from "./components/ScreenshotButton";
import ScreenshotOverlay from "./components/ScreenshotOverlay";

export default function ScreenshotForm() {
    const {
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
    } = useScreenshotFormState();

    const { status, isCapturing, takeScreenshot, cancelCapture } = useScreenshot();

    const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        takeScreenshot({ url, hiddenElements, selectedBrowsers });
    };

    return (
        <>
        {isCapturing && <ScreenshotOverlay />}
        <form className="space-y-4" onSubmit={handleSubmit}>
            <BaseUrlInput url={url} setUrl={setUrl} />
            <HiddenElementsSection hiddenElements={hiddenElements} setHiddenElements={setHiddenElements} />
            <TargetPagePathsSection targetPages={targetPagePaths} setTargetPagesPaths={settargetPagePaths} />
            {/* inputの下に無駄なmarginが入っている */}
            <BasicAuthSection
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
        </>
    );
}
