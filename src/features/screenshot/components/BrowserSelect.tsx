import { Box } from "@mui/material";

import BottomArrowTooltip from "@/components/CustomTooltips";

const browsers = [
    { name: "Chrome", colorImg: "/images/chrome--checked.png", grayImg: "/images/chrome--unchecked.png" },
    { name: "Firefox", colorImg: "/images/firefox--checked.png", grayImg: "/images/firefox--unchecked.png" },
    { name: "Safari", colorImg: "/images/safari--checked.png", grayImg: "/images/safari--unchecked.png" },
    { name: "Edge", colorImg: "/images/edge--checked.png", grayImg: "/images/edge--unchecked.png" },
];

interface BrowserSelectProps {
    selectedBrowsers: Record<string, boolean>;
    setSelectedBrowsers: React.Dispatch<React.SetStateAction<Record<string, boolean>>>;
    error?: string;
}

export default function BrowserSelect({ selectedBrowsers, setSelectedBrowsers, error }: BrowserSelectProps) {
    const toggleBrowser = (name: string) => {
        setSelectedBrowsers((prev) => ({
            ...prev,
            [name]: !prev[name], // 選択状態をトグル
        }));
    };

    return (
        <Box sx={{ display: "flex", flexDirection: "column" }}>
            <label>ブラウザ選択</label>
            <BottomArrowTooltip title={error || ""}>
                <Box sx={{
                        display: "flex",
                        gap: "30px",
                        pt: 1,
                        border: error ? "1px solid red" : "1px solid transparent",
                        borderRadius: 1,
                    }}
                >
                    {browsers.map((browser) => (
                        <Box key={browser.name} onClick={() => toggleBrowser(browser.name)} sx={{ cursor: "pointer", textAlign: "center" }}>
                            <img
                                src={selectedBrowsers[browser.name] ? browser.colorImg : browser.grayImg}
                                alt={browser.name}
                                width={80}
                                height={80}
                                style={{ display: "block", marginBottom: "5px" }}
                            />
                            <span>{browser.name}</span>
                        </Box>
                    ))}
                </Box>
            </BottomArrowTooltip>
        </Box>
    );
}
