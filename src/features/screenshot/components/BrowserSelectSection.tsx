import { Box } from "@mui/material";

import BottomArrowTooltip from "@/components/CustomTooltips";
import { BROWSERS } from "@/constants/browsers";

interface BrowserSelectSectionProps {
    selectedBrowsers: string[];
    setSelectedBrowsers: React.Dispatch<React.SetStateAction<string[]>>;
    error?: string;
}

export default function BrowserSelectSection({ selectedBrowsers, setSelectedBrowsers, error }: BrowserSelectSectionProps) {
    const toggleBrowser = (name: string) => {
        setSelectedBrowsers((prev) =>
            prev.includes(name)
                ? prev.filter((b) => b !== name)
                : [...prev, name]
        );
    };
    console.log("selectedBrowsers", selectedBrowsers);

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
                    {BROWSERS.map((browser) => (
                        <Box key={browser.name} onClick={() => toggleBrowser(browser.name)} sx={{ cursor: "pointer", textAlign: "center" }}>
                            <img
                                src={selectedBrowsers.includes(browser.name) ? browser.colorImg : browser.grayImg}
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
