import { useRef, useState, useEffect } from "react";

import { isIosDependenciesInstalled } from "./api";

export type Dependency = { name: string; installed: boolean };

export function useDependencies() {
    const [dependencies, setDependencies] = useState<Dependency[]>([]);
    const [open, setOpen] = useState(false);
    const isExecuted = useRef(false);

    useEffect(() => {
        if (isExecuted.current) return;
        isExecuted.current = true;

        const checkDependencies = async () => {
            try {
                const installed = await isIosDependenciesInstalled();
                const formattedList = installed.map(obj => {
                    const [name, installed] = Object.entries(obj)[0];
                    return { name, installed };
                });
                setDependencies(formattedList);
                setOpen(!formattedList.every(dep => dep.installed));
            } catch (err) {
                console.error("invoke error:", err);
            }
        };

        checkDependencies();
    }, []);

    return { dependencies, open, setOpen };
}
