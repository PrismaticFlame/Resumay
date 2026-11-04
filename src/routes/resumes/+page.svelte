<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from 'svelte';
    import type { Application, Resume } from "$lib/types";
    import { STATUS } from "$lib/status";

    import Rail from "$lib/component/Rail.svelte";
    import DetailPane from "$lib/component/DetailPane.svelte";
    import ResumeList from "$lib/component/ResumeList.svelte"

    let applications = $state<Application[]>([]);
    let resumes = $state<Resume[]>([]);

    let query = $state("");
    let filter = $state<"all" | "unused" | "used">("all");
    let previewUrl = $state("");

    let rSelectedId = $state<number | null>(null);
    let showModal = $state(false);
    let editing = $state<Application | null>(null);

    let name = $state("");

    let message = $state("");

    let latexSource = $state("");
    let compileError = $state("");
    let pdfBlobUrl = $state("");

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
      <input type="search" placeholder="Search resumes" bind:value={query} />
    </header>

    <div class="work">
      <div><ResumeList resume={visible} rSelectedId={rSelected?.id} onselect={(id: number) => { rSelectedId = id; previewUrl = ""; }} /></div>
      <div>
        {#if rSelected}
          {#key rSelected.id}
            <h2>{rSelected.label}</h2>
            <button onclick={() => showResume(rSelected.id)}>View PDF</button>
            {#if previewUrl}
              <iframe title="Resume preview" src={previewUrl}></iframe>
            {/if}
          {/key}
        {/if}
      </div>
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
</style>