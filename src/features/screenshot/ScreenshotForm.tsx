import { useScreenshotFormState } from "./useScreenshotFormState";
import { useScreenshot } from "./useScreenshot";
import BaseUrlInput from "./components/BaseUrlInput";
import HiddenElementsSection from "./components/HiddenElementsSection";
import TargetPagePathsSection from "./components/TargetPagePathsSection";
import BasicAuthSection from "./components/BasicAuthSection";
import BrowserSelectSection from "./components/BrowserSelectSection";
import ScreenshotButton from "./components/ScreenshotButton";
import ScreenshotOverlay from "./components/ScreenshotOverlay";

export default function ScreenshotForm() {
    const {
        url,
        setUrl,
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
        browserError,
        validate,
    } = useScreenshotFormState();

    const { status, isCapturing, takeScreenshot, cancelScreenshot } = useScreenshot();

    const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        if (validate()) {
            takeScreenshot({url, targetPagePaths, hiddenElements, selectedBrowsers});
        }else {
            console.warn("Validation failed, not submitting form");
        }
    };

    return (
        <>
            {isCapturing && <ScreenshotOverlay handleCancel={cancelScreenshot} />}
            <form className="space-y-4" onSubmit={handleSubmit}>
                <BaseUrlInput url={url} setUrl={setUrl} error={urlError} />
                <HiddenElementsSection hiddenElements={hiddenElements} setHiddenElements={setHiddenElements} />
                <TargetPagePathsSection targetPagePaths={targetPagePaths} setTargetPagesPaths={setTargetPagePaths} />
                <BasicAuthSection
                    useAuth={useAuth}
                    setUseAuth={setUseAuth}
                    username={username}
                    setUsername={setUsername}
                    password={password}
                    setPassword={setPassword}
                />
                <BrowserSelectSection selectedBrowsers={selectedBrowsers} setSelectedBrowsers={setSelectedBrowsers} error={browserError} />
                {!isCapturing && <ScreenshotButton />}
            </form>
        </>
    );
}
