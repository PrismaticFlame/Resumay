<script lang="ts">
    import { timeFormat } from "$lib/time-format";
    let { resume, rSelectedId, onselect } = $props();
</script>

<div class="row head"><span>Resume Name</span><span>Use Counts</span><span>Date Created</span></div>
{#each resume as r (r.id)}
    <button class="row" class:on={r.id === rSelectedId} onclick={() => onselect(r.id)}>
        <span class="who"><b>{r.label}</b></span>
        <span class="mono">{r.uses}</span>
        <span class="mono">{timeFormat(r.created_at)}</span>
    </button>
{/each}