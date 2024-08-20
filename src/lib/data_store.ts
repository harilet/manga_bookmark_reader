import { Store } from "@tauri-apps/plugin-store";

const dataStore = new Store('store.bin');

export default dataStore;