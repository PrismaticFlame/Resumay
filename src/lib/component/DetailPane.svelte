<script lang="ts">
    import StatusPill from "./StatusPill.svelte";
    let { app, ondelete } = $props();
    let confirming = $state(false);
</script>

<div class="pane">
    <StatusPill status={app.status} />
    <h2>{app.company}</h2>
    <p class="role">{app.position}</p>

    <h3>Portal</h3>
    <dl>
        <dt>Website</dt><dd>{app.website ?? "—"}</dd>
        <dt>Username</dt><dd class="mono">{app.username ?? "—"}</dd>
    </dl>

    <h3>Referral</h3><p>{app.refer ?? "—"}</p>
    <h3>Notes</h3><p>{app.notes ?? "—"}</p>
</div>

<div class="danger">
    {#if confirming}
        <span>Delete this application?</span>
        <button class="del" onclick={ondelete}>Yes, delete</button>
        <button onclick={() => confirming = false}>Cancel</button>
    {:else}
        <button class="del-ghost" onclick={() => confirming = true}>Delete</button>
    {/if}
</div>

<style>
    .pane { padding: 24px; }
    h2 { margin: 10px 0 2px; font-size: 22px; font-weight: 600; }
    .role { margin: 0 0 20px; }
    h3 { margin: 20px 0 8px; font-size: 12px; font-weight: 600; color: var(--muted);
         text-transform: uppercase; letter-spacing: .06em; }
    dl { display: grid; grid-template-columns: 96px 1fr; gap: 8px 12px; margin: 0; }
    dt { color: var(--muted); font-size: 13px; }
    dd { margin: 0; }
    .mono { font-family: 'IBM Plex Mono', monospace; font-size: 13px; }
    .danger { display: flex; align-items: center; gap: 8px; margin-top: 28px;
          padding-top: 16px; border-top: 1px solid var(--line); }
    .danger button { min-height: 44px; padding: 0 14px; border-radius: 8px;
                    border: 1px solid #D1D5DB; background: #fff; cursor: pointer; }
    .del { background: #B91C1C !important; border-color: #B91C1C !important; color: #fff; }
    .del-ghost { color: #B91C1C; }
</style>