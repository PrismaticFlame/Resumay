<script lang="ts">
    import type { Snippet } from 'svelte';

    let { label, description, children }: {
        label: string;
        description: string;
        children: Snippet;
    } = $props();

</script>

<div class="row">
    <div class="text">
        <span class="label">{label}</span>
        {#if description}<span class="desc">{description}</span>{/if}
    </div>
    {@render children()}
</div>

<style>
    .row { 
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding-top: 12px 0;
        gap: 4px;  
    }
    .row + .row { border-top: 1px solid var(--line); }
    .text { display: flex; flex-direction: column; gap: 2px; }
    .desc { font-size: 0.85em; color: var(--muted); }
</style>