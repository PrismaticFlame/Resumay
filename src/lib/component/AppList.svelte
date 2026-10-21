<script lang="ts">
    import StatusPill from "./StatusPill.svelte";
    let { apps, selectedId, onselect } = $props();
</script>

<div class="row head"><span>Company / role</span><span>Status</span><span>applied</span></div>
{#each apps as a (a.id)}
    <button class="row" class:on={a.id === selectedId} onclick={() => onselect(a.id)}>
        <span class="who"><b>{a.company}</b><small>{a.position}</small></span>
        <span><StatusPill status={a.status} /></span>
        <span class="mono">{a.apply_date}</span>
    </button>
{/each}
