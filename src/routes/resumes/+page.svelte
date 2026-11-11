<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from 'svelte';
    import type { Application, Resume } from "$lib/types";
    import { USES } from "$lib/status";

    import Rail from "$lib/component/Rail.svelte";
    import ResumeDetailPane from "$lib/component/ResumeDetailPane.svelte";
    import ResumeList from "$lib/component/ResumeList.svelte"

    let resumes = $state<Resume[]>([]);

    let query = $state("");
    let filter = $state("all");
    let previewUrl = $state("");

    let rSelectedId = $state<number | null>(null);

    let message = $state("");

    let visible = $derived(resumes.filter(r =>
        r.label.toLowerCase().includes(query.toLowerCase()) &&
        (filter === "all" || (filter === "used") === (r.uses > 0))
    ));
    let rSelected = $derived(visible.find(a => a.id === rSelectedId) ?? visible[0]);

    async function load() {
        resumes = await invoke<Resume[]>("list_resumes"); 
    }
    onMount(load);

    async function showResume(id: number) {
      message = "";
      try {
        const bytes = await invoke<number[]>("fetch_resumes", { 
          id 
        });
        const blob = new Blob([new Uint8Array(bytes)], { type: "application/pdf" });
        if (previewUrl) URL.revokeObjectURL(previewUrl);
        previewUrl = URL.createObjectURL(blob);
      } catch (err) {
        message = String(err);
      }
    }

    async function deleteResume(id: number) {
      message = "";
      try {
        await invoke("delete_resume", { id });
        rSelectedId = null;
        await load();
      } catch (err) {
        message = String(err);
      }
    }
</script>

<div class="shell">
  <Rail />
  <main>
    <header>
      <div>
        <h1>Resumes</h1>
        {#if filter !== "all" || query}
          <span class="sub">{visible.length} of {resumes.length} shown</span>
        {/if}
      </div>
      <input type="search" placeholder="Search resumes" bind:value={query} />
      <!-- Button for new Resumes here -->
    </header>

    <div class="chips">
      {#each ["all", ...Object.keys(USES)] as f}
        <button class="chip" class:on={filter === f} onclick={() => filter = f}>{f}</button>
      {/each}
    </div>

    <div class="work">
      <div>
        {#if rSelected}
          {#key rSelected.id}
            <ResumeDetailPane resume={rSelected} ondelete={() => deleteResume(rSelected.id)}/>
          {/key}
        {/if}
      </div>
      <div><ResumeList resume={visible} rSelectedId={rSelected?.id} onselect={(id: number) => { rSelectedId = id; previewUrl = ""; }} /></div>
    </div>
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
  .primary { min-height: 44px; padding: 0 18px; border: none; border-radius: 8px;
    background: var(--accent); color: #fff; font-weight: 600; cursor: pointer; }

  .chips { display: flex; gap: 8px; padding: 0 28px 16px; }
  .chip { min-height: 40px; padding: 0 14px; border-radius: 999px; cursor: pointer;
    border: 1px solid #D1D5DB; background: var(--surface); text-transform: capitalize; }
  .chip.on { background: var(--ink); border-color: var(--ink); color: #fff; }

  .work { display: grid; grid-template-columns: 1fr 420px; gap: 20px;
    flex: 1; min-height: 0; padding: 0 28px 28px; }
  .work > div { background: var(--surface); border: 1px solid var(--line);
    border-radius: 12px; overflow: auto; }
    
</style>