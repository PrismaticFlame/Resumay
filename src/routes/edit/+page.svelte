<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from 'svelte';
    
    import Rail from "$lib/component/Rail.svelte";
</script>

<div class="shell">
    <Rail />
    <main>
        <header>
            <div>
                <h1>Editor</h1>
            </div>
        </header>
    </main>
</div>

<style>
    .shell { display: grid; grid-template-columns: 224px 1fr; height: 100vh; }
    main { display: flex; flex-direction: column; min-height: 0; min-width: 0; }
    header { display: flex; align-items: center; gap: 16px; padding: 22px 28px 14px; }
    header div { flex: 1; }
    h1 { margin: 0; font-size: 24px; font-weight: 600; }
</style>