import { useScreenshotFormState } from "./useScreenshotFormState";
import { useScreenshot } from "./useScreenshot";
import DeviceNotFoundDialog from "./components/DeviceNotFoundDialog";
import BaseUrlInput from "./components/BaseUrlInput";
import HiddenElementsSection from "./components/HiddenElementsSection";
import TargetPagePathsSection from "./components/TargetPagePathsSection";
import BasicAuthSection from "./components/BasicAuthSection";
import BrowserSelectSection from "./components/BrowserSelectSection";
import ScreenshotButton from "./components/ScreenshotButton";
import ScreenshotProgress from "./components/ScreenshotProgress";

export default function ScreenshotForm() {
    const {
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
        browserError,
        validate,
    } = useScreenshotFormState();

    const {
        status,
        isTakingScreenshot,
        groupedTaskStatuses,
        success,
        showDeviceNotFoundDialog,
        setShowDeviceNotFoundDialog,
        checkDeviceConnected,
        takeScreenshot,
        cancelScreenshot,
    } = useScreenshot();

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();

        const isConnected = await checkDeviceConnected();
        if (!isConnected) {
            console.warn("No device connected, cannot take screenshot");
            setShowDeviceNotFoundDialog(true);
            return;
        }
        if (validate()) {
            takeScreenshot({ baseUrl, targetPagePaths, hiddenElements, selectedBrowsers });
        } else {
            console.warn("Validation failed, not submitting form");
        }
    };

    return (
        <>
            <DeviceNotFoundDialog
                open={showDeviceNotFoundDialog}
                onClose={() => setShowDeviceNotFoundDialog(false)}
            />
            {isTakingScreenshot &&
                <ScreenshotProgress
                    status={status}
                    groupedTaskStatuses={groupedTaskStatuses}
                    success={success}
                    handleCancel={cancelScreenshot}
                />}
            <form className="space-y-4" onSubmit={handleSubmit}>
                <BaseUrlInput url={baseUrl} setUrl={setBaseUrl} error={urlError} />
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
                {!isTakingScreenshot && <ScreenshotButton />}
            </form>
        </>
    );
}
