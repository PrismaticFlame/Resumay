<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from 'svelte';
    import type { Application, Resume } from "$lib/types";
    import { STATUS } from "$lib/status";

    import Rail from "$lib/component/Rail.svelte";
    import DetailPane from "$lib/component/DetailPane.svelte";
    import AppList from "$lib/component/AppList.svelte";

    let applications = $state<Application[]>([]);
    let resumes = $state<Resume[]>([]);

    let query = $state("");
    let filter = $state<"all" | "unused" | "used">("all");
    let previewUrl = $state("");

    let visible = $derived(resumes.filter(r =>
        r.label.toLowerCase().includes(query.toLowerCase()) &&
        (filter === "all" || (filter === "used") === (r.uses > 0))
    ));

    async function load() {
        resumes = await invoke<Resume[]>("list_resumes"); 
    }
    onMount(load);
</script>

<div class="shell">
  <Rail count={applications.length} resumeCount={resumes.length}/>
  <main>
    <header>
      <div>
        <h1>Resumes</h1>
        {#if filter !== "all" || query}
          <span class="sub">{visible.length} of {resumes.length} shown</span>
        {/if}
      </div>
      <input type="search" placeholder="Search company or role" bind:value={query} />
    </header>
  </main>
</div>

<style>
  
  .shell { display: grid; grid-template-columns: 224px 1fr; height: 100vh; }
main { display: flex; flex-direction: column; min-height: 0; min-width: 0; }
header { display: flex; align-items: center; gap: 16px; padding: 22px 28px 14px; }
header div { flex: 1; }
h1 { margin: 0; font-size: 24px; font-weight: 600; }
.sub { color: var(--muted); font-size: 13px; }
input[type="search"] { width: 300px; min-height: 44px; padding: 0 12px;
  border: 1px solid #D1D5DB; border-radius: 8px; background: var(--surface); }

</style>