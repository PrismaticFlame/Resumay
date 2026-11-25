
<script lang="ts">
  let { label, wide = false, children } = $props();
</script>

<label class:wide>
    <span>{label}</span>
    {@render children()}
</label>

<style>
    label { display: flex; flex-direction: column; gap: 6px; min-width: 0; }
    label span { font-size: 13px; font-weight: 600; color: var(--muted); }
    .wide { grid-column: 1 / -1; }
</style>

