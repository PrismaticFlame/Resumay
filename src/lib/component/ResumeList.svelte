<script lang="ts">
    let { resume } = $props();
</script>

<div class="row head"><span>Label</span><span>Date</span><span>Use Count</span></div>
{#each resume as a (r.id)}
    <button class="row" class:on={r.id === rSelectedId} onclick={() => onselect(r.id)}>
        <span class="who"><b>{r.label}</b><small>{r.date}</small></span>
        <span class="mono"></span>
    </button>
{/each}