<script lang="ts">
    import { refreshCounts } from "$lib/counts.svelte";
    import { checkForUpdate, loadVersion, updater } from "$lib/updater.svelte";

    import UpdateModal from "$lib/component/settings/UpdateModal.svelte";

    import "@fontsource/ibm-plex-sans/400.css";
    import "@fontsource/ibm-plex-sans/600.css";
    import "@fontsource/ibm-plex-mono/400.css";
    import "../app.css";
    import { onMount } from "svelte";
    

    let { children } = $props();

    onMount(() => {
        refreshCounts();
        checkForUpdate();
        loadVersion();
    });
    
</script>

{@render children()}

{#if updater.available && !updater.dismissed}
    <UpdateModal />
{/if}