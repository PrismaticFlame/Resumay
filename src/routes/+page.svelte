<script lang="ts">
  // import { event } from "@tauri-apps/api";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from 'svelte';
  import type { Application, Resume } from "$lib/types";
  import { STATUS } from "$lib/status";

  import Rail from "$lib/component/Rail.svelte";
  import DetailPane from "$lib/component/DetailPane.svelte";
  import AppList from "$lib/component/AppList.svelte";
  import NewApplicationModal from "$lib/component/NewApplicationModal.svelte"

  interface Application {
    id: number;
    company: string;
    position: string;
    website: string | null;
    job_description: string | null;
    username: string | null;
    status: string;
    resume: number | null;
    notes: string | null;
    refer: string | null;
    apply_date: string;
  }

  interface Resume {
    id: number;
    label: string;
    created_at: string;
    uses: number;
  }

  onMount(async () => {
    listApplications();
  });

  let filter = $state("all");
  let query = $state("");
  let selectedId = $state<number | null>(null);
  let showModal = $state(false);
  let editing = $state<Application | null>(null);

  let message = $state("");
  let applications = $state<Application[]>([]);
  let resumes = $state<Resume[]>([]);

  let visible = $derived(applications.filter(a =>
  (filter === "all" || a.status === filter) &&
  `${a.company} ${a.position}`.toLowerCase().includes(query.toLowerCase())
  ));
  let selected = $derived(visible.find(a => a.id === selectedId) ?? visible[0]);

  async function listApplications() {
    try {
      applications = await invoke<Application[]>("list_applications");
    } catch (err) {
      message = `Error: ${err}`;
    }
  }

  async function deleteApplication(id: number) {
    try {
      await invoke("delete_application", { id });
      selectedId = null;
      await listApplications();
    } catch (err) {
      message = `Error: ${err}`;
    }
  }

  async function handleSaved(id: number) {
    await listApplications();
    selectedId = id;
    showModal = false;
    editing = null;
  }

</script>

<div class="shell">
  <Rail />
  <main>
    <header>
      <div>
        <h1>Applications</h1>
        {#if filter !== "all" || query}
          <span class="sub">{visible.length} of {applications.length} shown</span>
        {/if}
      </div>
      <input type="search" placeholder="Search company or role" bind:value={query} />
      <button class="primary" onclick={() => showModal = true}>+ New application</button>
    </header>

    <div class="chips">
      {#each ["all", ...Object.keys(STATUS)] as f}
        <button class="chip" class:on={filter === f} onclick={() => filter = f}>{f}</button>
      {/each}
    </div>

    <div class="work">
      <div><AppList apps={visible} selectedId={selected?.id} onselect={(id: number) => selectedId = id} /></div>
      <div>
        {#if selected}
          {#key selected.id}
            <DetailPane app={selected} ondelete={() => deleteApplication(selected.id)} onedit={() => editing = selected}/>
          {/key}
        {/if}
      </div>
    </div>

    {#if showModal}
      <NewApplicationModal onclose={() => showModal = false} onsaved={handleSaved} />
    {/if}

    {#if editing}
      <NewApplicationModal app={editing} onclose={() => editing = null} onsaved={handleSaved} />
    {/if}
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
  .work { display: grid; grid-template-columns: 1fr 420px; gap: 20px;
    flex: 1; min-height: 0; padding: 0 28px 28px; }
  .work > div { background: var(--surface); border: 1px solid var(--line);
    border-radius: 12px; overflow: auto; }

</style>

