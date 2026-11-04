<script>
    import { refreshCounts } from "$lib/counts.svelte";

    import "@fontsource/ibm-plex-sans/400.css";
    import "@fontsource/ibm-plex-sans/600.css";
    import "@fontsource/ibm-plex-mono/400.css";
    import "../app.css";
    import { onMount } from "svelte";

    let { children } = $props();

    onMount(refreshCounts);

</script>
{@render children()}