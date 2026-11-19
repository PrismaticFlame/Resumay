<script lang="ts">
    import { updater, app, installUpdate } from "$lib/updater.svelte";

    // let { onclose }: { onclose: () => void } = $props();

    let dialog = $state<HTMLDialogElement>();

    $effect(() => {
        dialog?.showModal();
    });
    
</script>

<dialog
    bind:this={dialog}
    aria-label="Settings"
    // onclose={onclose}
    onclick={(e) => e.target === e.currentTarget && e.currentTarget.close()}
>
    <div class="panel">
        <h2>Update available</h2>
        <p class="muted">Resumay {updater.available?.version} is available (you have {app.version}).</p>

        <!-- {#if updater.available?.notes}
            <pre class="notes">{updater.available.notes}</pre>
        {/if} -->

        {#if updater.status === 'downloading'}
            <progress value={updater.progress}></progress>
        {:else}
            <div class="actions">
                <button onclick={() => (updater.dismissed = true)}>Later</button>
                <button class="action" onclick={installUpdate}>Install & Restart</button>
            </div>
        {/if}
    </div>
</dialog>

<style>     
    .panel { 
        padding: 24px;
    }
    /* .seg {
        display: flex;
        gap: 4px;
        padding: 3px;
        background: var(--bg);
        border-radius: 8px;
    }
    .seg button { 
        border: none;
        background: transparent;
        color: var(--muted);
        padding: 6px 14px;
        border-radius: 6px;
        cursor: pointer;
        font: inherit;
    }
    .seg button.active {
        background: var(--surface);
        color: var(--ink);
        font-weight: 600;
    } */
    .action{
        border: 3px solid var(--bg);
        border-radius: 8px;
        background: var(--surface);
        color: var(--ink);
        padding: 6px 14px;
        font: inherit;
        font-weight: 600;
        cursor: pointer;
    }
    .action:disabled {
        color: var(--muted);
        cursor: default;
    }
    dialog { 
        padding: 0;
        width: min(420px, 90vw);
        background: var(--surface);
        color: var(--ink);
        border: 1px solid var(--line);
        border-radius: 12px;
        box-shadow: 0 20px 50px rgba(0, 0, 0, .25);
        overflow: hidden;
    }
    dialog::backdrop {
        background: rgba(0, 0, 0, .4);
        backdrop-filter: blur(6px);
    }
</style>