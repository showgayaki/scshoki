import { create } from "zustand";

interface DeviceState {
    isConnected: boolean;
    setIsConnected: (value: boolean) => void;
}

export const useDeviceStore = create<DeviceState>((set) => ({
    isConnected: false,
    setIsConnected: (value) => set({ isConnected: value }),
}));
