<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { STATUS } from "$lib/status";
    import { onMount } from "svelte";
    import type { Resume } from "$lib/types";
    import { timeFormat } from "$lib/time-format";

    let {onclose, onsaved, app = null} = $props();

    let dragging = $state(false);
    let resumeFile = $state<File | null>(null);
    let resumeId = $state<number | null>(app?.resume ?? null);
    let previewUrl = $state("");
    
</script>

<div class="overlay">

</div>

<style>

</style>