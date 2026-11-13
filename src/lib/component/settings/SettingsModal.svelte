<script lang="ts">
    import { checkForUpdate } from "$lib/updater.svelte";
    import { getVersion } from "@tauri-apps/api/app";
    import { onMount } from "svelte";

    import { app } from "$lib/updater.svelte";
    import SettingRow from "./SettingRow.svelte";

    let { onclose }: { onclose: () => void } = $props();

    type Theme = 'light' | 'dark';

    let dialog = $state<HTMLDialogElement>();

    $effect(() => {
        dialog?.showModal();
    });

    let theme = $state<Theme>(
        (document.documentElement.dataset.theme as Theme) ?? 'light'
    );

    let updateChecking = $state(false);
    let updateMessage = $state("");

    function setTheme(next: Theme) {
        theme = next;
        document.documentElement.dataset.theme = next;
        localStorage.setItem('theme', next);
    }

    async function handleCheck() {
        updateMessage = '';
        try {
            updateChecking = true;
            await checkForUpdate(true);
            
        } catch (e) {
            updateMessage = String(e);
        } finally {
            updateChecking = false;
        }
    }
</script>

<dialog
    bind:this={dialog}
    aria-label="Settings"
    onclose={onclose}
    onclick={(e) => e.target === e.currentTarget && e.currentTarget.close()}
>
    <div class="panel">
        <h2>Settings</h2>
        <section>
            <h3>Appearance</h3>
            <SettingRow label="Color mode" description="Applies to the whole app">
                <div class="seg">
                    <button class:active={theme === 'light'} onclick={() => setTheme('light')}>Light</button>
                    <button class:active={theme === 'dark'} onclick={() => setTheme('dark')}>Dark</button>
                </div>
            </SettingRow>
        </section>
        <section>
            <h3>App</h3>
            <SettingRow label="Updates" description="Version {app.version}">
                <button class="action" onclick={handleCheck} disabled={updateChecking}>
                    {updateChecking ? 'Checking...' : 'Check for updates'}
                </button>
            </SettingRow>
            {#if updateMessage}
                <p class="error">{updateMessage}</p>
            {/if}
        </section>
    </div>
</dialog>

<style>     
    .panel { 
        padding: 24px;
    }
    .seg {
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
    }
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