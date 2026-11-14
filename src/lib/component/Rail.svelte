<script lang="ts">
    import logo from "$lib/assets/resumay-logo.svg"
    import { page } from "$app/state";
    
    import { counts } from "$lib/counts.svelte";
    import { app } from "$lib/updater.svelte";

    import SettingsModal from "./settings/SettingsModal.svelte";
    let showSettings = $state(false);
</script>

<nav>
    <div class="brand">
        <img src={logo} alt="Resumay" width="190px" height="50px">
    </div>
    <a href="/" class="item" class:on={page.url.pathname === "/"}>
        <span>Applications</span>
        <span class="count">{counts.applications}</span>
    </a>
    <a href="/resumes" class="item" class:on={page.url.pathname === "/resumes"}>
        <span>Resumes</span>
        <span class="count">{counts.resumes}</span>
    </a>
    <a href="/edit" class="item" class:on={page.url.pathname === "/edit"}>
        <span>Edit</span>
    </a>
    <button class="item">Tailor</button>
    <button class ="item settings" onclick={() => (showSettings = true)}>Settings</button>

    {#if showSettings}
        <SettingsModal onclose={() => (showSettings = false)} />
    {/if}
    {#if app.version}
        <span class="version">v{app.version}</span>
    {/if}
</nav>

<style>
    nav { display: flex; flex-direction: column; gap: 4px; padding: 22px 14px;
            background: var(--surface); border-right: 1px solid var(--line); }
    .brand { font-size: 19px; font-weight: 700; padding: 0 8px 24px; }
    .item { text-align: left; min-height: 44px; padding: 0 12px; border: none; text-decoration: none;
            border-radius: 8px; background: transparent; color: var(--ink); cursor: pointer; }
    .item.on { background: var(--accent-weak); color: var(--accent); font-weight: 600; }
    .item { display: flex; align-items: center; justify-content: space-between; }
    .count { font-family: 'IBM Plex Mono', monospace; font-size: 12px; }
    .settings { margin-top: auto; }
    .version {
        margin-top: 0;
        font-family: 'IBM Plex Mono', monospace;
        font-size: 11px;
        color: var(--muted);
        
    }
</style>
