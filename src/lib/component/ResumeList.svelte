<script lang="ts">
    let { resume, rSelectedId, onselect } = $props();
</script>

<div class="row head"><span>Label</span><span>Day Created</span><span>Use Count</span></div>
{#each resume as r (r.id)}
    <button class="row" class:on={r.id === rSelectedId} onclick={() => onselect(r.id)}>
        <span class="who"><b>{r.label}</b><small>{r.created_at ?? ""}</small></span>
        <span class="mono">{r.uses}</span>
    </button>
{/each}