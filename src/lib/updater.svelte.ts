import { check, Update } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { getVersion } from "@tauri-apps/api/app";


export const app = $state({ version: '' });

export const updater = $state({
    available: null as { version: string; notes: string } | null,
    status: 'idle' as 'idle' | 'downloading' | 'error',
    progress: 0,
    dismissed: false,
});

let pending: Update | null = null;

export async function loadVersion() {
    app.version = await getVersion();
}

export async function checkForUpdate(manual = false) {
    try {
        const update = await check();
        // if (update && confirm(`Resumay ${update.version} is available. Install and restart?`)) {
        //     await update.downloadAndInstall();
        //     await relaunch();
        // }
        if (update) {
            pending = update;
            updater.available = { version: update.version, notes: update.body ?? '' };
            updater.dismissed = false;
            return true;
        }
        return false;
        if (!update) {
            if (manual) alert("You're on the latest version.");
            return;
        }
    } catch (e) {
        if (manual) throw e;
        console.warn('Update check failed:', e);

    }
}

export async function installUpdate() {
    if (!pending) return;
    updater.status = 'downloading';
    let total = 0, done = 0;
    try {
        await pending.downloadAndInstall((e) => {
            if (e.event === 'Started') total = e.data.contentLength ?? 0;
            if (e.event === 'Progress') {
                done += e.data.chunkLength;
                updater.progress = total ? done / total : 0;
            }
        });
        await relaunch();
    } catch {
        updater.status = 'error';
    }
}