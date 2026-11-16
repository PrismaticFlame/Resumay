import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { getVersion } from "@tauri-apps/api/app";

export const app = $state({ version: '' });

export async function loadVersion() {
    app.version = await getVersion();
}

export async function checkForUpdate(manual = false) {
    try {
        const update = await check();
        if (update && confirm(`Resumay ${update.version} is available. Install and restart?`)) {
            await update.downloadAndInstall();
            await relaunch();
        }
        if (!update) {
            if (manual) alert("You're on the latest version.");
            return;
        }
    } catch (e) {
        if (manual) throw e;
        console.warn('Update check failed:', e);

    }
}