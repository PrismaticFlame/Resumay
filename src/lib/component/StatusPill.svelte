<script lang="ts">
    import { STATUS, FALLBACK } from "$lib/status";
    let { status } = $props();
    let s = $derived(STATUS[status] ?? { ...FALLBACK, label: status });
</script>

<span class="pill" style:background={s.bg} style:color={s.fg}>
    <i style:background={s.fg}></i>{s.label}
</span>

<style>
    .pill { display: inline-flex; align-items: center; gap: 6px; padding: 4px 10px;
            border-radius: 999px; font-size: 12px; font-weight: 600; }
    i { width: 6px; height: 6px; border-radius: 50%; }
</style>