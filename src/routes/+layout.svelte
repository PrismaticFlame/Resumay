<script lang="ts">
    import { refreshCounts } from "$lib/counts.svelte";
    import { check } from '@tauri-apps/plugin-updater';
    import { relaunch } from '@tauri-apps/plugin-process';

    import "@fontsource/ibm-plex-sans/400.css";
    import "@fontsource/ibm-plex-sans/600.css";
    import "@fontsource/ibm-plex-mono/400.css";
    import "../app.css";
    import { onMount } from "svelte";

    let { children } = $props();

    onMount(refreshCounts);

    (async () => {
        try {
            const update = await check();
            if (update && confirm(`Resumay ${update.version} is available. Install and restart?`)) {
            await update.downloadAndInstall();
            await relaunch();
            }
        } catch (e) {
            console.warn('Update check failed:', e);
        }
    })();
    
    

</script>
{@render children()}