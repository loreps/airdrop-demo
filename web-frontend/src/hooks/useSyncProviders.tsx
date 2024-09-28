import { useSyncExternalStore } from "react";
import { store } from "./store";

export const useSyncProviders = () => {
    return useSyncExternalStore(store.subscribe, store.value, store.value);
}
