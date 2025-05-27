import { Box } from "@mui/material";

import TagsInput from "@/components/TagsInput";

interface TargetPagePathsSection {
    targetPages: string[];
    setTargetPagesPaths: (value: string[]) => void;
}

export default function TargetPagePathsSection({ targetPages: targetPages, setTargetPagesPaths: setTargetPages }: TargetPagePathsSection) {
    const options = [
        "/about",
        "/contact",
        "/access",
        "/company",
        "/services",
        "/products",
        "/support",
        "/faq",
        "/privacy",
        "/terms",
        "/news",
        "/blog",
        "/careers",
        "/recruit",
        "/sitemap",
        "/profile",
        "/login",
        "/signup",
        "/dashboard",
        "/settings",
        "/account",
        "/help",
        "/guide",
        "/downloads",
        "/pricing",
        "/features",
        "/portfolio",
        "/testimonials",
        "/case-studies",
        "/events",
        "/press",
        "/media",
        "/team"
    ];

    return (
        <Box>
            <TagsInput
                id="targetPages"
                label="ページ"
                placeholder="/contact, /about, /access"
                options={options}
                value={targetPages.join(", ")}
                onChange={(newValues) => setTargetPages(newValues)}
            />
        </Box>
    );
}
