<script lang="ts">
    import { timeFormat } from "$lib/time-format";
    let { resume, rSelectedId, onselect } = $props();
</script>

<div class="head"><span>Resume Name</span><span>Num. Apps. Using</span></div>
{#each resume as r (r.id)}
    <button class="row" class:on={r.id === rSelectedId} onclick={() => onselect(r.id)}>
        <span class="top">
            <span class="label">{r.label}</span>
            <span class="uses">{r.uses}</span>
        </span>
        <small>{timeFormat(r.created_at)}</small>
    </button>
{/each}

<style>
    .head { 
        display: flex; 
        justify-content: space-between; 
        padding: 12px 16px; 
    }
    .row { display: flex; flex-direction: column; align-items: stretch;
       justify-content: flex-start; text-align: left;
       width: 100%; padding: 12px 16px; gap: 4px; }
    .top { display: flex; justify-content: space-between; align-items: baseline; gap: 12px; }
    .label { font-weight: 600; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
    .uses { font-weight: 700; flex-shrink: 0; }
    small { text-align: left; }
</style>