const browsers = [
    { name: "Chrome", colorImg: "/images/chrome--active.png", grayImg: "/images/chrome--inactive.png" },
    { name: "Firefox", colorImg: "/images/firefox--active.png", grayImg: "/images/firefox--inactive.png" },
    { name: "Safari", colorImg: "/images/safari--active.png", grayImg: "/images/safari--inactive.png" },
    { name: "Edge", colorImg: "/images/edge--active.png", grayImg: "/images/edge--inactive.png" },
];

interface BrowserSelectProps {
    selectedBrowsers: Record<string, boolean>;
    setSelectedBrowsers: React.Dispatch<React.SetStateAction<Record<string, boolean>>>;
}

export default function BrowserSelect({ selectedBrowsers, setSelectedBrowsers }: BrowserSelectProps) {
    const toggleBrowser = (name: string) => {
        setSelectedBrowsers((prev) => ({
            ...prev,
            [name]: !prev[name], // 選択状態をトグル
        }));
    };

    return (
        <div style={{ display: "flex", gap: "20px" }}>
            {browsers.map((browser) => (
                <div key={browser.name} onClick={() => toggleBrowser(browser.name)} style={{ cursor: "pointer", textAlign: "center" }}>
                    <img
                        src={selectedBrowsers[browser.name] ? browser.colorImg : browser.grayImg}
                        alt={browser.name}
                        width={80}
                        height={80}
                        style={{ display: "block", marginBottom: "5px" }}
                    />
                    <span style={{ display: "block", color: "#fff" }}>{browser.name}</span>
                </div>
            ))}
        </div>
    );
}
