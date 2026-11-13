import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';

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