<script lang="ts">
  // import { event } from "@tauri-apps/api";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from 'svelte';
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
    username: string | null;
    status: string;
    resume: number | null;
    notes: string | null;
    refer: string | null;
    apply_date: string;
  }

  onMount(() => {
    listApplications()
  });

  let filter = $state("all");
  let query = $state("");
  let selectedId = $state<number | null>(null);
  let showModal = $state(false);
  // let visible = $derived(applications.filter(a =>
  // (filter === "all" || a.status === filter) &&
  // `${a.company} ${a.position}`.toLowerCase().includes(query.toLowerCase())
  // ));

  let name = $state("");
  let greetMsg = $state("");

  let message = $state("");
  let applications = $state<Application[]>([]);

  let visible = $derived(applications.filter(a =>
  (filter === "all" || a.status === filter) &&
  `${a.company} ${a.position}`.toLowerCase().includes(query.toLowerCase())
  ));
  let selected = $derived(visible.find(a => a.id === selectedId) ?? visible[0]);

  let latexSource = $state("");
  let compileError = $state("");
  let pdfBlobUrl = $state("");

  async function greet(event: any) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }

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

  async function compileLatex() {
    compileError = "";
    try {
      const bytes = await invoke<number[]>("compile_resume", {
        tex: latexSource
      });
      const blob = new Blob([new Uint8Array(bytes)], { type: "application/pdf" });
      if (pdfBlobUrl) URL.revokeObjectURL(pdfBlobUrl);
      pdfBlobUrl = URL.createObjectURL(blob);
    } catch (err) {
      compileError = String(err);
    }
  }

  async function handleSaved(id: number) {
    await listApplications();
    selectedId = id;
    showModal = false;
  }

</script>

<!-- <main class="container">
  <h1>Resumay</h1>

  <form id="appliform" class="greet" onsubmit={addApplication}>
    <input id="company" placeholder="Company name..." bind:value={company} />
    <input id="position" placeholder="Position..." bind:value={position} />
    <input id="website" placeholder="Website URL..." bind:value={website} />
    <input id="applyDate" type="date" bind:value={applyDate} />
    <input id="status" placeholder="Status of Application" bind:value={status} />
    <textarea id="notes" placeholder="Notes..." bind:value={notes}></textarea>
    <input id="refer" placeholder="Referral..." bind:value={refer} />
    <input id="username" placeholder="Username..." bind:value={username} />
    <input id="password" placeholder="Password..." type="password" bind:value={password} />
    <button type="submit">Add Application</button>
  </form>
  <p>{message}</p>

  <AppList apps={visible} selectedId={selectedId?.id} onselect={(id) => selectedId = id} />

  <ul>
    {#each applications as appli (appli.id)}
    <li>{appli.company}, {appli.position}, {appli.apply_date}, {appli.status}</li>
    {/each}
  </ul>

  <textarea
    bind:value={latexSource}
    rows = "12"
    placeholder="LaTeX Source here..."
  ></textarea>

  <button onclick={compileLatex}>Compile latex</button>

  {#if compileError}
    <pre class="error">{compileError}</pre>
  {/if}
  {#if pdfBlobUrl}
    <iframe title="pdf" src={pdfBlobUrl} width="100%" height="600"></iframe>
  {/if}
  </main> -->

<div class="shell">
  <Rail count={applications.length}/>
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
            <DetailPane app={selected} ondelete={() => deleteApplication(selected.id)} />
          {/key}
        {/if}
      </div>
    </div>

    {#if showModal}
      <NewApplicationModal onclose={() => showModal = false} onsaved={handleSaved} />
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

.chips { display: flex; gap: 8px; padding: 0 28px 16px; }
.chip { min-height: 40px; padding: 0 14px; border-radius: 999px; cursor: pointer;
  border: 1px solid #D1D5DB; background: var(--surface); text-transform: capitalize; }
.chip.on { background: var(--ink); border-color: var(--ink); color: #fff; }

.work { display: grid; grid-template-columns: 1fr 420px; gap: 20px;
  flex: 1; min-height: 0; padding: 0 28px 28px; }
.work > div { background: var(--surface); border: 1px solid var(--line);
  border-radius: 12px; overflow: auto; }

</style>

