import { invoke } from "@tauri-apps/api/core";

export const isIosDependenciesInstalled = (): Promise<{ [key: string]: boolean }[]> =>
    invoke("is_ios_dependencies_installed");
