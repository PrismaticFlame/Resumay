<script lang="ts">
    import { refreshCounts } from "$lib/counts.svelte";
    import { checkForUpdate, loadVersion, updater } from "$lib/updater.svelte";

    import UpdateModal from "$lib/component/settings/UpdateModal.svelte";
    import Rail from "$lib/component/Rail.svelte";

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

<div class="shell">
    <Rail />
    {@render children()}
</div>

<style>
    .shell { display: grid; grid-template-columns: 224px 1fr; height: 100vh; }
</style>

{#if updater.available && !updater.dismissed}
    <UpdateModal />
{/if}